#!/usr/bin/env bash
# Setup script for TalkBank data workspace.
# Clones data repos and installs pre-commit tooling.
#
# Usage:
#   ./setup-data-workspace.sh [TARGET_DIR] [REPO...]
#
# Examples:
#   ./setup-data-workspace.sh                    # Clone ALL 24 repos to ~/0data
#   ./setup-data-workspace.sh ~/0data            # Clone ALL 24 repos to ~/0data
#   ./setup-data-workspace.sh ~/0data aphasia-data dementia-data  # Clone only these
#   ./setup-data-workspace.sh ~/0data --bank aphasia dementia     # Clone by bank name
#
# If repos already exist, they are skipped (not overwritten).
# Tooling (update-chat-types) is always installed regardless of which repos.

set -euo pipefail

TARGET="${1:-$HOME/0data}"
shift 2>/dev/null || true

# All 24 data repos (12 unsplit + 12 from splits)
REPOS=(
    # Unsplit (1:1 with bank)
    aphasia-data
    asd-data
    biling-data
    class-data
    dementia-data
    fluency-data
    motor-data
    psychosis-data
    rhd-data
    samtale-data
    slabank-data
    tbi-data

    # childes (4-way split by language group)
    childes-eng-na-data        # Eng-NA, Eng-AAE
    childes-eng-uk-data        # Eng-UK, Clinical-Eng, Clinical-Other
    childes-romance-germanic-data  # French, Romance, Spanish, German, DutchAfrikaans, Scandinavian, Celtic
    childes-other-data         # Biling, Chinese, EastAsian, Japanese, Slavic, Finno-Ugric, Other, Frogs, MAIN, GlobalTales, XLing

    # ca (2-way split: CANDOR vs everything else)
    ca-candor-data             # CANDOR only (4.8 GB)
    ca-data                    # Everything else (40+ corpora)

    # phon (2-way split by language)
    phon-eng-french-data       # Eng-NA, French
    phon-other-data            # All other languages

    # homebank (4-way split by access tier)
    homebank-public-data       # Public + Secure
    homebank-cougar-data       # Password/Cougar
    homebank-bergelson-data    # Password/Bergelson
    homebank-password-data     # Password/ remainder
)

GITHUB_ORG="TalkBank"
TOOLING_REPO="TalkBank/update-chat-types"

# ── Bank name → repo name mapping (for --bank shorthand) ──
declare -A BANK_REPOS=(
    [aphasia]="aphasia-data"
    [asd]="asd-data"
    [biling]="biling-data"
    [ca]="ca-candor-data ca-data"
    [childes]="childes-eng-na-data childes-eng-uk-data childes-romance-germanic-data childes-other-data"
    [class]="class-data"
    [dementia]="dementia-data"
    [fluency]="fluency-data"
    [homebank]="homebank-public-data homebank-cougar-data homebank-bergelson-data homebank-password-data"
    [motor]="motor-data"
    [phon]="phon-eng-french-data phon-other-data"
    [psychosis]="psychosis-data"
    [rhd]="rhd-data"
    [samtale]="samtale-data"
    [slabank]="slabank-data"
    [tbi]="tbi-data"
)

# ── Parse arguments: select which repos to clone ──
SELECTED_REPOS=()
BANK_MODE=false

for arg in "$@"; do
    if [ "$arg" = "--bank" ]; then
        BANK_MODE=true
        continue
    fi
    if [ "$BANK_MODE" = true ]; then
        if [ -n "${BANK_REPOS[$arg]+x}" ]; then
            for repo in ${BANK_REPOS[$arg]}; do
                SELECTED_REPOS+=("$repo")
            done
        else
            echo "Unknown bank: $arg" >&2
            echo "Available banks: ${!BANK_REPOS[*]}" >&2
            exit 1
        fi
    else
        # Direct repo name
        SELECTED_REPOS+=("$arg")
    fi
done

# If no repos specified, clone all
if [ ${#SELECTED_REPOS[@]} -eq 0 ]; then
    SELECTED_REPOS=("${REPOS[@]}")
fi

# ── Detect platform for binary download ──
detect_platform() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"
    case "$os-$arch" in
        Darwin-arm64)  echo "macos-arm64" ;;
        Linux-x86_64)  echo "linux-x86_64" ;;
        *)
            echo "unknown"
            echo "WARNING: No pre-built binary for $os-$arch. Install Rust and build from source." >&2
            ;;
    esac
}

