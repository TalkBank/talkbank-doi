# Windows Code Signing — TODO

**Status:** Draft
**Last updated:** 2026-03-16

## Background

Windows SmartScreen blocks unsigned executables with a "Windows protected your
PC" dialog. Users must click "More info" → "Run anyway." This is a dealbreaker
for non-technical researchers who are our primary audience.

All TalkBank Windows binaries are currently **unsigned**:
- `CLAN.exe` (Windows-CLAN) — Leonid builds with Visual Studio 2013, no signing
- `chatter.exe` (talkbank-tools) — unsigned GitHub Release artifact
- `batchalign3.exe` — not yet distributed for Windows
- Chatter desktop app (Tauri) — not yet built for Windows

## Apps That Need Signing

| App | Binary | Distribution | Priority |
|-----|--------|-------------|----------|
| **Chatter desktop** (Tauri) | `Chatter.exe` + `.msi` | GitHub Release download | High — first desktop app for non-technical users |
| **chatter CLI** | `chatter.exe` | GitHub Release download | High — public release imminent |
| **batchalign3 CLI** | `batchalign3.exe` | PyPI wheel + standalone download | Medium — PyPI path is primary |
| **CLAN** (Windows-CLAN) | `CLAN.exe` | Manual distribution | Low — legacy, maintained by Leonid |

## Certificate Options

| Option | Cost | SmartScreen trust | CI-friendly | Notes |
|--------|------|-------------------|-------------|-------|
| **Azure Trusted Signing** | $9.99/month | Immediate | Yes (GitHub Action) | Microsoft's managed service. No hardware token. Requires Azure subscription + identity validation. **Recommended.** |
| **OV (Organization Validation) cert** | ~$200-400/year | Builds reputation over time | Yes (with `.pfx`) | Traditional CA (DigiCert, Sectigo, SSL.com). Requires legal entity verification. |
| **EV (Extended Validation) cert** | ~$300-600/year | Immediate | Harder (hardware token or cloud HSM) | Hardware USB token required for private key. Best trust, worst CI story. |
| **SignPath.io** (free for OSS) | Free | Depends on cert | Yes | CI-integrated signing service. Free tier for open-source. |

### Recommendation

**Azure Trusted Signing** is the best fit:
- $9.99/month is negligible
- Immediate SmartScreen trust (no reputation building)
- No hardware token needed
- Native GitHub Actions integration (`azure/trusted-signing-action`)
- Works for all our apps (Tauri `.msi`, standalone `.exe`, CLI binaries)

## Questions for Leonid

Ask Leonid these questions to understand the current Windows signing situation:

1. **Have you ever code-signed CLAN.exe?** If yes, what certificate did you use
   and where is it stored? (Even an expired cert tells us if CMU has a code
   signing relationship with a CA.)

2. **Does CMU have an organizational code signing certificate?** CMU's IT
   department may already have an OV or EV cert that departments can use.
   Contact CMU Software Licensing or IT Security if Leonid doesn't know.

3. **Do you have an Azure account?** If CMU already uses Azure (common for
   universities), Azure Trusted Signing may be available through the existing
   subscription.

4. **Have users complained about SmartScreen warnings?** This helps prioritize
   — if users are already working around it, signing is urgent; if Leonid
   distributes via a method that bypasses SmartScreen (e.g., internal network),
   it's less urgent for CLAN specifically.

5. **Would you like us to set up signing for CLAN.exe too?** If we get an Azure
   Trusted Signing account, we could sign all TalkBank Windows binaries through
   the same pipeline — CLAN included.

## How Windows Code Signing Works

### Step 1: Get a Certificate

**Azure Trusted Signing setup:**

1. Create Azure subscription (or use existing CMU one)
2. Go to Azure Portal → "Trusted Signing"
3. Create a Trusted Signing account
4. Complete identity validation (organization name, address, EIN/DUNS)
5. Create a certificate profile (type: "Public Trust")
6. Note: Account Name, Certificate Profile Name, Endpoint URL

**Traditional CA (OV cert) setup:**

1. Purchase from DigiCert, Sectigo, or SSL.com
2. Generate CSR with organizational details
3. Complete domain/org validation (phone call, legal docs)
4. Receive `.pfx` file with cert + private key

