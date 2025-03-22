import { exec } from '@0x-jerry/utils/node'
import { copyFile } from 'node:fs/promises'
import path from 'node:path'

buildTextSelectionHelper()

async function buildTextSelectionHelper() {
  const cwd = path.resolve('src-tauri')

  const binFileConfig = {
    from: path.join(cwd, 'target/release/text-selection-helper'),
    to: path.join(cwd, 'bin/text-selection-helper'),
  }

  process.chdir(cwd)

  await exec('cargo build --release -p text-selection-helper', {
    env: process.env,
  })

  await copyFile(binFileConfig.from, binFileConfig.to)

  console.log('Done')
}
