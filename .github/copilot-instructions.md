## Copilot Instructions for hi-eva

This file gives focused, actionable guidance for AI coding agents working in this repo. Keep suggestions specific to discovered patterns and commands.

- **Project type:** Vue 3 + TypeScript UI bundled into a Tauri desktop app (Rust). Frontend powered by `vite`, `unocss`, `primevue`; backend/native via `src-tauri` (Tauri + Rust).

- **Install:** Use pnpm (CI and workflows use pnpm). Run:
  - `pnpm i`

- **Local development:**
  - UI-only (hot reload): `pnpm ui:dev` — serves at `http://localhost:8990` (see `vite.config.ts`).
  - Full Tauri dev (UI + Rust): `pnpm dev` — runs `tauri dev`, which executes `pnpm ui:dev` first (see `src-tauri/tauri.conf.json`).
  - Build UI: `pnpm ui:build`
  - Build App: `pnpm build` (runs `tauri build`).

- **Ports / HMR:** Vite is configured to use `port = 8990` and `strictPort: true` in `vite.config.ts`. `TAURI_DEV_HOST` may be set in CI/remote HMR scenarios. Do not change port unless updating `src-tauri/tauri.conf.json` devUrl.

- **Tauri integration notes:**
  - `src-tauri/tauri.conf.json` defines `beforeDevCommand`/`beforeBuildCommand` (runs UI commands). Keep these in sync if you change scripts.
  - Plugins used: `sql` (preload `sqlite:data.db`), `store`, and `updater` — check `tauri.conf.json` for endpoints and behavior.
  - Native commands (Rust) are defined under `src-tauri/src/commands.rs`. JS/TS calls use a proxy in `src/logic/commands.ts` that maps method names to snake_case Tauri commands via `invoke()`.

- **Where to look for major patterns:**
  - Chat / AI usage: `src/components/Chat/chat.ts` — shows OpenAI client creation and `stream: true` usage; endpoints and models are resolved via `src/logic/config`.
  - Command bridge to Rust: `src/logic/commands.ts` — produces a typed proxy; signatures must match `src-tauri/src/commands.rs`.
  - Database layer: `src/database/*` — sqlite-backed model managers (e.g., `chatHistory.ts`, `chatHistoryMsg.ts`) use a `BaseModelManager` pattern. Persistence is via Tauri SQL plugin using the `sqlite:data.db` preload.
  - Router: `unplugin-vue-router` + generated types at `src/typed-router.d.ts` (see `vite.config.ts` router plugin config).

- **API keys & endpoints:**
  - Endpoints and API keys are stored in the app configuration (`logic/config` and `useEndpointsConfig` composable). The frontend sometimes creates an OpenAI client with `dangerouslyAllowBrowser: true` (see `src/components/Chat/chat.ts`) — prefer using stored endpoints rather than hardcoding keys.

- **CI / Releases:**
  - GitHub Actions: `.github/workflows/release.yml` — the release job uses `pnpm` and `tauri-action`. Secrets required: `TAURI_PRIVATE_KEY`, `RELEASE_GITHUB_TOKEN`.

- **Coding conventions & expectations for changes:**
  - Preserve Tauri command signatures: when adding new native commands, update both `src-tauri/src/commands.rs` and the TypeScript `ICommandType` in `src/logic/commands.ts`.
  - Use the existing `BaseModelManager` pattern in `src/database` for new persistent entities so data loading (e.g., `getAllById`) remains consistent.
  - For UI: prefer composables in `src/composables` and settings patterns in `src/components/settings/*` when adding configuration UI.
  - When altering dev/build scripts, update `src-tauri/tauri.conf.json` `beforeDevCommand`/`devUrl` accordingly.

- **Common pitfalls to avoid:**
  - Changing Vite dev port without updating Tauri config will break `tauri dev` flow.
  - Mismatched Tauri command names/signatures between Rust and TS causes runtime `invoke` errors — use `snake_case` mapping in `commands.ts` as reference.
  - Storing raw API keys in repo is forbidden; use the endpoint config UI or environment-based injection for CI.

- **Examples to reference when implementing tasks:**
  - Create a streaming chat: follow `src/components/Chat/chat.ts` and use `getPromptConf()` + `getEndpointConf()` from `src/logic/config`.
  - Call native command: `await commands.getSelectedText()` (see `src/logic/commands.ts` proxy and `src-tauri/src/commands.rs`).
  - Read/write persisted chat: use `chatHistoryTable.getAllById(id)` (see `src/database/chatHistory.ts`).


## You should follow those rules

1. Do not add useless comment, make code more readable and efficient.
2. Do not use literal object types, always define an interface or type alias for complex objects.
3. Follow SOLID principles and best practices of software design.

If anything here is unclear or you'd like more detail about a particular area (e.g., Rust commands, database schema, or deployment), tell me which part to expand and I will iterate.
