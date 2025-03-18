use tauri::{Result, Runtime, WebviewWindow};

use tauri_nspanel::objc::class;
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
    fn ns_hide(&self) -> Result<()>;
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
                NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                    | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                    | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
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

            if !check_menubar_frontmost() {
                self.run_on_main_thread(move || {
                    panel.show();
                })
                .unwrap();

                log::info!("make key window");
            }
        }

        Ok(())
    }

    fn ns_hide(&self) -> Result<()> {
        #[cfg(unix)]
        {
            let panel = self.get_webview_panel(self.label()).unwrap();

            if check_menubar_frontmost() {
                self.run_on_main_thread(move || {
                    panel.resign_key_window();
                })
                .unwrap();

                log::info!("resign key window");
            }
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

fn app_pid() -> i32 {
    let process_info: id = unsafe { msg_send![class!(NSProcessInfo), processInfo] };

    let pid: i32 = unsafe { msg_send![process_info, processIdentifier] };

    pid
}

fn get_frontmost_app_pid() -> i32 {
    let workspace: id = unsafe { msg_send![class!(NSWorkspace), sharedWorkspace] };

    let frontmost_application: id = unsafe { msg_send![workspace, frontmostApplication] };

    let pid: i32 = unsafe { msg_send![frontmost_application, processIdentifier] };

    pid
}

fn check_menubar_frontmost() -> bool {
    get_frontmost_app_pid() == app_pid()
}
