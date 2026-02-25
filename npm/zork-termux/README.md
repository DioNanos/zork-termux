# @mmmbuto/zork-termux

NPM distribution for **zork-termux** focused on **Android Termux (arm64)**.

## Install

```bash
npm i -g @mmmbuto/zork-termux
```

## Run

```bash
zork-termux
```

## Notes

- This package expects a prebuilt `prebuilt/zork-termux` binary.
- If prebuilt is missing, `postinstall` tries a local Rust build fallback.
- Linux/macOS users should prefer Homebrew tap installation.
