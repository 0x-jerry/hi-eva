use std::io::Error;
use std::thread::sleep;
use std::time::Duration;
use std::{error, io::ErrorKind};

use clipboard_rs::{Clipboard, ClipboardContext, ContentFormat};

pub trait ClipboardHostTrait {
    fn trigger_copy_action(&self) -> Result<(), Error>;
}

pub fn get_selected_text_from_clipboard<T: ClipboardHostTrait>(host: &T) -> Result<String, Error> {
    match copy_from_clipboard(host) {
        Ok(text) => Ok(text),
        Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
    }
}

fn copy_from_clipboard<T: ClipboardHostTrait>(
    host_ctx: &T,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let ctx = ClipboardContext::new()?;

    let formats = vec![
        ContentFormat::Text,
        ContentFormat::Files,
        ContentFormat::Image,
        ContentFormat::Rtf,
        ContentFormat::Html,
    ];

    let old_datas = ctx.get(&formats)?;

    host_ctx.trigger_copy_action()?;

    sleep(Duration::from_millis(50));

    let result = ctx.get_text()?;

    ctx.set(old_datas)?;

    Ok(result.trim().to_string())
}
