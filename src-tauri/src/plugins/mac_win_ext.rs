use tauri::{Result, Runtime, WebviewWindow};

#[cfg(unix)]
use tauri_nspanel::{
    cocoa::{
        appkit::{NSMainMenuWindowLevel, NSView, NSWindow},
        base::id,
    },
    objc::{msg_send, sel, sel_impl},
    WebviewWindowExt,
};

pub trait MacWindowExt<R: Runtime> {
    fn non_active_hide(&self) -> Result<()>;
    fn non_active_show(&self) -> Result<()>;

    fn set_radius(&self, radius: f64) -> Result<()>;
}

impl<R: Runtime> MacWindowExt<R> for WebviewWindow<R> {
    fn non_active_show(&self) -> Result<()> {
        #[cfg(unix)]
        {
            let panel = self.to_panel()?;

            panel.set_level(NSMainMenuWindowLevel + 1);
        }

        Ok(())
    }

    fn non_active_hide(&self) -> Result<()> {
        #[cfg(unix)]
        {
            let panel = self.to_panel()?;

            panel.order_out(None);
        }

        Ok(())
    }

    fn set_radius(&self, radius: f64) -> Result<()> {
        #[cfg(unix)]
        {
            let win = self.ns_window()?;
            let win = win as id;

            // Set window radius
            unsafe {
                let view: id = win.contentView();

                view.wantsLayer();

                let layer: id = view.layer();

                let _: () = msg_send![layer, setCornerRadius: radius];
            }
        }

        Ok(())
    }
}
