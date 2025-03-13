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
    fn set_none_active_panel(&self) -> Result<()>;

    fn set_radius(&self, radius: f64) -> Result<()>;
}

impl<R: Runtime> MacWindowExt<R> for WebviewWindow<R> {
    fn set_none_active_panel(&self) -> Result<()> {
        #[cfg(unix)]
        {
            // let panel = self.to_panel()?;

            // log::info!("panel");
            // #[allow(non_upper_case_globals)]
            // const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;

            // panel.set_level(NSMainMenuWindowLevel + 1);
            // log::info!("panel 1");

            // panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);
            // log::info!("panel 2");
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
