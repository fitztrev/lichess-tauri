# lichess tauri

## Development Setup

1. Follow prerequisities for installing Rust + Tauri here: https://tauri.app/v1/guides/getting-started/prerequisites

2. Run project

   ```
   npm install
   npm run tauri dev
   ```

## Icon generation

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
