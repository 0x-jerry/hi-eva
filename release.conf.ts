import { writeFile } from 'node:fs/promises'
import { readJson } from '@0x-jerry/utils/node'
import { defineConfig } from '@0x-jerry/x-release'

export default defineConfig({
  publish: false,
  async beforeCommit(ctx) {
    const confPath = 'src-tauri/tauri.conf.json'
    const conf = (await readJson<Record<string, string>>(confPath)) || {}

    conf.version = ctx.nextVersion.replace(/[a-z]+\./g, '')

    await writeFile(confPath, JSON.stringify(conf, null, 2))
  },
})
