//! Prompt engine - reference resolution and command execution

mod commands;
mod references;
mod resolver;

pub use commands::execute_command;
pub use references::{
    find_file_references, find_references, has_file_references, read_file_content,
    validate_file_reference, validate_reference, FileReference, Reference,
};
pub use resolver::{resolve_commands_in_content, resolve_prompt, ResolveOptions};
