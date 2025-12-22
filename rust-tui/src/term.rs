use crossterm::{
    Command, QueueableCommand,
    cursor::MoveTo,
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::{
    fmt, fs,
    io::{self, BufRead, StdoutLock, Write},
};

use crate::app_state::CheckProgress;

pub trait CountedWrite<'lock> {
    fn write_str(&mut self, unicode: &str) -> io::Result<()>;
    fn stdout(&mut self) -> &mut StdoutLock<'lock>;
}

impl<'a> CountedWrite<'a> for StdoutLock<'a> {
    #[inline]
    fn write_str(&mut self, unicode: &str) -> io::Result<()> {
        self.write_all(unicode.as_bytes())
    }

    #[inline]
    fn stdout(&mut self) -> &mut StdoutLock<'a> {
        self
    }
}

pub struct CheckProgressVisualizer<'a, 'lock> {
    stdout: &'a mut StdoutLock<'lock>,
    n_cols: usize,
}

impl<'a, 'lock> CheckProgressVisualizer<'a, 'lock> {
    const CHECKING_COLOR: Color = Color::Blue;
    const DONE_COLOR: Color = Color::Green;
    const PENDING_COLOR: Color = Color::Red;

    pub fn build(stdout: &'a mut StdoutLock<'lock>, term_width: u16) -> io::Result<Self> {
        clear_terminal(stdout)?;
        stdout.write_all("Checking all exercises…\n".as_bytes())?;

        // Legend
        stdout.write_all(b"Color of exercise number: ")?;
        stdout.queue(SetForegroundColor(Self::CHECKING_COLOR))?;
        stdout.write_all(b"Checking")?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b" - ")?;
        stdout.queue(SetForegroundColor(Self::DONE_COLOR))?;
        stdout.write_all(b"Done")?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b" - ")?;
        stdout.queue(SetForegroundColor(Self::PENDING_COLOR))?;
        stdout.write_all(b"Pending")?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b"\n")?;

        // Exercise numbers with up to 3 digits.
        // +1 because the last column doesn't end with a whitespace.
        let n_cols = usize::from(term_width + 1) / 4;

        Ok(Self { stdout, n_cols })
    }

    pub fn update(&mut self, progresses: &[CheckProgress]) -> io::Result<()> {
        self.stdout.queue(MoveTo(0, 2))?;

        let mut exercise_num = 1;
        for exercise_progress in progresses {
            match exercise_progress {
                CheckProgress::None => (),
                CheckProgress::Checking => {
                    self.stdout
                        .queue(SetForegroundColor(Self::CHECKING_COLOR))?;
                }
                CheckProgress::Done => {
                    self.stdout.queue(SetForegroundColor(Self::DONE_COLOR))?;
                }
                CheckProgress::Pending => {
                    self.stdout.queue(SetForegroundColor(Self::PENDING_COLOR))?;
                }
            }

            write!(self.stdout, "{exercise_num:<3}")?;
            self.stdout.queue(ResetColor)?;

            if exercise_num != progresses.len() {
                if exercise_num % self.n_cols == 0 {
                    self.stdout.write_all(b"\n")?;
                } else {
                    self.stdout.write_all(b" ")?;
                }

                exercise_num += 1;
            }
        }

        self.stdout.flush()
    }
}

pub struct ProgressCounter<'a, 'lock> {
    stdout: &'a mut StdoutLock<'lock>,
    total: usize,
    counter: usize,
}

impl<'a, 'lock> ProgressCounter<'a, 'lock> {
    pub fn new(stdout: &'a mut StdoutLock<'lock>, total: usize) -> io::Result<Self> {
        write!(stdout, "Progress: 0/{total}")?;
        stdout.flush()?;

        Ok(Self {
            stdout,
            total,
            counter: 0,
        })
    }

    pub fn increment(&mut self) -> io::Result<()> {
        self.counter += 1;
        write!(self.stdout, "\rProgress: {}/{}", self.counter, self.total)?;
        self.stdout.flush()
    }
}

impl Drop for ProgressCounter<'_, '_> {
    fn drop(&mut self) {
        let _ = self.stdout.write_all(b"\n\n");
    }
}

pub fn clear_terminal(stdout: &mut StdoutLock) -> io::Result<()> {
    stdout
        .queue(MoveTo(0, 0))?
        .queue(Clear(ClearType::All))?
        .queue(Clear(ClearType::Purge))
        .map(|_| ())
}

pub fn press_enter_prompt(stdout: &mut StdoutLock) -> io::Result<()> {
    stdout.flush()?;
    io::stdin().lock().read_until(b'\n', &mut Vec::new())?;
    stdout.write_all(b"\n")
}

/// Canonicalize, convert to string and remove verbatim part on Windows.
pub fn canonicalize(path: &str) -> Option<String> {
    fs::canonicalize(path)
        .ok()?
        .into_os_string()
        .into_string()
        .ok()
        .map(|mut path| {
            // Windows itself can't handle its verbatim paths.
            if cfg!(windows) && path.as_bytes().starts_with(br"\\?\") {
                path.drain(..4);
            }

            path
        })
}

pub fn file_path<'a, W: CountedWrite<'a>>(
    writer: &mut W,
    color: Color,
    f: impl FnOnce(&mut W) -> io::Result<()>,
) -> io::Result<()> {
    writer
        .stdout()
        .queue(SetForegroundColor(color))?
        .queue(SetAttribute(Attribute::Underlined))?;

    f(writer)?;

    writer
        .stdout()
        .queue(SetForegroundColor(Color::Reset))?
        .queue(SetAttribute(Attribute::NoUnderline))?;

    Ok(())
}

pub fn terminal_file_link<'a>(
    writer: &mut impl CountedWrite<'a>,
    path: &str,
    canonical_path: &str,
) -> io::Result<()> {
    writer.stdout().write_all(b"\x1b]8;;file://")?;
    writer.stdout().write_all(canonical_path.as_bytes())?;
    writer.stdout().write_all(b"\x1b\\")?;
    // Only this part is visible.
    writer.write_str(path)?;
    writer.stdout().write_all(b"\x1b]8;;\x1b\\")
}

pub fn write_ansi(output: &mut Vec<u8>, command: impl Command) {
    struct FmtWriter<'a>(&'a mut Vec<u8>);

    impl fmt::Write for FmtWriter<'_> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.extend_from_slice(s.as_bytes());
            Ok(())
        }
    }

    let _ = command.write_ansi(&mut FmtWriter(output));
}
