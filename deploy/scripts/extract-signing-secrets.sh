#!/usr/bin/env bash
# extract-signing-secrets.sh
#
# Run this on the dev machine that has the Developer ID certificate and
# notarytool-password keychain profile. Extracts everything needed to set up
# portable, CI-ready signing and notarization.
#
# Usage: bash deploy/scripts/extract-signing-secrets.sh
#
# Outputs values for these GitHub Actions secrets:
#   APPLE_CERT_P12_BASE64        base64-encoded .p12 certificate
#   APPLE_CERT_P12_PASSWORD      passphrase used during export
#   APPLE_TEAM_ID                45EEEGL6UX (hardcoded)
#   APPLE_ID                     extracted from notarytool-password keychain item
#   APPLE_APP_SPECIFIC_PASSWORD  extracted from notarytool-password keychain item
#   APPLE_API_KEY_P8             manual — download from App Store Connect
#   APPLE_API_KEY_ID             manual — from App Store Connect
#   APPLE_API_ISSUER             manual — from App Store Connect

set -euo pipefail

CERT_CN="Developer ID Application"
KEYCHAIN="$HOME/Library/Keychains/login.keychain-db"
NOTARYTOOL_PROFILE="notarytool-password"
TEAM_ID="45EEEGL6UX"
P12_TMP="/tmp/developer-id-application.p12"

# Colours
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
RESET='\033[0m'

err()  { echo -e "${RED}ERROR:${RESET} $*" >&2; }
ok()   { echo -e "${GREEN}✓${RESET} $*"; }
warn() { echo -e "${YELLOW}WARNING:${RESET} $*"; }
hdr()  { echo -e "\n${BOLD}=== $* ===${RESET}"; }

# ── 1. Verify Developer ID cert ───────────────────────────────────────────────
hdr "Step 1: Verify Developer ID Application certificate"

CERT_LINE=$(security find-identity -v -p codesigning | grep "$CERT_CN" || true)
if [[ -z "$CERT_LINE" ]]; then
    err "No '$CERT_CN' certificate found in the keychain."
    echo "Make sure you are running this script on the correct machine."
    echo "Expected identity: Developer ID Application: Brian MacWhinney (45EEEGL6UX)"
    exit 1
fi
ok "Found: $CERT_LINE"

# ── 2. Generate passphrase ────────────────────────────────────────────────────
hdr "Step 2: Generate .p12 export passphrase"

P12_PASSPHRASE=$(openssl rand -base64 32)
ok "Generated passphrase (save this — you will need it as APPLE_CERT_P12_PASSWORD)"

# ── 3. Export .p12 ───────────────────────────────────────────────────────────
hdr "Step 3: Export .p12 to $P12_TMP"

# Remove leftover from a previous run
rm -f "$P12_TMP"

if security export \
    -k "$KEYCHAIN" \
    -t identities \
    -f pkcs12 \
    -P "$P12_PASSPHRASE" \
    -o "$P12_TMP" 2>/dev/null; then
    ok "Exported to $P12_TMP"
else
    warn "security export failed (cert may be in a different keychain)."
    echo ""
    echo "Fallback: export manually via Keychain Access:"
    echo "  1. Open Keychain Access (Spotlight → 'Keychain Access')"
    echo "  2. Find 'Developer ID Application: Brian MacWhinney (45EEEGL6UX)'"
    echo "  3. Right-click → Export → save as 'developer-id-application.p12'"
    echo "  4. Set passphrase to: $P12_PASSPHRASE"
    echo "  5. Move the file to: $P12_TMP"
    echo ""
    echo "Then re-run this script — it will skip the export step if $P12_TMP already exists."
    exit 1
fi

if [[ ! -f "$P12_TMP" ]]; then
    err "$P12_TMP was not created."
    exit 1
fi

# ── 4. Encode .p12 as base64 ──────────────────────────────────────────────────
hdr "Step 4: Encode .p12 as base64"

CERT_BASE64=$(base64 < "$P12_TMP")
ok "Encoded (${#CERT_BASE64} characters)"

# ── 5. Extract notarytool credentials ─────────────────────────────────────────
hdr "Step 5: Extract notarytool-password keychain credentials"

APPLE_ID=""
APPLE_APP_PASS=""

