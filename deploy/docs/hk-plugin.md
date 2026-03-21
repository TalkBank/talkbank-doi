# Historical Note: HK Plugin Folded Into `batchalign3`

> **Status:** Retired in March 2026.
>
> `batchalign-hk-plugin` is no longer a current package. Its engines were folded
> into `batchalign3` as built-in HK/Cantonese engines.

## Current Release Guidance

Use the public `batchalign3` docs for anything current:

- `../batchalign3/book/src/user-guide/plugins.md`
- `../batchalign3/book/src/architecture/hk-cantonese-engines.md`

Current install surface:

- `batchalign3`

HK/Cantonese engines are part of the normal package surface now; there is no
separate HK install tier.

Current built-in engine names:

- `tencent`
- `aliyun`
- `funaudio`
- `wav2vec_canto`

## What Stayed Conceptually True

These points from the retired plugin work remain relevant:

- Tencent and Aliyun still read credentials from `~/.batchalign.ini`
- Cantonese forced alignment still romanizes hanzi to jyutping before MMS
- Cantonese ASR output still goes through HK normalization before final CHAT
  assembly

## What Changed

- there is no live `PluginDescriptor` or `batchalign.plugins` discovery layer
- the old plugin package name and install commands are retired
- the old plugin draft's UTR-specific engine names are not part of the current
  public release contract

## Historical Design Archive

If the old plugin architecture needs to be studied for provenance, use:

- `../../archive/batchalign-plugin-system.md`