# ── Install update-chat-types from GitHub Releases ──
install_tooling() {
    local platform="$1"

    # Check if already installed and up to date
    if command -v update-chat-types >/dev/null 2>&1; then
        echo "update-chat-types already installed: $(which update-chat-types)"
        echo "  (To upgrade, remove it and re-run this script)"
        return 0
    fi

    if [ "$platform" = "unknown" ]; then
        echo "Skipping tooling install (unsupported platform)"
        return 0
    fi

    echo "Installing update-chat-types..."

    # Get latest release download URL
    local artifact="update-chat-types-${platform}"
    local download_url
    download_url=$(curl -sL "https://api.github.com/repos/${TOOLING_REPO}/releases/latest" \
        | grep "browser_download_url.*${artifact}" \
        | head -1 \
        | cut -d '"' -f 4) || true

    if [ -z "$download_url" ]; then
        echo "WARNING: No release binary found. Trying to build from source..."
        install_from_source
        return $?
    fi

    # Download binary
    local tmpbin
    tmpbin=$(mktemp)
    echo "  Downloading from $download_url..."
    if curl -sL "$download_url" -o "$tmpbin"; then
        chmod +x "$tmpbin"
        run_bootstrap "$tmpbin"
        rm -f "$tmpbin"
    else
        echo "WARNING: Download failed. Trying to build from source..."
        rm -f "$tmpbin"
        install_from_source
    fi
}

install_from_source() {
    if ! command -v cargo >/dev/null 2>&1; then
        echo "WARNING: Rust not installed. Skipping update-chat-types."
        echo "  Install Rust: https://rustup.rs/"
        echo "  Then re-run this script."
        return 1
    fi

    local tmpdir
    tmpdir=$(mktemp -d)
    echo "  Cloning and building from source..."
    if git clone --depth 1 "https://github.com/${TOOLING_REPO}.git" "$tmpdir/update-chat-types" 2>/dev/null; then
        (cd "$tmpdir/update-chat-types" && cargo build --release 2>&1 | tail -1)
        run_bootstrap "$tmpdir/update-chat-types/target/release/update-chat-types"
    else
        echo "WARNING: Could not clone ${TOOLING_REPO}. Skipping tooling install."
    fi
    rm -rf "$tmpdir"
}

run_bootstrap() {
    local binary="$1"
    # Download bootstrap.sh from the repo
    local bootstrap
    bootstrap=$(mktemp)
    curl -sL "https://raw.githubusercontent.com/${TOOLING_REPO}/main/bootstrap.sh" -o "$bootstrap"
    if [ -s "$bootstrap" ]; then
        bash "$bootstrap" --binary "$binary"
    else
        # Fallback: manual install
        mkdir -p "$HOME/.talkbank/bin"
        cp "$binary" "$HOME/.talkbank/bin/update-chat-types"
        chmod +x "$HOME/.talkbank/bin/update-chat-types"
        echo "  Installed binary to ~/.talkbank/bin/update-chat-types"
        echo "  NOTE: Run bootstrap.sh manually to set up the pre-commit hook."
    fi
    rm -f "$bootstrap"
}

# ── Main ──

echo "Setting up TalkBank data workspace at: $TARGET"
echo "Repos to clone: ${#SELECTED_REPOS[@]}"
echo ""

# Install tooling first
PLATFORM=$(detect_platform)
install_tooling "$PLATFORM"
echo ""

# Clone repos
mkdir -p "$TARGET"

cloned=0
skipped=0
failed=0

for repo in "${SELECTED_REPOS[@]}"; do
    if [ -d "$TARGET/$repo/.git" ]; then
        echo "  skip  $repo (already exists)"
        skipped=$((skipped + 1))
    else
        echo "  clone $repo..."
        if git clone "git@github.com:${GITHUB_ORG}/${repo}.git" "$TARGET/$repo" 2>/dev/null; then
            cloned=$((cloned + 1))
        else
            echo "  FAILED to clone $repo"
            failed=$((failed + 1))
        fi
    fi
done

echo ""
echo "Done. Cloned: $cloned, Skipped: $skipped, Failed: $failed"
echo "Workspace: $TARGET"

if [ "$failed" -gt 0 ]; then
    echo ""
    echo "WARNING: $failed repos failed to clone. Check SSH key access to github.com."
    exit 1
fi
