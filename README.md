## H-hat LSP (VSCode + Node + TypeScript)

This repository contains a basic Language Server Protocol (LSP) implementation for a quantum programming language.

### Structure

- `client/`: VSCode extension (Language Client)
- `server/`: Node-based Language Server

### Requirements

- Node.js 20+
- npm 10+

### Install

```bash
npm install
```

### Build

```bash
npm run build
```

### Debug in VSCode

- Open this workspace in VSCode
- Press `F5` to launch **Run Extension (H-hat LSP)**
- Create a file with extension `.hat` or `.hhat` to activate the language server

### How to test the LSP (manual checklist)

- **Build first**: run `npm run build` (or use the default build task)
- **Start Extension Host**: press `F5` (a new VSCode window opens)
- **Create a test file**: in the Extension Host window, create `test.hat`
- **Check it activated**:
  - Open **Output** panel
  - Select **H-hat Quantum Language Server** (or **Log (Extension Host)**) and confirm there are no startup errors
- **Test completion**:
  - Type `main`, `print`, or a type such as `i32` and use `Ctrl+Space` (see [AGENTS.md](AGENTS.md) for the full symbol list)
- **Test hover**:
  - Hover `main`, `print`, or a known type (e.g. `i32`, `u64`)
- **Test diagnostics**:
  - Remove any `main { ... }` block and confirm an information diagnostic suggests adding one
  - Add a declaration with an unknown type, e.g. `x: unknownType`, and confirm a warning on the type name

### Packaging (VSIX)

```bash
npm run package
```
