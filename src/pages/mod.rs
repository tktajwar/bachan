pub mod template;

pub mod helper;
pub mod forms;

pub mod root_handler;
pub use root_handler::*;

pub mod k_handlers;
pub use k_handlers::*;

pub mod board_handlers;
pub use board_handlers::*;

pub mod fallback_handler;
pub use fallback_handler::*;
