use anyhow::{Context, Error, Result, bail};
use crossterm::{QueueableCommand, cursor, terminal};
use std::{
    collections::HashSet,
    env,
    fs::{File, OpenOptions},
    io::{Read, Seek, StdoutLock, Write},
    path::{MAIN_SEPARATOR_STR, Path},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicUsize, Ordering::Relaxed},
        mpsc,
    },
    thread,
};

use crate::{
    clear_terminal,
    cmd::CmdRunner,
    embedded::EMBEDDED_FILES,
    exercise::{Exercise, RunnableExercise},
    info_file::ExerciseInfo,
    term::{self, CheckProgressVisualizer},
};

const STATE_FILE_NAME: &str = ".rustlings-state.txt";
const DEFAULT_CHECK_PARALLELISM: usize = 8;

#[must_use]
pub enum ExercisesProgress {
    // All exercises are done.
    AllDone,
    // A new exercise is now pending.
    NewPending,
}

pub enum StateFileStatus {
    Read,
    NotRead,
}

#[derive(Clone, Copy)]
pub enum CheckProgress {
    None,
    Checking,
    Done,
    Pending,
}

pub struct AppState {
    current_exercise_ind: usize,
    exercises: Vec<Exercise>,
    // Caches the number of done exercises to avoid iterating over all exercises every time.
    n_done: u16,
    final_message: String,
    state_file: File,
    // Preallocated buffer for reading and writing the state file.
    file_buf: Vec<u8>,
    official_exercises: bool,
    cmd_runner: CmdRunner,
    emit_file_links: bool,
}

// Build the canonical path for an exercise (extracted from new() for clarity)
fn build_canonical_path(
    dir_canonical_path: Option<&str>,
    dir: Option<&str>,
    name: &str,
) -> Option<String> {
    dir_canonical_path.map(|base| {
        let mut path = if let Some(d) = dir {
            let mut p = String::with_capacity(2 + base.len() + d.len() + name.len());
            p.push_str(base);
            p.push_str(MAIN_SEPARATOR_STR);
            p.push_str(d);
            p
        } else {
            let mut p = String::with_capacity(1 + base.len() + name.len());
            p.push_str(base);
            p
        };
        path.push_str(MAIN_SEPARATOR_STR);
        path.push_str(name);
        path.push_str(".rs");
        path
    })
}

// Parse the state file and update exercises' done status
fn parse_state_file(
    state_file: &mut File,
    file_buf: &mut Vec<u8>,
    exercises: &mut [Exercise],
) -> (usize, u16, StateFileStatus) {
    let mut current_exercise_ind = 0;
    let mut n_done: u16 = 0;

    if state_file.read_to_end(file_buf).is_err() {
        return (0, 0, StateFileStatus::NotRead);
    }

    let mut lines = file_buf.split(|c| *c == b'\n').skip(2);

    let Some(current_exercise_name) = lines.next() else {
        return (0, 0, StateFileStatus::NotRead);
    };

    if current_exercise_name.is_empty() || lines.next().is_none() {
        return (0, 0, StateFileStatus::NotRead);
    }

    let done_exercises: HashSet<_> = lines.take_while(|name| !name.is_empty()).collect();

    for (ind, exercise) in exercises.iter_mut().enumerate() {
        if done_exercises.contains(exercise.name.as_bytes()) {
            exercise.done = true;
            n_done += 1;
        }
        if exercise.name.as_bytes() == current_exercise_name {
            current_exercise_ind = ind;
        }
    }

    (current_exercise_ind, n_done, StateFileStatus::Read)
}

impl AppState {
    pub fn new(
        exercise_infos: Vec<ExerciseInfo>,
        final_message: String,
    ) -> Result<(Self, StateFileStatus)> {
        let cmd_runner = CmdRunner::build()?;
        let mut state_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(STATE_FILE_NAME)
            .with_context(|| {
                format!("Failed to open or create the state file {STATE_FILE_NAME}")
            })?;

        let dir_canonical_path = term::canonicalize("exercises");
        let mut exercises = exercise_infos
            .into_iter()
            .map(|exercise_info| {
                // Leaking to be able to borrow in the watch mode `Table`.
                let path = exercise_info.path().leak();
                let name = exercise_info.name.leak();
                let dir = exercise_info.dir.map(|dir| &*dir.leak());
                let hint = exercise_info.hint.leak().trim_ascii();
                let canonical_path = build_canonical_path(dir_canonical_path.as_deref(), dir, name);

                Exercise {
                    dir,
                    name,
                    path,
                    canonical_path,
                    test: exercise_info.test,
                    strict_clippy: exercise_info.strict_clippy,
                    hint,
                    done: false,
                }
            })
            .collect::<Vec<_>>();

        let mut file_buf = Vec::with_capacity(2048);
        let (current_exercise_ind, n_done, state_file_status) =
            parse_state_file(&mut state_file, &mut file_buf, &mut exercises);

        file_buf.clear();
        file_buf.extend_from_slice(STATE_FILE_HEADER);

        let slf = Self {
            current_exercise_ind,
            exercises,
            n_done,
            final_message,
            state_file,
            file_buf,
            official_exercises: !Path::new("info.toml").exists(),
            cmd_runner,
            // VS Code has its own file link handling
            emit_file_links: env::var_os("TERM_PROGRAM").is_none_or(|v| v != "vscode"),
        };

        Ok((slf, state_file_status))
    }

