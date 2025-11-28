use anyhow::Result;
use tauri::{Manager, WebviewWindow};

use tauri_nspanel::{
    tauri_panel, CollectionBehavior, ManagerExt, PanelLevel, StyleMask, WebviewWindowExt,
};

use crate::core::AppState;

tauri_panel! {
    panel!(HoverActivatePanel {
        config: {
            can_become_main_window: false,
            can_become_key_window: true,
            becomes_key_only_if_needed: true,
            is_floating_panel: true
        }
    })
}

pub trait MacWindowExt {
    fn ns_resign_focus(&self) -> Result<()>;
    fn ns_focus(&self) -> Result<()>;
    fn to_non_active_panel(&self) -> Result<()>;

    fn set_radius(&self, radius: f64) -> Result<()>;
}

impl MacWindowExt for WebviewWindow {
    fn to_non_active_panel(&self) -> Result<()> {
        let panel = self.to_panel::<HoverActivatePanel>()?;

        panel.set_level(PanelLevel::Floating.value());
        panel.set_style_mask(StyleMask::empty().nonactivating_panel().into());
        panel.set_collection_behavior(
            CollectionBehavior::new()
                .full_screen_auxiliary()
                .can_join_all_spaces()
                .into(),
        );

        panel.set_hides_on_deactivate(false);
        panel.set_works_when_modal(true);

        Ok(())
    }

    /// only used by toolbar window
    fn ns_focus(&self) -> Result<()> {
        let state = self.state::<AppState>();

        if !state.is_toolbar_focused() {
            let panel = self.get_webview_panel(self.label()).unwrap();
            self.run_on_main_thread(move || {
                panel.show_and_make_key();
            })
            .unwrap();

            log::info!("make key window");
        }
        state.set_toolbar_focused(true);

        Ok(())
    }

    /// only used by toolbar window
    fn ns_resign_focus(&self) -> Result<()> {
        let state = self.state::<AppState>();

        if state.is_toolbar_focused() {
            let panel = self.get_webview_panel(self.label()).unwrap();
            self.run_on_main_thread(move || {
                panel.resign_key_window();
            })
            .unwrap();

            log::info!("resign key window");
        }

        state.set_toolbar_focused(false);

        Ok(())
    }

    fn set_radius(&self, radius: f64) -> Result<()> {
        let win = self.ns_window()?;

        // Set window radius
        unsafe {
            let win = win as *mut NSWindow;

            let win = win.as_ref().unwrap();

            let view = win.contentView().unwrap();

            view.wantsLayer();

            let layer = view.layer().unwrap();

            let _: () = msg_send![&*layer, setCornerRadius: radius];
        }
        Ok(())
    }
}
