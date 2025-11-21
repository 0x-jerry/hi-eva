import { getCurrentWindow } from '@tauri-apps/api/window'

export async function checkWindowPosition() {
  const win = getCurrentWindow()
  const size = (await win.outerSize()).toLogical(devicePixelRatio)

  const winPos = (await win.outerPosition()).toLogical(devicePixelRatio)
  let shouldUpdate = false

  const gap = 10

  const bottom = screen.availHeight - (winPos.y + size.height)

  const right = screen.availWidth - (winPos.x + size.width)

  if (bottom < gap) {
    const nextY = screen.availHeight - (size.height + gap)
    winPos.y = nextY
    shouldUpdate = true
  }

  if (right < gap) {
    const nextX = screen.availWidth - (size.width + gap)
    winPos.x = nextX
    shouldUpdate = true
  }

  if (shouldUpdate) {
    await win.setPosition(winPos)
  }
}
