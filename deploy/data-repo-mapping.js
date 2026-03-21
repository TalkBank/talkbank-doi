// Data repo mapping for TBB on talkbank.org
//
// All 24 data repos are cloned at /home/macw/data/ on talkbank.org.
// GitHub Actions already runs `git pull` on push.
//
// For 14 of 18 banks the repo list has one entry — resolveDataPath does
// one fs.existsSync, identical to current code. For the 4 split banks
// (childes, ca, phon, homebank) it does 2–4 lookups.

const fs = require('fs');
const path = require('path');

const DATA_ROOT = '/home/macw/data';

const BANK_REPOS = {
  aphasia:   ['aphasia-data'],
  asd:       ['asd-data'],
  biling:    ['biling-data'],
  ca:        ['ca-candor-data', 'ca-data'],
  childes:   ['childes-eng-na-data', 'childes-eng-uk-data', 'childes-romance-germanic-data', 'childes-other-data'],
  class:     ['class-data'],
  dementia:  ['dementia-data'],
  fluency:   ['fluency-data'],
  homebank:  ['homebank-public-data', 'homebank-cougar-data', 'homebank-bergelson-data', 'homebank-password-data'],
  motor:     ['motor-data'],
  phon:      ['phon-eng-french-data', 'phon-other-data'],
  psychosis: ['psychosis-data'],
  rhd:       ['rhd-data'],
  samtale:   ['samtale-data'],
  slabank:   ['slabank-data'],
  tbi:       ['tbi-data'],
};

// Reverse lookup: repo name → bank name
const REPO_TO_BANK = {};
for (const [bank, repos] of Object.entries(BANK_REPOS)) {
  for (const repo of repos) {
    REPO_TO_BANK[repo] = bank;
  }
}

/**
 * Virtual URL path → real filesystem path.
 *
 * resolveDataPath('childes', 'Eng-NA/MacWhinney/foo.cha')
 *   → '/home/macw/data/childes-eng-na-data/Eng-NA/MacWhinney/foo.cha'
 *
 * @param {string} bank - e.g., "childes"
 * @param {string} subPath - e.g., "Eng-NA/MacWhinney/foo.cha"
 * @returns {string|null} Absolute path, or null if not found
 */
function resolveDataPath(bank, subPath) {
  const repos = BANK_REPOS[bank];
  if (!repos) return null;

  for (const repo of repos) {
    const fullPath = path.join(DATA_ROOT, repo, subPath);
    if (fs.existsSync(fullPath)) {
      return fullPath;
    }
  }
  return null;
}

/**
 * Real filesystem path → virtual URL path.
 *
 * toVirtualPath('/home/macw/data/childes-eng-na-data/Eng-NA/MacWhinney/foo.cha')
 *   → { bank: 'childes', virtualPath: '/childes/data-orig/Eng-NA/MacWhinney/foo.cha' }
 *
 * @param {string} filePath - Absolute path under DATA_ROOT
 * @returns {{bank: string, virtualPath: string}|null}
 */
function toVirtualPath(filePath) {
  const rel = path.relative(DATA_ROOT, filePath);
  const parts = rel.split(path.sep);
  const repoName = parts[0];
  const bank = REPO_TO_BANK[repoName];
  if (!bank) return null;

  const subPath = parts.slice(1).join('/');
  return { bank, virtualPath: `/${bank}/data-orig/${subPath}` };
}

/**
 * List directory contents across all repos for a bank.
 * Merges entries from split repos, deduplicates, filters .git/.gitignore.
 *
 * listBankDir('childes', '')  → ['Biling', 'Chinese', 'Eng-NA', 'Eng-UK', ...]
 *
 * @param {string} bank - e.g., "childes"
 * @param {string} subPath - e.g., "" for root, "Eng-NA/" for subdirectory
 * @returns {string[]} Sorted, deduplicated list of entries
 */
function listBankDir(bank, subPath) {
  const repos = BANK_REPOS[bank];
  if (!repos) return [];

  const entries = new Set();
  for (const repo of repos) {
    const dirPath = path.join(DATA_ROOT, repo, subPath);
    try {
      for (const entry of fs.readdirSync(dirPath)) {
        if (entry === '.git' || entry === '.gitignore') continue;
        entries.add(entry);
      }
    } catch (e) {
      // Directory doesn't exist in this repo — skip
    }
  }
  return [...entries].sort();
}

module.exports = { DATA_ROOT, BANK_REPOS, resolveDataPath, toVirtualPath, listBankDir };
