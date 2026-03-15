# Code Signing, Notarization, and Distribution

**Status:** Reference
**Last updated:** 2026-03-15

This document covers code signing and distribution for all TalkBank projects that ship binaries to end users. Our users are primarily academic researchers (psychology, linguistics, law) who are not technical — they need installers that "just work" without Gatekeeper warnings or manual security exceptions.

## Projects That Need Signing

| Project | Binary type | Platforms | Current state |
|---------|------------|-----------|---------------|
| **java-chatter-stable** | Java `.app` via `jpackage` + DMG | macOS only | Signed + notarized (manual, from dev machine) |
| **batchalign3** (`batchalign3`) | Rust binary via maturin | macOS, Linux, Windows | Not signed yet for standalone archive distribution |
| **talkbank-tools** (`chatter`, `talkbank-lsp-server`) | Rust CLIs via cargo | macOS, Linux, Windows | Not signed yet; current public-release workflow emits macOS `.tar.gz` archives, so notarization is not wired |
| **batchalign3** (Python wheel) | PyO3 `.so`/`.pyd` + CLI entry point | macOS, Linux, Windows | Not signed (installed via `uv`/`pip`) |

## Release Decision Summary

- **`java-chatter-stable` DMG on macOS:** yes, it needs Apple signing +
  notarization. That path already exists and is documented below.
- **`talkbank-tools` public macOS downloads (`chatter`,
  `talkbank-lsp-server`):** yes, treat Apple signing + notarization as required
  before the first public GitHub Release. The current macOS `.tar.gz`
  packaging must become a notarizable `.zip` or `.dmg` first.
- **`batchalign3` PyPI wheel path:** not a notarization blocker while PyPI is on
  hold. Keep signing secrets/workflow prep ready, but this is lower urgency than
  the first public `talkbank-tools` release.
- **`batchalign3` standalone macOS archives or the revived desktop dashboard:**
  yes, sign + notarize if we ship them as direct downloads.

## Apple Developer Account

- **Team ID:** `45EEEGL6UX`
- **Account holder:** Brian MacWhinney
- **Developer ID Application CN:** `Developer ID Application: Brian MacWhinney (45EEEGL6UX)`
- **Apple Developer portal:** https://developer.apple.com/account

### Certificate Types We Need

| Certificate | Purpose | Used for |
|-------------|---------|----------|
| Developer ID Application | Sign `.app` bundles and standalone binaries | Chatter.app, `chatter`, `talkbank-lsp-server`, `batchalign3` CLI |
| Developer ID Installer | Sign `.pkg` installers (if we use them) | Optional — DMGs don't need this |

Both are issued under the team account and can be downloaded by any team member.

## Current State: Legacy Java Chatter

The Java Chatter signing process is documented in `java-chatter-stable/build-mac-app.sh` and `java-chatter-stable/notarize.sh`. Here's what it does:

### Build + Sign (`build-mac-app.sh`)