    #[inline]
    pub fn current_exercise_ind(&self) -> usize {
        self.current_exercise_ind
    }

    #[inline]
    pub fn exercises(&self) -> &[Exercise] {
        &self.exercises
    }

    #[inline]
    pub fn n_done(&self) -> u16 {
        self.n_done
    }

    #[inline]
    pub fn n_pending(&self) -> u16 {
        self.exercises.len() as u16 - self.n_done
    }

    #[inline]
    pub fn current_exercise(&self) -> &Exercise {
        &self.exercises[self.current_exercise_ind]
    }

    #[inline]
    pub fn cmd_runner(&self) -> &CmdRunner {
        &self.cmd_runner
    }

    #[inline]
    pub fn emit_file_links(&self) -> bool {
        self.emit_file_links
    }

    pub fn write(&mut self) -> Result<()> {
        self.file_buf.truncate(STATE_FILE_HEADER.len());

        self.file_buf
            .extend_from_slice(self.current_exercise().name.as_bytes());
        self.file_buf.push(b'\n');

        for exercise in &self.exercises {
            if exercise.done {
                self.file_buf.push(b'\n');
                self.file_buf.extend_from_slice(exercise.name.as_bytes());
            }
        }

        self.state_file
            .rewind()
            .with_context(|| format!("Failed to rewind the state file {STATE_FILE_NAME}"))?;
        self.state_file
            .set_len(0)
            .with_context(|| format!("Failed to truncate the state file {STATE_FILE_NAME}"))?;
        self.state_file
            .write_all(&self.file_buf)
            .with_context(|| format!("Failed to write the state file {STATE_FILE_NAME}"))?;

        Ok(())
    }

    pub fn set_current_exercise_ind(&mut self, exercise_ind: usize) -> Result<()> {
        if exercise_ind == self.current_exercise_ind {
            return Ok(());
        }

        if exercise_ind >= self.exercises.len() {
            bail!(BAD_INDEX_ERR);
        }

        self.current_exercise_ind = exercise_ind;

        self.write()
    }

    pub fn set_current_exercise_by_name(&mut self, name: &str) -> Result<()> {
        // O(N) is fine since this method is used only once until the program exits.
        // Building a hashmap would have more overhead.
        self.current_exercise_ind = self
            .exercises
            .iter()
            .position(|exercise| exercise.name == name)
            .with_context(|| format!("No exercise found for '{name}'!"))?;

        self.write()
    }

    // Set the status of an exercise without saving. Returns `true` if the
    // status actually changed (and thus needs saving later).
    pub fn set_status(&mut self, exercise_ind: usize, done: bool) -> Result<bool> {
        let exercise = self
            .exercises
            .get_mut(exercise_ind)
            .context(BAD_INDEX_ERR)?;

        if exercise.done == done {
            return Ok(false);
        }

        exercise.done = done;
        if done {
            self.n_done += 1;
        } else {
            self.n_done -= 1;
        }

        Ok(true)
    }

    // Set the status of an exercise to "pending" and save.
    pub fn set_pending(&mut self, exercise_ind: usize) -> Result<()> {
        if self.set_status(exercise_ind, false)? {
            self.write()?;
        }

        Ok(())
    }

    // Official exercises: Dump the original file from the binary.
    // Community exercises: Reset the exercise file with `git stash`.
    fn reset(&self, exercise_ind: usize, path: &str) -> Result<()> {
        if self.official_exercises {
            return EMBEDDED_FILES
                .write_exercise_to_disk(exercise_ind, path)
                .with_context(|| format!("Failed to reset the exercise {path}"));
        }

        let output = Command::new("git")
            .arg("stash")
            .arg("push")
            .arg("--")
            .arg(path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .output()
            .with_context(|| format!("Failed to run `git stash push -- {path}`"))?;

        if !output.status.success() {
            bail!(
                "`git stash push -- {path}` didn't run successfully: {}",
                String::from_utf8_lossy(&output.stderr),
            );
        }

        Ok(())
    }

    pub fn reset_current_exercise(&mut self) -> Result<&'static str> {
        self.set_pending(self.current_exercise_ind)?;
        let exercise = self.current_exercise();
        self.reset(self.current_exercise_ind, exercise.path)?;

        Ok(exercise.path)
    }

