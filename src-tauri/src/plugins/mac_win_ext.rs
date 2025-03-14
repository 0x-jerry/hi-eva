use tauri::{Result, Runtime, WebviewWindow};

#[cfg(unix)]
use tauri_nspanel::{
    cocoa::{
        appkit::{NSMainMenuWindowLevel, NSView, NSWindow, NSWindowCollectionBehavior},
        base::id,
    },
    objc::{msg_send, sel, sel_impl},
    ManagerExt, WebviewWindowExt,
};

pub trait MacWindowExt<R: Runtime> {
    #[allow(dead_code)]
    fn ns_hide(&self) -> Result<()>;
    #[allow(dead_code)]
    fn ns_show(&self) -> Result<()>;
    fn to_non_active_panel(&self) -> Result<()>;

    fn set_radius(&self, radius: f64) -> Result<()>;
}

impl<R: Runtime> MacWindowExt<R> for WebviewWindow<R> {
    fn to_non_active_panel(&self) -> Result<()> {
        #[cfg(unix)]
        {
            let panel = self.to_panel()?;

            // Set panel level
            panel.set_level(NSMainMenuWindowLevel + 1);

            // Allows the panel to display on the same space as the full screen window
            panel.set_collection_behaviour(
                NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
            );

            #[allow(non_upper_case_globals)]
            const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;

            // Ensures the panel cannot activate the App
            panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);
        }

        Ok(())
    }

    fn ns_show(&self) -> Result<()> {
        #[cfg(unix)]
        {
            let panel = self.get_webview_panel(self.label()).unwrap();
            let is_visible = panel.is_visible();

            log::info!("visible {}", is_visible);
            log::info!("label {}", self.label());

            panel.show();
            log::info!("done")
        }

        #[cfg(not(unix))]
        {
            self.show()?;
        }

        Ok(())
    }

    fn ns_hide(&self) -> Result<()> {
        #[cfg(unix)]
        {
            let panel = self.get_webview_panel(self.label()).unwrap();

            if panel.is_visible() {
                panel.order_out(None);
            }
        }

        #[cfg(not(unix))]
        {
            self.hide()?;
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

        let _ = radius;
        Ok(())
    }
}
