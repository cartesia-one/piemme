//! Prompt engine - reference resolution and command execution

mod commands;
mod references;
mod resolver;

pub use commands::execute_command;
pub use references::{find_references, validate_reference, Reference};
pub use resolver::{resolve_commands_in_content, resolve_prompt, ResolveOptions};