### Step 2: Sign Binaries

**Using Azure Trusted Signing (recommended):**

```powershell
# Install the signing tool
dotnet tool install --global Azure.CodeSigning.Dlib

# Sign a single binary
azure-code-signing sign ^
    --azure-tenant-id TENANT_ID ^
    --azure-client-id CLIENT_ID ^
    --azure-client-secret CLIENT_SECRET ^
    --endpoint https://eus.codesigning.azure.net/ ^
    --trusted-signing-account-name ACCOUNT_NAME ^
    --certificate-profile-name PROFILE_NAME ^
    --files Chatter.exe

# Sign with signtool (Windows SDK) + Azure Trusted Signing dlib
signtool sign /v /debug /fd SHA256 /tr http://timestamp.acs.microsoft.com /td SHA256 ^
    /dlib "Azure.CodeSigning.Dlib.dll" ^
    /dmdf metadata.json ^
    Chatter.exe
```

**Using signtool with `.pfx` (traditional):**

```powershell
# Sign with timestamp (important — signature survives cert expiry)
signtool sign /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 ^
    /f cert.pfx /p PASSWORD ^
    Chatter.exe

# Verify
signtool verify /pa Chatter.exe
```

### Step 3: CI Integration (GitHub Actions)

**Azure Trusted Signing:**

```yaml
- name: Sign Windows binary
  uses: azure/trusted-signing-action@v0.5.0
  with:
    azure-tenant-id: ${{ secrets.AZURE_TENANT_ID }}
    azure-client-id: ${{ secrets.AZURE_CLIENT_ID }}
    azure-client-secret: ${{ secrets.AZURE_CLIENT_SECRET }}
    endpoint: https://eus.codesigning.azure.net/
    trusted-signing-account-name: ${{ secrets.AZURE_SIGNING_ACCOUNT }}
    certificate-profile-name: ${{ secrets.AZURE_CERT_PROFILE }}
    files-folder: target/release/
    files-folder-filter: exe,msi
```

**Traditional `.pfx`:**

```yaml
- name: Import certificate
  run: |
    $bytes = [System.Convert]::FromBase64String("${{ secrets.WINDOWS_CERT_PFX }}")
    [System.IO.File]::WriteAllBytes("cert.pfx", $bytes)

- name: Sign binary
  run: |
    & "C:\Program Files (x86)\Windows Kits\10\bin\x64\signtool.exe" sign `
      /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 `
      /f cert.pfx /p "${{ secrets.WINDOWS_CERT_PASSWORD }}" `
      target/release/chatter.exe
```

### What to Sign

| Artifact | Tool | Notes |
|----------|------|-------|
| `.exe` binaries | signtool | Both CLI and GUI binaries |
| `.msi` installers | signtool | Tauri builds these for Windows |
| `.dll` / `.pyd` files | signtool | PyO3 native extensions in wheels |
| NSIS `.exe` installers | signtool | If we use NSIS instead of MSI |

### GitHub Actions Secrets Needed

| Secret | What | Source |
|--------|------|--------|
| `AZURE_TENANT_ID` | Azure AD tenant | Azure Portal |
| `AZURE_CLIENT_ID` | Service principal app ID | Azure Portal |
| `AZURE_CLIENT_SECRET` | Service principal secret | Azure Portal |
| `AZURE_SIGNING_ACCOUNT` | Trusted Signing account name | Azure Portal |
| `AZURE_CERT_PROFILE` | Certificate profile name | Azure Portal |

Or for traditional `.pfx`:

| Secret | What | Source |
|--------|------|--------|
| `WINDOWS_CERT_PFX` | Base64-encoded `.pfx` | `base64 < cert.pfx` |
| `WINDOWS_CERT_PASSWORD` | PFX passphrase | Set during export |

## Next Steps

1. **Ask Leonid** the questions above
2. **Check with CMU IT** about existing code signing infrastructure
3. **Set up Azure Trusted Signing** (or choose alternative based on findings)
4. **Add signing to CI** for talkbank-tools and batchalign3 release workflows
5. **Test on a clean Windows machine** — verify SmartScreen accepts the signed binary
