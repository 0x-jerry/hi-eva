use serde::Serialize;

#[allow(dead_code)]
pub fn trigger_copy_menu_for_app(app_name: &str) -> bool {
    #[derive(Serialize)]
    struct ScriptParams {
        name: String,
    }

    let code = "
        const appName = $params.name
        const app = Application(appName)
        app.activate()

        const sys = Application('System Events')
        const proc = sys.processes.byName(appName)

        const menuBar = proc.menuBars[0]
        const menuBarItem = menuBar.menuBarItems.byName('Edit')
        const menu = menuBarItem.menus[0]
        const menuItem = menu.menuItems.byName('Copy')

        menuItem.click()
    ";

    let script = osascript::JavaScript::new(&code);

    if let Err(err) = script.execute_with_params::<ScriptParams, ()>(ScriptParams {
        name: app_name.to_string(),
    }) {
        log::error!("Error: {}", err);
        return false;
    }

    return true;
}
