# HK Pluginization Assessment

## Date

February 24, 2026

## Question

Why was `BatchalignHK` private, and what is required to migrate it into
`batchalign3` without retaining a private fork?

## Findings

`BatchalignHK` is mainly a downstream patch set with four HK-specific additions:

1. Tencent Cloud ASR
2. Aliyun ASR
3. FunASR ASR
4. Cantonese-specific Wave2Vec FA preprocessing

The rest of the fork largely tracks upstream behavior.

## Why It Was Private

Primary practical reasons:

1. Credential-gated services:
   Tencent and Aliyun require provider accounts, project keys, and cloud resources.
2. Region-specific dependency burden:
   extra SDK stacks and a vendored Aliyun NLS client are not needed by most users.
3. Operational autonomy:
   HK collaborators could iterate independently without coupling all changes to
   the main public repo cadence.

No evidence suggests privacy was required by core architecture; plugin packaging
is sufficient.

## Permission / Access Requirements for Live Testing

To run end-to-end HK tests, the operator needs:

1. `~/.batchalign.ini` Tencent keys:
   - `engine.tencent.id`
   - `engine.tencent.key`
   - `engine.tencent.region`
   - `engine.tencent.bucket`
2. `~/.batchalign.ini` Aliyun keys:
   - `engine.aliyun.ak_id`
   - `engine.aliyun.ak_secret`
   - `engine.aliyun.ak_appkey`
3. Network access from runtime hosts to Tencent/Aliyun ASR endpoints.

Without these permissions, only non-network validation is possible
(plugin discovery, dispatch, CLI option routing, import-time checks).

## Implemented Migration Direction

HK functionality is moved into a plugin package:

- `~/talkbank/batchalign-hk-plugin`
- plugin entry point: `batchalign.plugins`
- registered engines:
  - `tencent`
  - `tencent_utr`
  - `aliyun`
  - `funaudio`
  - `funaudio_utr`
  - `wav2vec_fa_canto`

Rust CLI now supports explicit engine selection flags:

- `--asr-engine`
- `--fa-engine`
- `--utr-engine`

This removes the need for core-repo HK hacks while keeping the base install slim.

## 10-Year Architecture Position

For long-term maintainability, HK should stay as an external plugin project with:

1. independent release lifecycle
2. explicit compatibility range against `batchalign3`
3. stable plugin API contract on the host side

This is more durable than periodically merging HK-specific cloud/provider code
into core, and it minimizes future migration debt.

## Remaining Work Before Release

1. Run live provider smoke tests with real credentials.
2. Validate long-audio and failure-retry behavior for each provider.
3. Confirm third-party vendored SDK licensing posture for distribution.
4. Publish plugin packaging guidance and support policy.

## Decision

Continue with plugin-based HK support and keep provider-specific functionality out
of core runtime defaults. This preserves compatibility, reduces merge debt, and
keeps installation complexity optional.
