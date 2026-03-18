#!/usr/bin/env bash
# Create split data repos from the 4 large parent repos.
# Run ON git-talkbank in ~/staging/repos/
#
# Prerequisites:
#   - Brian and Davida have paused all pushes
#   - Parent repos are up to date (git pull)
#   - Split repos have been created on GitLab (empty)
#
# Usage:
#   ssh macw@git-talkbank
#   cd ~/staging/repos
#   bash /path/to/create-split-repos.sh [--dry-run]

set -euo pipefail

DRY_RUN=false
if [ "${1:-}" = "--dry-run" ]; then
    DRY_RUN=true
    echo "DRY RUN — no changes will be made"
fi

REPOS_DIR="${PWD}"

run() {
    echo "  \$ $*"
    if [ "$DRY_RUN" = false ]; then
        "$@"
    fi
}

init_split_repo() {
    local repo="$1"
    local parent="$2"
    shift 2
    local dirs=("$@")

    echo ""
    echo "=== Creating $repo from $parent ==="

    if [ -d "$repo/.git" ]; then
        echo "  SKIP: $repo already exists"
        return
    fi

    run mkdir -p "$repo"

    # Copy .gitignore from parent
    run cp "$parent/.gitignore" "$repo/.gitignore"

    # Copy each language-group directory
    for dir in "${dirs[@]}"; do
        if [ -d "$parent/$dir" ]; then
            echo "  Copying $dir..."
            run cp -a "$parent/$dir" "$repo/$dir"
        else
            echo "  WARNING: $parent/$dir does not exist, skipping"
        fi
    done

    # Init git repo and make initial commit
    if [ "$DRY_RUN" = false ]; then
        cd "$repo"
        git init
        git add -A
        git commit -m "Initial import: split from $parent"
        cd "$REPOS_DIR"
    fi

    echo "  Done: $repo"
}

echo "Working in: $REPOS_DIR"
echo ""

# Verify parent repos exist
for parent in childes-data ca-data phon-data homebank-data; do
    if [ ! -d "$parent/.git" ]; then
        echo "ERROR: $parent not found in $REPOS_DIR"
        exit 1
    fi
done

# Pull parent repos to make sure we have latest
echo "Pulling parent repos..."
for parent in childes-data ca-data phon-data homebank-data; do
    echo "  $parent..."
    run git -C "$parent" pull --quiet
done

# ── CHILDES: 4-way split by language group ──

init_split_repo "childes-eng-na-data" "childes-data" \
    Eng-NA Eng-AAE

init_split_repo "childes-eng-uk-data" "childes-data" \
    Eng-UK Clinical-Eng Clinical-Other

init_split_repo "childes-romance-germanic-data" "childes-data" \
    French Romance Spanish German DutchAfrikaans Scandinavian Celtic

init_split_repo "childes-other-data" "childes-data" \
    Biling Chinese EastAsian Japanese Slavic Finno-Ugric Other Frogs MAIN GlobalTales XLing

# ── CA: 2-way split (CANDOR vs everything else) ──

# ca-candor-data gets just CANDOR
init_split_repo "ca-candor-data" "ca-data" \
    CANDOR

# ca-data keeps everything else — we create a NEW ca-data (the original stays read-only)
# This is the tricky one: we need to copy everything EXCEPT CANDOR.
echo ""
echo "=== Creating ca-data (remainder, excluding CANDOR) ==="
if [ -d "ca-data-new/.git" ]; then
    echo "  SKIP: ca-data-new already exists"
else
    run mkdir -p "ca-data-new"
    run cp "ca-data/.gitignore" "ca-data-new/.gitignore"

    # Copy everything except CANDOR and .git
    for item in ca-data/*/; do
        dir_name=$(basename "$item")
        if [ "$dir_name" = ".git" ] || [ "$dir_name" = "CANDOR" ]; then
            continue
        fi
        echo "  Copying $dir_name..."
        run cp -a "$item" "ca-data-new/$dir_name"
    done

    # Also copy any top-level files (0metadata.cdc, etc.) that aren't in subdirs
    if [ "$DRY_RUN" = false ]; then
        find ca-data -maxdepth 1 -type f ! -name ".git*" -exec cp {} ca-data-new/ \;
    fi

    if [ "$DRY_RUN" = false ]; then
        cd "ca-data-new"
        git init
        git add -A
        git commit -m "Initial import: split from ca-data (everything except CANDOR)"
        cd "$REPOS_DIR"
    fi
    echo "  Done: ca-data-new"
    echo ""
    echo "  NOTE: Rename ca-data-new → ca-data after archiving the original."
    echo "        mv ca-data ca-data-original-ARCHIVED"
    echo "        mv ca-data-new ca-data"
fi

# ── PHON: 2-way split by language ──

init_split_repo "phon-eng-french-data" "phon-data" \
    Eng-NA French

# phon-other-data gets everything except Eng-NA, French
echo ""
echo "=== Creating phon-other-data (everything except Eng-NA, French) ==="
if [ -d "phon-other-data/.git" ]; then
    echo "  SKIP: phon-other-data already exists"
else
    run mkdir -p "phon-other-data"
    run cp "phon-data/.gitignore" "phon-other-data/.gitignore"

    for item in phon-data/*/; do
        dir_name=$(basename "$item")
        if [ "$dir_name" = ".git" ] || [ "$dir_name" = "Eng-NA" ] || [ "$dir_name" = "French" ]; then
            continue
        fi
        echo "  Copying $dir_name..."
        run cp -a "$item" "phon-other-data/$dir_name"
    done

    if [ "$DRY_RUN" = false ]; then
        find phon-data -maxdepth 1 -type f ! -name ".git*" -exec cp {} phon-other-data/ \;
        cd "phon-other-data"
        git init
        git add -A
        git commit -m "Initial import: split from phon-data (all except Eng-NA, French)"
        cd "$REPOS_DIR"
    fi
    echo "  Done: phon-other-data"
