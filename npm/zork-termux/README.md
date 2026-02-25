# @mmmbuto/zork-termux

NPM distribution for **zork-termux** focused on **Android Termux (arm64)**.

## Prerequisites

```bash
pkg update
pkg install -y nodejs-lts
```

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
- If prebuilt is missing, install fails and asks to reinstall from npm release.
- Linux/macOS users should use Homebrew tap:

```bash
brew tap Dionanos/zork-termux
brew install zork-termux
```
