#!/usr/bin/env bash
# Deploy batchalign3 to fleet machines.
#
# Builds the wheel once, then deploys to any combination of server and client
# machines. The server host (net) gets its daemon started and health-checked;
# client machines get the wheel installed only.
#
# Usage:
#   bash scripts/deploy_batchalign3.sh               # all fleet (server + clients)
#   bash scripts/deploy_batchalign3.sh --server      # net only
#   bash scripts/deploy_batchalign3.sh --clients     # clients only (not net)
#   bash scripts/deploy_batchalign3.sh bilbo brian   # specific hosts
#   bash scripts/deploy_batchalign3.sh --port 8000   # take over production port
#   bash scripts/deploy_batchalign3.sh --no-build    # skip wheel build
#   bash scripts/deploy_batchalign3.sh --dry-run     # show plan only

set -euo pipefail

DEFAULT_SSH_USER="macw"
SERVER_HOST="net"
CLIENT_HOSTS=(study bilbo brian davida frodo andrew lilly sue vaishnavi chen@ming)
ALL_HOSTS=("$SERVER_HOST" "${CLIENT_HOSTS[@]}")
PORT=8001  # Coexistence: batchalign-next keeps 8000

WORKSPACE_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BA3_REPO="$WORKSPACE_ROOT/batchalign3"

DRY_RUN=false
NO_BUILD=false

HOSTS=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run)   DRY_RUN=true; shift ;;
        --no-build)  NO_BUILD=true; shift ;;
        --server)    HOSTS=("$SERVER_HOST"); shift ;;
        --clients)   HOSTS=("${CLIENT_HOSTS[@]}"); shift ;;
        --all)       HOSTS=("${ALL_HOSTS[@]}"); shift ;;
        --port)      PORT="$2"; shift 2 ;;
        --help|-h)
            echo "Usage: bash scripts/deploy_batchalign3.sh [OPTIONS] [HOST...]"
            echo ""
            echo "Options:"
            echo "  --server      Deploy to server only (net)"
            echo "  --clients     Deploy to all clients (not net)"
            echo "  --all         Deploy to all fleet machines"
            echo "  --port PORT   Server port (default: 8001 for coexistence)"
            echo "  --no-build    Skip wheel build"
            echo "  --dry-run     Show plan only"
            echo ""
            echo "Default: all fleet machines"
            exit 0
            ;;
        *) HOSTS+=("$1"); shift ;;
    esac
done

