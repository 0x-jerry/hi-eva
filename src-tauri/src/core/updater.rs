use tauri::{async_runtime::block_on, AppHandle, Url};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_updater::UpdaterExt;

use crate::core::ConfigurationExt;

pub fn check_update(app: &AppHandle) {
    let app = app.clone();

    std::thread::spawn(move || {
        if let Err(err) = check_update_inner(&app) {
            let msg = format!("Check update failed: {}", err);
            let app_name = app.package_info().name.clone();

            app.notification()
                .builder()
                .title(app_name)
                .body(msg)
                .show()
                .unwrap();
        }
    });
}

fn check_update_inner(app: &AppHandle) -> tauri_plugin_updater::Result<()> {
    let app_name = app.package_info().name.clone();
    log::info!("start check update");

    let mut updater_builder = app.updater_builder();

    let proxy = app.get_conf().proxy;

    let url = Url::parse(proxy.as_str());

    if url.is_ok() {
        log::info!("set proxy {:?}", url);

        updater_builder = updater_builder.proxy(url.unwrap());
    }

    let updater = updater_builder.build()?;

    if let Some(update) = block_on(updater.check())? {
        let mut downloaded = 0;

        log::info!("start download");

        let binary = update.download(
            |chunk_length, content_length| {
                downloaded += chunk_length;
                log::info!("downloaded {downloaded} from {content_length:?}");
            },
            || {
                log::info!("download finished");
            },
        );

        let binary = block_on(binary)?;

        let answer = app
            .dialog()
            .message("Updates is ready, confirm to install")
            .title(app_name)
            .buttons(MessageDialogButtons::OkCancel)
            .blocking_show();

        if answer {
            update.install(binary)?;
            app.restart();
        }
    } else {
        app.notification()
            .builder()
            .title(app_name)
            .body("You are using the latest version")
            .show()
            .unwrap();
    }

    Ok(())
}
