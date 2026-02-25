#!/usr/bin/env node

const https = require('https');
const pkg = require('./package.json');

function parse(version) {
  return version.split('.').map((x) => parseInt(x, 10) || 0);
}

function compare(a, b) {
  const av = parse(a);
  const bv = parse(b);
  for (let i = 0; i < Math.max(av.length, bv.length); i += 1) {
    const ai = av[i] || 0;
    const bi = bv[i] || 0;
    if (ai > bi) return 1;
    if (ai < bi) return -1;
  }
  return 0;
}

const url = 'https://registry.npmjs.org/@mmmbuto%2fzork-termux';

https
  .get(url, (res) => {
    if (res.statusCode !== 200) {
      console.log('[zork-termux] Unable to check updates right now.');
      return;
    }

    let raw = '';
    res.on('data', (chunk) => {
      raw += chunk;
    });

    res.on('end', () => {
      try {
        const data = JSON.parse(raw);
        const latest = data['dist-tags'] && data['dist-tags'].latest;

        if (!latest) {
          console.log('[zork-termux] No latest tag found on npm.');
          return;
        }

        if (compare(latest, pkg.version) > 0) {
          console.log(`[zork-termux] Update available: ${pkg.version} -> ${latest}`);
          console.log('[zork-termux] Run: npm i -g @mmmbuto/zork-termux@latest');
        } else {
          console.log(`[zork-termux] You are up to date (${pkg.version}).`);
        }
      } catch (error) {
        console.log('[zork-termux] Failed to parse npm response.');
      }
    });
  })
  .on('error', () => {
    console.log('[zork-termux] Update check failed (network error).');
  });
