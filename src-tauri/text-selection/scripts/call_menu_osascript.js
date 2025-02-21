const appName = 'Code'

const app = Application(appName)
app.activate()

const sys = Application('System Events')
const proc = sys.processes.byName(appName)

const menuBar = proc.menuBars[0]
const menuBarItem = menuBar.menuBarItems.byName('Edit')
const menu = menuBarItem.menus[0]
const menuItem = menu.menuItems.byName('Copy')

menuItem.click()
