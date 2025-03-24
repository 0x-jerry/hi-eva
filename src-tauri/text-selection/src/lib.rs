pub mod types;

#[cfg(unix)]
mod unix_impl;
#[cfg(windows)]
mod win_impl;

pub use types::*;

pub fn get_selected_text() -> Result<String> {
    #[cfg(windows)]
    let host = win_impl::HostImpl::default();
    #[cfg(unix)]
    let host = unix_impl::HostImpl::default();

    let selected_result = host.get_selected_text()?;

    return Ok(selected_result);
}

pub fn get_selected_rect() -> Option<SelectionRect> {
    #[cfg(windows)]
    let host = win_impl::HostImpl::default();
    #[cfg(unix)]
    let host = unix_impl::HostImpl::default();

    match host.detect_selection_rect() {
        Ok(result) => return result,
        Err(err) => {
            log::error!("get_selected_rect error: {}", err);
            return None;
        }
    }
}

pub fn init() {
    #[cfg(unix)]
    {
        let _ = unix_impl::request_accessibility_access();
    }
}