    // Return the index of the next pending exercise or `None` if all exercises are done.
    fn next_pending_exercise_ind(&self) -> Option<usize> {
        let next_ind = self.current_exercise_ind + 1;
        self.exercises
            // If the exercise done isn't the last, search for pending exercises after it.
            .get(next_ind..)
            .and_then(|later_exercises| {
                later_exercises
                    .iter()
                    .position(|exercise| !exercise.done)
                    .map(|ind| next_ind + ind)
            })
            // Search from the start.
            .or_else(|| {
                self.exercises[..self.current_exercise_ind]
                    .iter()
                    .position(|exercise| !exercise.done)
            })
    }

    /// Get the solution path for the current exercise.
    /// Official exercises: Write embedded solution to disk (overwrites placeholder).
    /// Community exercises: Check if on-disk solution exists.
    pub fn current_solution_path(&self) -> Result<Option<String>> {
        let current_exercise = self.current_exercise();

        if self.official_exercises {
            // For official exercises, always write the embedded solution to disk
            // This overwrites the placeholder files with the real solutions
            EMBEDDED_FILES
                .write_solution_to_disk(self.current_exercise_ind, current_exercise.name)
                .map(Some)
        } else {
            // For community exercises, check if on-disk solution exists
            let sol_path = current_exercise.sol_path();
            if Path::new(&sol_path).exists() {
                return Ok(Some(sol_path));
            }
            Ok(None)
        }
    }

    fn check_all_exercises_impl(&mut self, stdout: &mut StdoutLock) -> Result<Option<usize>> {
        let term_width = terminal::size()
            .context("Failed to get the terminal size")?
            .0;
        let mut progress_visualizer = CheckProgressVisualizer::build(stdout, term_width)?;

        let next_exercise_ind = AtomicUsize::new(0);
        let mut progresses = vec![CheckProgress::None; self.exercises.len()];

        thread::scope(|s| {
            let (exercise_progress_sender, exercise_progress_receiver) = mpsc::channel();
            let n_threads = thread::available_parallelism()
                .map_or(DEFAULT_CHECK_PARALLELISM, |count| count.get());

            for _ in 0..n_threads {
                let exercise_progress_sender = exercise_progress_sender.clone();
                let next_exercise_ind = &next_exercise_ind;
                let slf = &self;
                thread::Builder::new()
                    .spawn_scoped(s, move || {
                        loop {
                            let exercise_ind = next_exercise_ind.fetch_add(1, Relaxed);
                            let Some(exercise) = slf.exercises.get(exercise_ind) else {
                                // No more exercises.
                                break;
                            };

                            if exercise_progress_sender
                                .send((exercise_ind, CheckProgress::Checking))
                                .is_err()
                            {
                                break;
                            };

                            let success = exercise.run_exercise(None, &slf.cmd_runner);
                            let progress = match success {
                                Ok(true) => CheckProgress::Done,
                                Ok(false) => CheckProgress::Pending,
                                Err(_) => CheckProgress::None,
                            };

                            if exercise_progress_sender
                                .send((exercise_ind, progress))
                                .is_err()
                            {
                                break;
                            }
                        }
                    })
                    .context("Failed to spawn a thread to check all exercises")?;
            }

            // Drop this sender to detect when the last thread is done.
            drop(exercise_progress_sender);

            while let Ok((exercise_ind, progress)) = exercise_progress_receiver.recv() {
                progresses[exercise_ind] = progress;
                progress_visualizer.update(&progresses)?;
            }

            Ok::<_, Error>(())
        })?;

        let first_pending =
            self.process_exercise_results(&mut progresses, &mut progress_visualizer)?;

        self.write()?;
        Ok(first_pending)
    }

    // Process results from parallel exercise checking and handle fallbacks
    fn process_exercise_results(
        &mut self,
        progresses: &mut [CheckProgress],
        visualizer: &mut CheckProgressVisualizer<'_, '_>,
    ) -> Result<Option<usize>> {
        let mut first_pending = None;

        for exercise_ind in 0..progresses.len() {
            match progresses[exercise_ind] {
                CheckProgress::Done => {
                    self.set_status(exercise_ind, true)?;
                }
                CheckProgress::Pending => {
                    self.set_status(exercise_ind, false)?;
                    if first_pending.is_none() {
                        first_pending = Some(exercise_ind);
                    }
                }
                CheckProgress::None | CheckProgress::Checking => {
                    // Retry failed exercises sequentially (may have hit fd limit)
                    progresses[exercise_ind] = CheckProgress::Checking;
                    visualizer.update(progresses)?;

                    let success =
                        self.exercises[exercise_ind].run_exercise(None, &self.cmd_runner)?;

                    progresses[exercise_ind] = if success {
                        CheckProgress::Done
                    } else {
                        if first_pending.is_none() {
                            first_pending = Some(exercise_ind);
                        }
                        CheckProgress::Pending
                    };

                    self.set_status(exercise_ind, success)?;
                    visualizer.update(progresses)?;
                }
            }
        }

        Ok(first_pending)
    }