# acct attribute holds the Apple ID
APPLE_ID=$(security find-generic-password -s "$NOTARYTOOL_PROFILE" -g 2>&1 \
    | grep '"acct"' \
    | sed 's/.*"acct"<blob>="//' \
    | sed 's/"$//' \
    || true)

# -w prints the password (app-specific password) to stdout
APPLE_APP_PASS=$(security find-generic-password -s "$NOTARYTOOL_PROFILE" -w 2>/dev/null || true)

if [[ -z "$APPLE_ID" ]]; then
    warn "Could not extract Apple ID from '$NOTARYTOOL_PROFILE' keychain item."
    APPLE_ID="<not found — check Keychain Access manually>"
fi

if [[ -z "$APPLE_APP_PASS" ]]; then
    warn "Could not extract app-specific password from '$NOTARYTOOL_PROFILE' keychain item."
    APPLE_APP_PASS="<not found — check Keychain Access manually>"
fi

ok "Apple ID: $APPLE_ID"
ok "App-specific password: ${APPLE_APP_PASS:0:4}****"

# ── 6. Print summary ──────────────────────────────────────────────────────────
hdr "GitHub Actions Secrets — copy these values"

echo ""
echo "────────────────────────────────────────────────────────────────"
echo ""
echo "APPLE_CERT_P12_BASE64:"
echo "$CERT_BASE64"
echo ""
echo "────────────────────────────────────────────────────────────────"
echo ""
echo "APPLE_CERT_P12_PASSWORD:"
echo "$P12_PASSPHRASE"
echo ""
echo "────────────────────────────────────────────────────────────────"
echo ""
echo "APPLE_TEAM_ID:"
echo "$TEAM_ID"
echo ""
echo "────────────────────────────────────────────────────────────────"
echo ""
echo "APPLE_ID:"
echo "$APPLE_ID"
echo ""
echo "────────────────────────────────────────────────────────────────"
echo ""
echo "APPLE_APP_SPECIFIC_PASSWORD:"
echo "$APPLE_APP_PASS"
echo ""
echo "────────────────────────────────────────────────────────────────"
echo ""
echo "APPLE_API_KEY_P8 / APPLE_API_KEY_ID / APPLE_API_ISSUER:"
echo "  → Manual step. See next steps below."
echo ""
echo "────────────────────────────────────────────────────────────────"

# ── 7. Clean up ───────────────────────────────────────────────────────────────
hdr "Step 7: Clean up temp file"

rm -f "$P12_TMP"
ok "Deleted $P12_TMP"

# ── Next steps ────────────────────────────────────────────────────────────────
hdr "Next Steps"

cat <<'EOF'
1. STORE IN 1PASSWORD
   Create a secure note or login item with all 5 auto-extracted values above
   (APPLE_CERT_P12_BASE64, APPLE_CERT_P12_PASSWORD, APPLE_TEAM_ID,
   APPLE_ID, APPLE_APP_SPECIFIC_PASSWORD).

2. ADD AS GITHUB ACTIONS SECRETS
   Go to: Settings → Secrets and variables → Actions → New repository secret
   Repos: talkbank-tools, batchalign3 (or use an org-level secret if available)
   Add each value from the output above.

3. CREATE APP STORE CONNECT API KEY (recommended for CI notarization)
   The API key avoids needing APPLE_ID + APPLE_APP_SPECIFIC_PASSWORD in CI.
   Browser-only, requires Account Holder role:
     a. Go to: https://appstoreconnect.apple.com/access/integrations/api
     b. Click "+" → name it "TalkBank CI", role: Developer
     c. Download the .p8 file (ONLY downloadable once!)
     d. Note the Key ID (10-character alphanumeric string)
     e. Note the Issuer ID (UUID shown at the top of the page)
     f. Store .p8 contents as APPLE_API_KEY_P8 (cat AuthKey_XXX.p8 | pbcopy)
     g. Store key ID as APPLE_API_KEY_ID
     h. Store issuer UUID as APPLE_API_ISSUER

4. VERIFY THE IMPORT WORKS (optional, on a fresh machine or CI runner)
   echo "$APPLE_CERT_P12_BASE64" | base64 --decode > /tmp/test-cert.p12
   security import /tmp/test-cert.p12 -P "$APPLE_CERT_P12_PASSWORD" -T /usr/bin/codesign
   rm /tmp/test-cert.p12

EOF
