# Lichess Local Engine

Run a more powerful version of Stockfish on your computer and connect it to the Analysis board. Lichess calls this an "External Engine". This is a desktop app that intends to make it easy to use.

![image](https://user-images.githubusercontent.com/271432/211694424-b8d955be-3275-4637-a4d4-5b1397f42c53.png)

## Development Setup

1. Follow prerequisities for installing Rust + Tauri here: https://tauri.app/v1/guides/getting-started/prerequisites

2. Run project

   ```
   npm install
   npm run tauri dev
   ```

## Architecture + How It Works

This is a Tauri-based app -- Tauri is like Electron except it uses Rust instead of Node.js. This app is written in Rust and Typescript.

Tauri uses the OS's native webview. It can be compiled to a native binary for Windows, Mac, and Linux.

## Development Notes

### Icon generation

```
npx svgexport src/assets/lichess-pad4-white.svg icon.png 64x
npm run tauri icon icon.png
```

Clear the cached build for it to take effect:

```
rm -rf src-tauri/target
npm run tauri dev
```

### SQLite dev

Uses diesel to manage migrations.

```
cargo install diesel_cli --no-default-features --features sqlite
```

To add a new migration:

```
cd src-tauri
diesel migration generate <migration_name>
```

For testing migrations:

```
cd src-tauri
diesel --database-url ~/.local/share/lichess-tauri/lichess-tauri.sqlite migration redo
```
