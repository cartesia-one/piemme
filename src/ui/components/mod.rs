//! UI Components

mod editor;
mod help;
mod popup;
mod prompt_list;
mod status_bar;
mod title_bar;

pub use editor::Editor;
pub use help::{render_help_overlay, get_help_max_scroll};
pub use popup::{centered_rect, render_confirm_dialog, render_popup_frame, PopupConfig};
pub use prompt_list::render_prompt_list;
pub use status_bar::render_status_bar;
pub use title_bar::render_title_bar;
