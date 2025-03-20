use tauri::{Manager, Result, Runtime, WebviewWindow};

use tauri_nspanel::{
    cocoa::{
        appkit::{NSMainMenuWindowLevel, NSView, NSWindow, NSWindowCollectionBehavior},
        base::id,
    },
    objc::{msg_send, sel, sel_impl},
    ManagerExt, WebviewWindowExt,
};

use crate::core::AppState;

pub trait MacWindowExt<R: Runtime> {
    fn ns_resign_focus(&self) -> Result<()>;
    fn ns_focus(&self) -> Result<()>;
    fn to_non_active_panel(&self) -> Result<()>;

    fn set_radius(&self, radius: f64) -> Result<()>;
}

impl<R: Runtime> MacWindowExt<R> for WebviewWindow<R> {
    fn to_non_active_panel(&self) -> Result<()> {
        let panel = self.to_panel()?;

        // Set panel level
        panel.set_level(NSMainMenuWindowLevel + 1);

        // Allows the panel to display on the same space as the full screen window
        panel.set_collection_behaviour(
            NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
        );

        #[allow(non_upper_case_globals)]
        const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;

        // Ensures the panel cannot activate the App
        panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);

        Ok(())
    }

    // todo, rename to ns_toolbar_focus
    fn ns_focus(&self) -> Result<()> {
        let state = self.state::<AppState>();
        let mut state = state.lock().unwrap();

        if !state.toolbar.focused {
            let panel = self.get_webview_panel(self.label()).unwrap();
            self.run_on_main_thread(move || {
                panel.show();
            })
            .unwrap();

            log::info!("make key window");
        }
        state.toolbar.focused = true;

        Ok(())
    }

    // todo, rename to ns_toolbar_resign_focus
    fn ns_resign_focus(&self) -> Result<()> {
        let state = self.state::<AppState>();
        let mut state = state.lock().unwrap();

        if state.toolbar.focused {
            let panel = self.get_webview_panel(self.label()).unwrap();
            self.run_on_main_thread(move || {
                panel.resign_key_window();
            })
            .unwrap();

            log::info!("resign key window");
        }
        state.toolbar.focused = false;

        Ok(())
    }

    fn set_radius(&self, radius: f64) -> Result<()> {
        let win = self.ns_window()?;
        let win = win as id;

        // Set window radius
        unsafe {
            let view: id = win.contentView();

            view.wantsLayer();

            let layer: id = view.layer();

            let _: () = msg_send![layer, setCornerRadius: radius];
        }
        Ok(())
    }
}