fi

# ── HOMEBANK: 4-way split by access tier ──

# homebank-public-data: Public + Secure
init_split_repo "homebank-public-data" "homebank-data" \
    Public Secure

# homebank-cougar-data: Password/Cougar only
echo ""
echo "=== Creating homebank-cougar-data ==="
if [ -d "homebank-cougar-data/.git" ]; then
    echo "  SKIP: homebank-cougar-data already exists"
else
    run mkdir -p "homebank-cougar-data/Password"
    run cp "homebank-data/.gitignore" "homebank-cougar-data/.gitignore"
    echo "  Copying Password/Cougar..."
    run cp -a "homebank-data/Password/Cougar" "homebank-cougar-data/Password/Cougar"

    if [ "$DRY_RUN" = false ]; then
        cd "homebank-cougar-data"
        git init
        git add -A
        git commit -m "Initial import: split from homebank-data (Password/Cougar)"
        cd "$REPOS_DIR"
    fi
    echo "  Done: homebank-cougar-data"
fi

# homebank-bergelson-data: Password/Bergelson only
echo ""
echo "=== Creating homebank-bergelson-data ==="
if [ -d "homebank-bergelson-data/.git" ]; then
    echo "  SKIP: homebank-bergelson-data already exists"
else
    run mkdir -p "homebank-bergelson-data/Password"
    run cp "homebank-data/.gitignore" "homebank-bergelson-data/.gitignore"
    echo "  Copying Password/Bergelson..."
    run cp -a "homebank-data/Password/Bergelson" "homebank-bergelson-data/Password/Bergelson"

    if [ "$DRY_RUN" = false ]; then
        cd "homebank-bergelson-data"
        git init
        git add -A
        git commit -m "Initial import: split from homebank-data (Password/Bergelson)"
        cd "$REPOS_DIR"
    fi
    echo "  Done: homebank-bergelson-data"
fi

# homebank-password-data: Password/ remainder (everything except Cougar and Bergelson)
echo ""
echo "=== Creating homebank-password-data (Password remainder) ==="
if [ -d "homebank-password-data/.git" ]; then
    echo "  SKIP: homebank-password-data already exists"
else
    run mkdir -p "homebank-password-data/Password"
    run cp "homebank-data/.gitignore" "homebank-password-data/.gitignore"

    for item in homebank-data/Password/*/; do
        dir_name=$(basename "$item")
        if [ "$dir_name" = "Cougar" ] || [ "$dir_name" = "Bergelson" ]; then
            continue
        fi
        echo "  Copying Password/$dir_name..."
        run cp -a "$item" "homebank-password-data/Password/$dir_name"
    done

    if [ "$DRY_RUN" = false ]; then
        cd "homebank-password-data"
        git init
        git add -A
        git commit -m "Initial import: split from homebank-data (Password remainder)"
        cd "$REPOS_DIR"
    fi
    echo "  Done: homebank-password-data"
fi

# ── Verification ──

echo ""
echo "========================================="
echo "Split repos created. Verification:"
echo "========================================="
echo ""

verify_split() {
    local parent="$1"
    shift
    local split_repos=("$@")

    if [ "$DRY_RUN" = true ]; then
        echo "  (skipped in dry-run)"
        return
    fi

    local parent_files
    parent_files=$(find "$parent" -not -path '*/.git/*' -type f | sed "s|^${parent}/||" | sort)

    local split_files
    split_files=""
    for repo in "${split_repos[@]}"; do
        local actual_repo="$repo"
        # Handle ca-data-new rename
        if [ "$repo" = "ca-data" ] && [ -d "ca-data-new" ]; then
            actual_repo="ca-data-new"
        fi
        if [ -d "$actual_repo" ]; then
            split_files+=$(find "$actual_repo" -not -path '*/.git/*' -type f | sed "s|^${actual_repo}/||")
            split_files+=$'\n'
        fi
    done

    local parent_count split_count
    parent_count=$(echo "$parent_files" | wc -l | tr -d ' ')
    split_count=$(echo "$split_files" | sort -u | grep -c . || true)

    if [ "$parent_count" = "$split_count" ]; then
        echo "  $parent: OK ($parent_count files = $split_count in splits)"
    else
        echo "  $parent: MISMATCH! $parent_count files in parent, $split_count in splits"
        echo "  Run diff to investigate:"
        echo "    diff <(find $parent -not -path '*/.git/*' -type f | sed 's|^${parent}/||' | sort) \\"
        echo "         <(for r in ${split_repos[*]}; do find \$r -not -path '*/.git/*' -type f | sed \"s|^\$r/||\"; done | sort)"
    fi
}

echo "Verifying childes-data split..."
verify_split "childes-data" \
    "childes-eng-na-data" "childes-eng-uk-data" \
    "childes-romance-germanic-data" "childes-other-data"

echo "Verifying ca-data split..."
verify_split "ca-data" "ca-candor-data" "ca-data"

echo "Verifying phon-data split..."
verify_split "phon-data" "phon-eng-french-data" "phon-other-data"

echo "Verifying homebank-data split..."
verify_split "homebank-data" \
    "homebank-public-data" "homebank-cougar-data" \
    "homebank-bergelson-data" "homebank-password-data"

echo ""
echo "Next steps:"
echo "  1. Push each split repo to GitLab"
echo "  2. For ca-data: mv ca-data ca-data-original-ARCHIVED && mv ca-data-new ca-data"
echo "  3. Update staging config and pull on git-talkbank"
echo "  4. Test deploys"
