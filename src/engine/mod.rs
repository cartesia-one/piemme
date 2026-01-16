//! Prompt engine - reference resolution and command execution

mod commands;
mod references;
mod resolver;

pub use commands::execute_command;
pub use references::{find_references, find_file_references, validate_reference, validate_file_reference, Reference, FileReference, has_file_references};
pub use resolver::{resolve_commands_in_content, resolve_prompt, resolve_prompt_with_base, ResolveOptions};
