# Lichess Local Engine

![image](https://user-images.githubusercontent.com/271432/232327082-11582b5b-cc6a-4db4-a6a8-1b3630c717d5.png)

Recent versions of [Stockfish](https://stockfishchess.org/) have gotten too big to run in the browser. Lichess has [created an API](https://lichess.org/api#tag/External-engine) that allows you to run an engine on your own computer and send the results back to their Analysis page. This is a desktop app that intends to make it easy to use.

## Download

Available for Windows, Mac, and Linux.

Still in development, but you can download the preview version here:

https://github.com/fitztrev/lichess-tauri/releases

## Development Setup

1. Follow prerequisities for installing Rust + Tauri here: https://tauri.app/v1/guides/getting-started/prerequisites

2. Run project

   ```bash
   npm install
   npm run tauri dev
   ```

## Architecture + How It Works

This is a Tauri-based app -- Tauri is like Electron except it uses Rust instead of Node.js. This app is written in Rust and Typescript.

Tauri uses the OS's native webview. It can be compiled to a native binary for Windows, Mac, and Linux.

## Development Notes

Commands and documentation related to development of this app.

### Testing

```bash
# test the utility functions for the frontend
npm run test

# same tests but in watch mode
npm run watch
```

### Formatting

Run linter

```bash
# for frontend
npm run format

# for backend
cd src-tauri
cargo fmt
```

### Auto-check format on commit

```bash
npm run prepare
```

Will apply the `.husky` commit hooks to your local git repo.

### Icon generation

All the app icons get generated from a single source SVG. To update the icons, run:

```bash
npx svgexport src/assets/lichess-pad4-white.svg icon.png 64x
npm run tauri icon icon.png
```

Clear the cached build for it to take effect locally:

```bash
rm -rf src-tauri/target
npm run tauri dev
```

### SQLite dev

Uses diesel to manage migrations.

```bash
cargo install diesel_cli --no-default-features --features sqlite
```

To add a new migration:

```bash
cd src-tauri
diesel migration generate <migration_name>
```

For testing migrations:

```bash
cd src-tauri
diesel --database-url ~/.local/share/lichess-tauri/lichess-tauri.sqlite migration redo
```

### How to release a new version

1. Test build locally

   ```bash
   npm run tauri build
   ```

1. Tag the new version:

   ```bash
   npm version 0.x.x
   git push origin main --tags
   ```

   Github workflow will build the app for each OS and add them to a release.

   - [Pre-release tag format](https://github.com/tauri-apps/tauri/issues/5286#issuecomment-1396970694): `0.0.6-2`

1. Manually edit the release and add the changelog to the description