1. **Sign helper binary:** `codesign --force --timestamp --options runtime --sign "$dev_id" send2clan-macos`
2. **Build fat JAR:** `mvn package`
3. **Create app-image:** `jpackage --mac-sign --mac-signing-key-user-name "$dev_id" --mac-entitlements entitlements.plist`
4. **Reseal .app:** `codesign --force --timestamp --options runtime --entitlements entitlements.plist --sign "$dev_id" Chatter.app` (belt-and-suspenders — jpackage's signing can miss the launcher)
5. **Create DMG:** `hdiutil create -volname Chatter -srcfolder Chatter.app -format UDZO Chatter-$version.dmg`

### Notarize (`notarize.sh`)

1. **Submit:** `xcrun notarytool submit Chatter-$version.dmg --keychain-profile "notarytool-password" --wait --verbose`
2. **Staple:** `xcrun stapler staple Chatter-$version.dmg`
3. **Verify:** `spctl --assess --type open --context context:primary-signature -vv Chatter-$version.dmg`

### Entitlements (`entitlements.plist`)

Java 25 requires JIT entitlements:
```xml
<key>com.apple.security.cs.allow-jit</key><true/>
<key>com.apple.security.cs.jit-write-protect</key><true/>
```

**Rust binaries do not need JIT entitlements.** They can use a minimal entitlements file or none at all (hardened runtime via `--options runtime` is sufficient).

### The Personal Keychain Problem

The notarization keychain profile (`notarytool-password`) was created interactively on the developer's machine:

```bash
xcrun notarytool store-credentials "notarytool-password" \
    --apple-id YOUR_APPLE_ID \
    --team-id 45EEEGL6UX \
    --password APP_SPECIFIC_PASSWORD
```

This stores credentials in the **login keychain** of whoever ran the command. It works on that machine only — not portable, not CI-friendly, and tied to a specific user account.

Similarly, the Developer ID certificate + private key were imported into the login keychain manually (probably via Xcode or `security import`). The `codesign` tool finds them by matching the CN string against keychain entries.

## Extracting Credentials from the Dev Machine

The Developer ID certificate and `notarytool-password` keychain profile live on one machine (Brian's or whoever originally ran `xcrun notarytool store-credentials`). Before any CI signing can work, those credentials must be exported and stored as GitHub Actions secrets.

**Script:** `deploy/scripts/extract-signing-secrets.sh`
**Run on:** the machine where `security find-identity -v -p codesigning | grep "Developer ID Application"` returns a result.

```bash
bash deploy/scripts/extract-signing-secrets.sh
```

The script:
- Verifies the Developer ID cert is present
- Exports the cert + private key as a `.p12` with a generated passphrase
- Prints the `.p12` as base64 (`APPLE_CERT_P12_BASE64`)
- Extracts the Apple ID and app-specific password from the `notarytool-password` keychain item
- Prints all values for pasting into GitHub Actions secrets
- Cleans up the temp `.p12` file

### Manual step: App Store Connect API key

The API key (recommended for CI notarization — avoids Apple ID + app-specific password in CI) cannot be created programmatically. It requires browser access with Account Holder role:

1. Go to https://appstoreconnect.apple.com/access/integrations/api
2. Click **+**, name it "TalkBank CI", role: Developer
3. Download the `.p8` file — **only downloadable once**
4. Note the Key ID (10-char string) and Issuer ID (UUID shown at the top)
5. Store `.p8` contents as `APPLE_API_KEY_P8`, key ID as `APPLE_API_KEY_ID`, issuer UUID as `APPLE_API_ISSUER`

### Where to store the output

- **1Password** — primary store for all extracted values
- **GitHub Actions secrets** — Settings → Secrets and variables → Actions → New repository secret, in both `talkbank-tools` and `batchalign3`

## How to Fix: Portable, CI-Ready Signing

### Option 1: App Store Connect API Key (Recommended for Notarization)

Instead of storing Apple ID credentials in the keychain, use an API key:

1. **Create API key** at https://appstoreconnect.apple.com/access/integrations/api (requires Account Holder or Admin role)
2. **Download the `.p8` file** (only downloadable once!)
3. **Store the key securely** (1Password, CI secrets, etc.)
4. **Use in notarytool:**
   ```bash
   xcrun notarytool submit app.dmg \
       --key /path/to/AuthKey_XXXXXXXXXX.p8 \
       --key-id XXXXXXXXXX \
       --issuer XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX \
       --wait
   ```
5. **In CI** (GitHub Actions), store key contents as a secret and write to a temp file:
   ```yaml
   - name: Write API key
     run: echo "${{ secrets.APPLE_API_KEY_P8 }}" > /tmp/AuthKey.p8
   - name: Notarize
     run: xcrun notarytool submit app.dmg --key /tmp/AuthKey.p8 --key-id ${{ secrets.APPLE_API_KEY_ID }} --issuer ${{ secrets.APPLE_API_ISSUER }} --wait
   ```

### Option 2: Export Certificate as .p12 (Required for Code Signing)

For `codesign` to work without the login keychain:

1. **Export from Keychain Access:** Right-click the "Developer ID Application" cert → Export → save as `.p12` with a passphrase
2. **In CI**, create a temporary keychain and import:
   ```bash
   # Create temporary keychain
   security create-keychain -p "$KEYCHAIN_PASSWORD" build.keychain
   security default-keychain -s build.keychain
   security unlock-keychain -p "$KEYCHAIN_PASSWORD" build.keychain

   # Import certificate
   security import cert.p12 -k build.keychain -P "$P12_PASSWORD" -T /usr/bin/codesign
   security set-key-partition-list -S apple-tool:,apple:,codesign: -s -k "$KEYCHAIN_PASSWORD" build.keychain

   # Now codesign will find it
   codesign --sign "Developer ID Application: Brian MacWhinney (45EEEGL6UX)" ...
   ```
3. **Clean up after build:**
   ```bash
   security delete-keychain build.keychain
   ```

### GitHub Actions Secrets Needed

| Secret | What | How to get it |
|--------|------|---------------|
| `APPLE_CERT_P12_BASE64` | Base64-encoded .p12 certificate | `base64 < cert.p12` |
| `APPLE_CERT_P12_PASSWORD` | Passphrase for .p12 | Set during export |
| `APPLE_API_KEY_P8` | App Store Connect API key contents | Download from portal |
| `APPLE_API_KEY_ID` | Key ID (10 chars) | Shown in portal |
| `APPLE_API_ISSUER` | Issuer UUID | Shown in portal |
| `APPLE_TEAM_ID` | `45EEEGL6UX` | Known |

## Signing Rust Binaries for macOS

Rust binaries are simple to sign compared to Java `.app` bundles — they're single-file Mach-O executables.

### Local signing (developer machine)

```bash
# Build release binary
cargo build --release -p batchalign-cli

# Sign with hardened runtime + timestamp
codesign --force --timestamp --options runtime \
    --sign "Developer ID Application: Brian MacWhinney (45EEEGL6UX)" \
    target/release/batchalign3

# Verify
codesign -dv --verbose=4 target/release/batchalign3
```

### Distributing as DMG or ZIP

Apple requires either a `.dmg`, `.pkg`, or `.zip` for notarization submission:

```bash
# Option A: ZIP (simplest for CLI tools)
zip batchalign3-macos-arm64.zip target/release/batchalign3
xcrun notarytool submit batchalign3-macos-arm64.zip --key AuthKey.p8 --key-id KEY_ID --issuer ISSUER --wait
# Note: stapling doesn't work on plain ZIPs — users get checked at first launch via Gatekeeper online check

# Option B: DMG (stapling works, more polished)
hdiutil create -volname batchalign3 -srcfolder release-staging/ -format UDZO batchalign3-macos-arm64.dmg
xcrun notarytool submit batchalign3-macos-arm64.dmg --key AuthKey.p8 --key-id KEY_ID --issuer ISSUER --wait
xcrun stapler staple batchalign3-macos-arm64.dmg
```

For CLI tools distributed via `uv tool install` (PyPI wheel), macOS doesn't check Gatekeeper on binaries installed this way — signing may not be strictly necessary for the wheel distribution path, but is still recommended.

### Entitlements for Rust

Rust binaries don't need JIT entitlements. A minimal entitlements file for hardened runtime:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
  <dict/>
</plist>
```

Or simply omit `--entitlements` — `--options runtime` alone enables hardened runtime.

## Windows Code Signing

### Why It Matters

Windows SmartScreen blocks unsigned executables. Users see a scary "Windows protected your PC" dialog and must click "More info" → "Run anyway." This is a dealbreaker for non-technical researchers.

### Certificate Options

| Option | Cost | Trust level | Notes |
|--------|------|-------------|-------|
| **OV (Organization Validation) code signing cert** | ~$200-400/year | Immediate SmartScreen trust with EV; OV builds reputation over time | Standard for open-source. DigiCert, Sectigo, SSL.com |
| **EV (Extended Validation) code signing cert** | ~$300-600/year | Immediate SmartScreen trust | Requires hardware token (USB) or cloud HSM. Better for installers. |
| **Azure Trusted Signing** | $9.99/month | Immediate SmartScreen trust | Microsoft's managed service. No hardware token needed. Good CI story. Requires Azure subscription + identity validation. |
| **SignPath.io** (free for OSS) | Free | Depends on cert type | CI-integrated signing service. Free tier for open-source projects. |

**Recommendation:** Azure Trusted Signing is the most CI-friendly option and reasonably priced. Alternatively, an OV cert from a CA like SSL.com works if we want something traditional.

### Signing Process (signtool)

```bash
# Using signtool (from Windows SDK)
signtool sign /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 \
    /f cert.pfx /p PASSWORD batchalign3.exe

# Using Azure Trusted Signing (via azure-code-signing-action in CI)
# or via `az code-signing sign` CLI
```

### What to Sign

- `batchalign3.exe` — the CLI binary
- Any `.dll` files we ship (PyO3 `.pyd` files in the wheel)
- Installer if we create one (`.msi` or NSIS `.exe`)

### CI Integration (GitHub Actions on Windows)

```yaml
- name: Sign binary
  uses: azure/trusted-signing-action@v0.5.0
  with:
    azure-tenant-id: ${{ secrets.AZURE_TENANT_ID }}
    azure-client-id: ${{ secrets.AZURE_CLIENT_ID }}
    azure-client-secret: ${{ secrets.AZURE_CLIENT_SECRET }}
    endpoint: https://eus.codesigning.azure.net/
    trusted-signing-account-name: ${{ secrets.AZURE_SIGNING_ACCOUNT }}
    certificate-profile-name: ${{ secrets.AZURE_CERT_PROFILE }}
    files-folder: target/release/
    files-folder-filter: exe
```

## Linux Distribution

Linux doesn't have OS-level code signing enforcement like macOS/Windows. However, there are still distribution concerns:

### Package Formats to Consider

| Format | Audience | Effort |
|--------|----------|--------|
| **Standalone binary** (tarball) | All distros, advanced users | Minimal — just `cargo build --release` and tar it |
| **AppImage** | Desktop Linux users | Low — single file, no install |
| **Debian `.deb`** | Ubuntu/Debian users | Medium — `cargo-deb` crate |
| **RPM** | Fedora/RHEL users | Medium — `cargo-generate-rpm` crate |
| **Homebrew formula** | macOS + Linux brew users | Low — tap formula |

**Recommendation:** Start with standalone tarball + Homebrew formula. Add `.deb`/`.rpm` later if there's demand. AppImage is good for desktop tools (Chatter) but less relevant for CLI tools.

### GPG Signing for Linux

Sign release tarballs with GPG so users can verify authenticity:

```bash
gpg --armor --detach-sign batchalign3-linux-x86_64.tar.gz
# produces batchalign3-linux-x86_64.tar.gz.asc
```

Publish the public key and include verification instructions in release notes.

## Distribution Matrix

Summary of what each project needs per platform:

| Project | macOS | Windows | Linux | Primary install method |
|---------|-------|---------|-------|----------------------|
| **batchalign3** (CLI) | codesign + notarize for standalone downloads | signtool/Azure | GPG-signed tarball | `uv tool install batchalign3` (planned PyPI path) |
| **batchalign3** (wheel) | Not strictly needed | Sign `.pyd` | Not needed | `uv pip install batchalign3` |
| **talkbank-tools CLIs** (`chatter`, `talkbank-lsp-server`) | codesign + notarize | signtool/Azure | GPG-signed tarball | Homebrew tap or standalone download |
| **java-chatter** | codesign + notarize (current) | jpackage + signtool | N/A | DMG download |

## Priority and Next Steps

### Immediate (needed for public release)

1. **Export the Developer ID certificate as .p12** from the current machine's keychain and store securely (1Password or equivalent)
2. **Create an App Store Connect API key** for notarization (replaces the keychain profile dependency)
3. **Change `talkbank-tools/.github/workflows/release.yml` macOS packaging** from `.tar.gz` to a notarizable `.zip` or `.dmg`
4. **Add macOS signing + notarization to the `talkbank-tools` public CLI release flow** for `chatter` and `talkbank-lsp-server`
5. **Decide whether we want standalone macOS `batchalign3` archives before PyPI resumes**; if yes, give them the same codesign + notarize path
6. **Test the signed artifacts** on a clean Mac (one that has never run the unsigned version)

### Short-term

7. **Set up Windows code signing** — evaluate Azure Trusted Signing vs traditional OV cert
8. **Add signing to CI** for both macOS and Windows release builds
9. **GPG key for Linux releases** — create a TalkBank project GPG key, publish the public key

### Nice-to-have

10. **Homebrew tap** for `chatter` / `talkbank-lsp-server` and `batchalign3`
11. **`.deb`/`.rpm` packages** if researchers report trouble with standalone binaries
12. **Automated release workflow** that builds + signs + notarizes + publishes for all platforms

## Reference: Codesign Flags

| Flag | Meaning |
|------|---------|
| `--force` | Replace existing signature |
| `--timestamp` | Include a secure timestamp (required for notarization) |
| `--options runtime` | Enable hardened runtime (required for notarization) |
| `--sign "Developer ID Application: ..."` | Sign with this identity |
| `--entitlements file.plist` | Grant specific capabilities (JIT, network, etc.) |
| `--deep` | **Do not use** — signs nested code incorrectly; sign each component individually |

## Reference: Notarytool Commands

```bash
# Store credentials (one-time, per-machine — prefer API key for CI)
xcrun notarytool store-credentials "notarytool-password" \
    --apple-id YOUR_ID --team-id 45EEEGL6UX --password APP_SPECIFIC_PASSWORD

# Submit and wait
xcrun notarytool submit app.dmg --keychain-profile "notarytool-password" --wait

# Submit with API key (CI-friendly, no keychain needed)
xcrun notarytool submit app.dmg --key AuthKey.p8 --key-id KEY_ID --issuer ISSUER --wait

# Check status of past submission
xcrun notarytool log SUBMISSION_UUID --keychain-profile "notarytool-password"

# Staple the ticket (allows offline Gatekeeper check)
xcrun stapler staple app.dmg

# Verify
spctl --assess --type open --context context:primary-signature -vv app.dmg
codesign -dv --verbose=4 /path/to/binary
```
