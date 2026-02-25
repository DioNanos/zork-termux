const fs = require('fs');
const path = require('path');

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

console.error('[zork-termux] Missing packaged binary prebuilt/zork-termux.');
console.error('[zork-termux] Reinstall from npm release package:');
console.error('  npm i -g @mmmbuto/zork-termux@latest');
process.exit(1);
