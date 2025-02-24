use std::error;
use std::thread::sleep;
use std::time::Duration;

use clipboard_rs::{Clipboard, ClipboardContext, ContentFormat};

use crate::types::Result;

pub trait ClipboardHostTrait {
    fn trigger_copy_action(&self) -> Result<()>;
}

pub fn get_selected_text_from_clipboard<T: ClipboardHostTrait>(host: &T) -> Result<String> {
    let r = copy_from_clipboard(host)
        //
        .map_err(|err| err.to_string().into());

    return r;
}

fn copy_from_clipboard<T: ClipboardHostTrait>(
    host_ctx: &T,
) -> std::result::Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let ctx = ClipboardContext::new()?;

    let formats = vec![
        ContentFormat::Text,
        ContentFormat::Files,
        ContentFormat::Image,
        ContentFormat::Rtf,
        ContentFormat::Html,
    ];

    let old_datas = ctx.get(&formats)?;

    ctx.clear();
    let result = host_ctx.trigger_copy_action();

    if let Err(err) = result {
        return Err(err.to_string().into());
    }

    sleep(Duration::from_millis(50));

    let result = ctx.get_text()?;

    ctx.set(old_datas)?;

    Ok(result.trim().to_string())
}