if [[ ${#HOSTS[@]} -eq 0 ]]; then
    HOSTS=("${ALL_HOSTS[@]}")
fi

if [[ ! -f "$BA3_REPO/pyproject.toml" ]]; then
    echo "ERROR: batchalign3 repo not found at $BA3_REPO"
    exit 1
fi

# --- Build wheel (once) ---

if ! $NO_BUILD; then
    # Dashboard is embedded in the binary at compile time, so build it first
    echo "=== Building dashboard + wheel ==="
    (cd "$BA3_REPO" && make build-dashboard && uv build --wheel --out-dir dist/ 2>&1 | tail -3)
    echo ""
fi

WHEEL="$(ls -t "$BA3_REPO"/dist/batchalign3-*.whl 2>/dev/null | head -1)"

if [[ -z "$WHEEL" || ! -f "$WHEEL" ]]; then
    echo "ERROR: No wheel found in $BA3_REPO/dist/"
    echo "Run without --no-build to build a fresh wheel."
    exit 1
fi

WHEEL_NAME="$(basename "$WHEEL")"

echo "Deploying batchalign3 to: ${HOSTS[*]}  (port $PORT)"
echo "  wheel: $WHEEL_NAME"
echo ""

if $DRY_RUN; then
    echo "=== DRY RUN — no changes made ==="
    for entry in "${HOSTS[@]}"; do
        host="${entry#*@}"
        if [[ "$host" == "$SERVER_HOST" ]]; then
            echo "  $entry  [server — install + start daemon + health check]"
        else
            echo "  $entry  [client — install only]"
        fi
    done
    exit 0
fi

# --- Deploy ---

FAILED=()
OK=()

resolve_ssh() {
    local entry="$1"
    if [[ "$entry" == *@* ]]; then
        echo "${entry%%@*}" "${entry#*@}"
    else
        echo "$DEFAULT_SSH_USER" "$entry"
    fi
}

# Install wheel on a single host (no daemon start, no health check).
install_host() {
    local entry="$1"
    local ssh_user host
    read -r ssh_user host <<< "$(resolve_ssh "$entry")"

    local is_server=false
    [[ "$host" == "$SERVER_HOST" ]] && is_server=true
    local role_label; $is_server && role_label="[server]" || role_label="[client]"

    echo ""
    echo "=== $ssh_user@$host $role_label ==="

    if ! ssh -o ConnectTimeout=5 "$ssh_user@$host" true 2>/dev/null; then
        echo "  SKIP: cannot connect"
        return 1
    fi

    scp -q "$WHEEL" "$ssh_user@$host:/tmp/$WHEEL_NAME"

    # Stop batchalign3 (leave batchalign-next alone)
    ssh "$ssh_user@$host" '
        batchalign3 serve stop 2>/dev/null || true
        sleep 1
        pkill -9 -f "batchalign3 serve" 2>/dev/null || true
    ' 2>/dev/null || true

    echo "  Installing (with HK engines)..."
    ssh "$ssh_user@$host" "
        uv tool uninstall batchalign3 2>/dev/null || true
        uv tool install --python 3.12 --force-reinstall '/tmp/${WHEEL_NAME}[hk]' 2>&1
    " | tail -3

    local ver
    ver=$(ssh "$ssh_user@$host" 'batchalign3 version 2>&1' || echo "FAILED")
    echo "  Version: $ver"

    if [[ "$ver" == *"FAILED"* ]]; then
        echo "  ERROR: batchalign3 not working after install"
        return 1
    fi

    # Write server.yaml
    if $is_server; then
        ssh "$ssh_user@$host" "
            mkdir -p \$HOME/.batchalign3
            CONFIG=\$HOME/.batchalign3/server.yaml
            if [ ! -f \"\$CONFIG\" ]; then
                cat > \"\$CONFIG\" <<YAML
# Auto-generated by deploy_batchalign3.sh
default_lang: eng
port: $PORT
warmup: true
job_ttl_days: 7
max_concurrent_jobs: 0
YAML
            else
                if grep -q '^port:' \"\$CONFIG\"; then
                    sed -i '' 's/^port:.*/port: $PORT/' \"\$CONFIG\"
                else
                    echo 'port: $PORT' >> \"\$CONFIG\"
                fi
            fi
        "
    else
        ssh "$ssh_user@$host" "
            mkdir -p \$HOME/.batchalign3
            CONFIG=\$HOME/.batchalign3/server.yaml
            if [ ! -f \"\$CONFIG\" ]; then
                cat > \"\$CONFIG\" <<YAML
# Auto-generated by deploy_batchalign3.sh
default_lang: eng
port: $PORT
YAML
            else
                if grep -q '^port:' \"\$CONFIG\"; then
                    sed -i '' 's/^port:.*/port: $PORT/' \"\$CONFIG\"
                else
                    echo 'port: $PORT' >> \"\$CONFIG\"
                fi
            fi
        "
    fi

    ssh "$ssh_user@$host" "rm -f /tmp/$WHEEL_NAME" 2>/dev/null || true
    return 0
}

# Wait for the server daemon to become healthy (daemon already started in Phase 1).
health_check_server() {
    local entry="$1"
    local ssh_user host
    read -r ssh_user host <<< "$(resolve_ssh "$entry")"

    echo ""
    echo "=== $ssh_user@$host [server health check] ==="
    echo "  Waiting for health..."
    local elapsed=0
    local health=""
    while [ "$elapsed" -lt 180 ]; do
        sleep 5
        elapsed=$((elapsed + 5))
        health=$(ssh "$ssh_user@$host" "curl -s http://localhost:$PORT/health 2>/dev/null" || echo '{}')
        if echo "$health" | grep -q '"status":"ok"'; then
            echo "  OK: healthy after ${elapsed}s"
            return 0
        fi
        printf "    %ds/180s...\r" "$elapsed"
    done

    echo "  WARNING: health check timed out"
    echo "  Logs: ssh $ssh_user@$host 'tail -50 ~/.batchalign3/server.log'"
    return 1
}

# --- Phase 1: Install server (if in target list) ---
# Start daemon immediately so it warms up while clients deploy.

SERVER_ENTRY=""
CLIENT_ENTRIES=()
for entry in "${HOSTS[@]}"; do
    host="${entry#*@}"
    if [[ "$host" == "$SERVER_HOST" ]]; then
        SERVER_ENTRY="$entry"
    else
        CLIENT_ENTRIES+=("$entry")
    fi
done

if [[ -n "$SERVER_ENTRY" ]]; then
    if install_host "$SERVER_ENTRY"; then
        OK+=("$SERVER_ENTRY")
        # Start daemon now — it warms up while clients install in parallel
        echo ""
        echo "  Starting server daemon (warming up while clients deploy)..."
        read -r ssh_user host <<< "$(resolve_ssh "$SERVER_ENTRY")"
        ssh "$ssh_user@$host" 'batchalign3 serve start'
    else
        FAILED+=("$SERVER_ENTRY")
    fi
fi

# --- Phase 2: Install all clients in parallel ---

CLIENT_PIDS=()
CLIENT_LOGS=()

for entry in "${CLIENT_ENTRIES[@]}"; do
    logfile=$(mktemp)
    CLIENT_LOGS+=("$logfile")
    (
        if install_host "$entry"; then
            echo "OK" > "$logfile.status"
        else
            echo "FAILED" > "$logfile.status"
        fi
    ) > "$logfile" 2>&1 &
    CLIENT_PIDS+=($!)
done

# Wait for all clients and collect results
for i in "${!CLIENT_PIDS[@]}"; do
    wait "${CLIENT_PIDS[$i]}" 2>/dev/null || true
    cat "${CLIENT_LOGS[$i]}"
    entry="${CLIENT_ENTRIES[$i]}"
    status_file="${CLIENT_LOGS[$i]}.status"
    if [[ -f "$status_file" ]] && grep -q "OK" "$status_file"; then
        OK+=("$entry")
    else
        FAILED+=("$entry")
    fi
    rm -f "${CLIENT_LOGS[$i]}" "$status_file"
done

# --- Phase 3: Health-check server (it has been warming up this whole time) ---

if [[ -n "$SERVER_ENTRY" ]] && printf '%s\n' "${OK[@]}" | grep -qx "$SERVER_ENTRY"; then
    if ! health_check_server "$SERVER_ENTRY"; then
        # Move from OK to FAILED
        OK=("${OK[@]/$SERVER_ENTRY/}")
        FAILED+=("$SERVER_ENTRY")
    fi
fi

echo ""
echo "==============================="
if [[ ${#FAILED[@]} -eq 0 ]]; then
    echo "SUCCESS: ${#OK[@]} host(s) on port $PORT"
else
    echo "PARTIAL: ${#OK[@]} ok, ${#FAILED[@]} failed (${FAILED[*]})"
fi
echo "==============================="

[[ ${#FAILED[@]} -eq 0 ]]
