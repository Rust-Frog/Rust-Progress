pub mod command;
pub mod insert;
pub mod normal;
pub mod visual;

pub use command::handle_command_mode;
pub use insert::handle_insert_mode;
pub use normal::handle_normal_mode;
pub use visual::handle_visual_mode;
