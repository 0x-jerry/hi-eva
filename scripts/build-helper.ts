import { exec } from '@0x-jerry/utils/node'
import { execSync } from 'node:child_process'
import { copyFile } from 'node:fs/promises'
import path from 'node:path'

buildTextSelectionHelper()

async function buildTextSelectionHelper() {
  const cwd = path.resolve('src-tauri')

  const name = 'text-selection-helper'
  const binFileConfig = {
    from: path.join(cwd, `target/release/${name}`),
    to: path.join(cwd, `bin/${name}-${await getSuffix()}`),
  }

  process.chdir(cwd)

  await exec('cargo build --release -p text-selection-helper', {
    env: process.env,
  })

  await copyFile(binFileConfig.from, binFileConfig.to)

  console.log('Done')
}

async function getSuffix() {
  const extension = process.platform === 'win32' ? '.exe' : ''

  const rustInfo = execSync('rustc -vV').toString('utf8')
  const targetTriple = /host: (\S+)/g.exec(rustInfo)?.[1]

  if (!targetTriple) {
    throw new Error('Failed to determine platform target triple')
  }

  return `${targetTriple}${extension}`
}
