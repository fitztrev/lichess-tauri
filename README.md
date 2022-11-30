# lichess tauri

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
npm install svgexport -g
svgexport src/assets/lichess-white.svg lichess.png 64x
npm run tauri icon lichess.png
rm lichess.png
```

Delete the target directory and run `npm run tauri dev` again for it to take effect.

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
diesel migration generate <migration_name>
```

For testing migrations:

```
diesel --database-url ~/.local/share/lichess-tauri/lichess-tauri.sqlite migration redo
```
