//! File system operations

mod directory;
mod prompt_io;
mod index;

pub use directory::{ensure_directories, list_folders, create_folder};
pub use prompt_io::{load_prompt, save_prompt, load_all_prompts, load_all_prompts_everywhere, delete_prompt, move_prompt, rename_prompt, get_all_prompt_names, create_new_prompt};
pub use index::{Index, IndexEntry};
