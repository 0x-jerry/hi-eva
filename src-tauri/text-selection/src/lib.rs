pub mod types;
pub mod utils;

#[cfg(windows)]
mod win_impl;

pub use types::*;

pub fn detect_selected_text() -> Result<TextSelectionDetectResult> {
    let host = {
        #[cfg(windows)]
        win_impl::HostImpl::default()
    };

    let selected_text = host.detect_selected_text()?;

    match selected_text {
        TextSelectionDetectResult::None => {
            let text_from_clipboard = utils::get_selected_text_from_clipboard(&host)?;

            if !text_from_clipboard.is_empty() {
                return Ok(TextSelectionDetectResult::Text(text_from_clipboard));
            }

            return Ok(TextSelectionDetectResult::None);
        }
        _ => return Ok(selected_text),
    }
}
