const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const BINARY_NAME = 'zork-termux';
const packageBinary = path.join(__dirname, 'prebuilt', BINARY_NAME);

const isTermux =
  process.env.TERMUX_VERSION !== undefined ||
  process.env.PREFIX === '/data/data/com.termux/files/usr';

if (!isTermux) {
  console.warn('[zork-termux] This npm package is designed for Android Termux (arm64).');
}

if (fs.existsSync(packageBinary)) {
  fs.chmodSync(packageBinary, 0o755);
  console.log(`[zork-termux] Using packaged binary: ${packageBinary}`);
  process.exit(0);
}

console.warn('[zork-termux] No prebuilt binary bundled in this package instance.');
console.warn('[zork-termux] Trying local source fallback build...');

const repoRoot = path.resolve(__dirname, '..', '..');
const cargoToml = path.join(repoRoot, 'Cargo.toml');

if (!fs.existsSync(cargoToml)) {
  console.error('[zork-termux] Source tree not found (Cargo.toml missing).');
  console.error('[zork-termux] Install a release that includes prebuilt/zork-termux.');
  process.exit(1);
}

try {
  execSync('cargo build --release', {
    cwd: repoRoot,
    stdio: 'inherit'
  });

  let sourceBinary = path.join(repoRoot, 'target', 'release', BINARY_NAME);
  if (!fs.existsSync(sourceBinary)) {
    sourceBinary = path.join(
      repoRoot,
      'target',
      'aarch64-linux-android',
      'release',
      BINARY_NAME
    );
  }

  if (!fs.existsSync(sourceBinary)) {
    throw new Error('binary not found after build');
  }

  fs.mkdirSync(path.dirname(packageBinary), { recursive: true });
  fs.copyFileSync(sourceBinary, packageBinary);
  fs.chmodSync(packageBinary, 0o755);

  console.log(`[zork-termux] Binary built and installed at: ${packageBinary}`);
} catch (error) {
  console.error('[zork-termux] Build failed:', error.message);
  console.error('[zork-termux] In Termux install Rust: pkg install rust');
  process.exit(1);
}
