# Lichess Local Engine

![image](https://user-images.githubusercontent.com/271432/232246956-27ac56e2-24c1-4543-8da9-7cdbd71075d6.png)

Recent versions of [Stockfish](https://stockfishchess.org/) have gotten too big to run in the browser. Lichess has [created an API](https://lichess.org/api#tag/External-engine) that allows you to run an engine on your own computer and send the results back to their Analysis page. This is a desktop app that intends to make it easy to use.

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