    // Return the exercise index of the first pending exercise found.
    pub fn check_all_exercises(&mut self, stdout: &mut StdoutLock) -> Result<Option<usize>> {
        stdout.queue(cursor::Hide)?;
        let res = self.check_all_exercises_impl(stdout);
        stdout.queue(cursor::Show)?;

        res
    }

    /// Mark the current exercise as done and move on to the next pending exercise if one exists.
    /// If all exercises are marked as done, run all of them to make sure that they are actually
    /// done. If an exercise which is marked as done fails, mark it as pending and continue on it.
    pub fn done_current_exercise<const CLEAR_BEFORE_FINAL_CHECK: bool>(
        &mut self,
        stdout: &mut StdoutLock,
    ) -> Result<ExercisesProgress> {
        let exercise = &mut self.exercises[self.current_exercise_ind];
        if !exercise.done {
            exercise.done = true;
            self.n_done += 1;
        }

        if let Some(ind) = self.next_pending_exercise_ind() {
            self.set_current_exercise_ind(ind)?;
            return Ok(ExercisesProgress::NewPending);
        }

        if CLEAR_BEFORE_FINAL_CHECK {
            clear_terminal(stdout)?;
        } else {
            stdout.write_all(b"\n")?;
        }

        if let Some(first_pending_exercise_ind) = self.check_all_exercises(stdout)? {
            self.set_current_exercise_ind(first_pending_exercise_ind)?;

            return Ok(ExercisesProgress::NewPending);
        }

        self.render_final_message(stdout)?;

        Ok(ExercisesProgress::AllDone)
    }

    pub fn render_final_message(&self, stdout: &mut StdoutLock) -> Result<()> {
        clear_terminal(stdout)?;
        stdout.write_all(FENISH_LINE.as_bytes())?;

        let final_message = self.final_message.trim_ascii();
        if !final_message.is_empty() {
            stdout.write_all(final_message.as_bytes())?;
            stdout.write_all(b"\n")?;
        }

        Ok(())
    }
}

const BAD_INDEX_ERR: &str = "The current exercise index is higher than the number of exercises";
const STATE_FILE_HEADER: &[u8] = b"DON'T EDIT THIS FILE!\n\n";
const FENISH_LINE: &str = "+----------------------------------------------------+
|          You made it to the Fe-nish line!          |
+--------------------------  ------------------------+
                           \\/\x1b[31m
     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
 ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒
           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒
       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒
           ▒▒  ▒▒                      ▒▒  ▒▒\x1b[0m

";

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_exercise() -> Exercise {
        Exercise {
            dir: None,
            name: "0",
            path: "exercises/0.rs",
            canonical_path: None,
            test: false,
            strict_clippy: false,
            hint: "",
            done: false,
        }
    }

    #[test]
    fn next_pending_exercise() {
        let mut app_state = AppState {
            current_exercise_ind: 0,
            exercises: vec![dummy_exercise(), dummy_exercise(), dummy_exercise()],
            n_done: 0,
            final_message: String::new(),
            state_file: tempfile::tempfile().unwrap(),
            file_buf: Vec::new(),
            official_exercises: true,
            cmd_runner: CmdRunner::build().unwrap(),
            emit_file_links: true,
        };

        let mut assert = |done: [bool; 3], expected: [Option<usize>; 3]| {
            for (exercise, done) in app_state.exercises.iter_mut().zip(done) {
                exercise.done = done;
            }
            for (ind, expected) in expected.into_iter().enumerate() {
                app_state.current_exercise_ind = ind;
                assert_eq!(
                    app_state.next_pending_exercise_ind(),
                    expected,
                    "done={done:?}, ind={ind}",
                );
            }
        };

        assert([true, true, true], [None, None, None]);
        assert([false, false, false], [Some(1), Some(2), Some(0)]);
        assert([false, true, true], [None, Some(0), Some(0)]);
        assert([true, false, true], [Some(1), None, Some(1)]);
        assert([true, true, false], [Some(2), Some(2), None]);
        assert([true, false, false], [Some(1), Some(2), Some(1)]);
        assert([false, true, false], [Some(2), Some(2), Some(0)]);
        assert([false, false, true], [Some(1), Some(0), Some(0)]);
    }
}
