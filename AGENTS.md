# AGENTS.md — H-hat LSP

Context for AI agents and contributors working in this repository. For end-user install and packaging, see [README.md](README.md).

## Purpose

This repo is a **VS Code / Cursor extension** plus a **Language Server Protocol (LSP)** server for the **H-hat** language (quantum-oriented naming; the current implementation is a small core: syntax highlighting, completions, hovers, and light diagnostics).

## Requirements

- Node.js 20+
- npm 10+

## Repository layout

| Path | Role |
|------|------|
| `client/` | VS Code extension (language client), TextMate grammar, `contributes.languages` |
| `server/` | Node LSP server (`vscode-languageserver`) |
| `example.hat` | Sample source file |
| Root `package.json` | npm **workspaces**: `client`, `server` |

The editor language id is **`hhatq`**. File extensions: **`.hat`**, **`.hhat`**.

## Commands (run from repository root)

```bash
npm install          # install all workspace dependencies
npm run build        # build client then server (tsc → dist/)
npm run watch        # watch both packages (parallel)
npm run lint         # eslint in client and server
npm run package      # build and produce VSIX (client workspace)
```

## Run and debug

1. Open this folder as a workspace in VS Code or Cursor.
2. Press **F5** to launch **Run Extension (H-hat LSP)** (Extension Development Host).
3. In the new window, open or create a `.hat` / `.hhat` file so the client activates and starts the server.
4. Use **Output**: channel **H-hat Quantum Language Server** (or Extension Host log) to confirm a clean startup.

## LSP capabilities (current implementation)

Implemented in [`server/src/server.ts`](server/src/server.ts):

- **Text sync**: incremental.
- **Completion** and **hover** for symbols in `ALL_KNOWN`: `main`, `print`, numeric types `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64`, and `string`.
- **Hover** for declared variables of the form `name: type` (shows the declared type when hovering `name`).
- **Diagnostics**:
  - Information if the file does not contain a `main {` block.
  - Warning on **unknown types** in declarations of the form `name: type` (type not in the known list above).

There is **no** completion/hover yet for quantum gates or keywords such as `H`, `CNOT`, `qubit`, `measure`, `collapse`. If documentation elsewhere mentions them, treat that as aspirational until added to the server.

## Where to change what

| Goal | Where |
|------|--------|
| Completions, hover text, symbol list | `server/src/server.ts` — extend `KEYWORDS_AND_BUILTINS`, `TYPES`, or merge lists into `ALL_KNOWN` |
| Diagnostic rules | `validateTextDocument` in `server/src/server.ts` |
| Syntax highlighting / scopes | `client/syntaxes/hhatq.tmLanguage.json` |
| Language id, extensions, activation | `client/package.json` → `contributes` |
| Wiring client ↔ server | `client/src` (extension entry compiles to `dist/extension.js`) |

After changing TypeScript, run **`npm run build`** at the root before manual extension tests or `npm run package`.

## Conventions for agents

- Write **documentation and comments in English**.
- Match existing style (e.g. section comments like `//* LSP imports` in the server).
- Prefer **small, task-focused changes**; do not refactor unrelated code.
- When you add or remove LSP behavior, **update this file’s “LSP capabilities” section** so it stays accurate.

## Related documentation

- [README.md](README.md) — user-facing setup, debug shortcut, manual test checklist, VSIX packaging.

## AGENTS.md maintenance (good practice)

- Keep sections short and **scannable**; prefer real paths and commands over prose.
- Describe behavior that **matches the code**, not planned features, unless clearly labeled as roadmap.
- Keep **README** (users) vs **AGENTS.md** (developers/agents): README can stay high-level; this file should say where to edit and what the server actually does.
