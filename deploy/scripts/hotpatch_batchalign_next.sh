#!/bin/bash
set -e

SRC="$HOME/batchalign-next/batchalign"
PATCHED_DISPATCH="/tmp/dispatch_patched.py"

deploy_remote() {
    local host="$1"
    local user="${2:-macw}"
    local site
    site=$(ssh -o ConnectTimeout=5 -o BatchMode=yes "${user}@${host}" \
        "find ~/.local/share/uv/tools/batchalign-next -path '*/batchalign/cli/dispatch.py' 2>/dev/null | head -1" 2>/dev/null)
    
    if [ -z "$site" ]; then
        echo "SKIP $host: batchalign-next not found"
        return
    fi
    
    local base
    base=$(echo "$site" | sed 's|/batchalign/cli/dispatch.py||')
    
    # Copy files
    scp -q "$PATCHED_DISPATCH"                          "${user}@${host}:${base}/batchalign/cli/dispatch.py"
    scp -q "$SRC/cli/dispatch_local.py"                 "${user}@${host}:${base}/batchalign/cli/dispatch_local.py"
    scp -q "$SRC/cli/dispatch_common.py"                "${user}@${host}:${base}/batchalign/cli/dispatch_common.py"
    scp -q "$SRC/cli/file_io.py"                        "${user}@${host}:${base}/batchalign/cli/file_io.py"
    scp -q "$SRC/cli/daemon.py"                         "${user}@${host}:${base}/batchalign/cli/daemon.py"
    scp -q "$SRC/cli/dispatch_server.py"                "${user}@${host}:${base}/batchalign/cli/dispatch_server.py"
    scp -q "$SRC/serve/fleet.py"                        "${user}@${host}:${base}/batchalign/serve/fleet.py"
    scp -q "$SRC/serve/jobs.py"                         "${user}@${host}:${base}/batchalign/serve/jobs.py"
    scp -q "$SRC/runtime.py"                            "${user}@${host}:${base}/batchalign/runtime.py"
    scp -q "$SRC/pipelines/asr/whisper.py"              "${user}@${host}:${base}/batchalign/pipelines/asr/whisper.py"
    scp -q "$SRC/pipelines/asr/rev.py"                  "${user}@${host}:${base}/batchalign/pipelines/asr/rev.py"
    scp -q "$SRC/pipelines/asr/oai_whisper.py"          "${user}@${host}:${base}/batchalign/pipelines/asr/oai_whisper.py"
    scp -q "$SRC/pipelines/asr/whisperx.py"             "${user}@${host}:${base}/batchalign/pipelines/asr/whisperx.py"
    scp -q "$SRC/pipelines/asr/utils.py"                "${user}@${host}:${base}/batchalign/pipelines/asr/utils.py"

    # Clear pycache
    ssh -o BatchMode=yes "${user}@${host}" "rm -rf ${base}/batchalign/cli/__pycache__ ${base}/batchalign/serve/__pycache__ ${base}/batchalign/__pycache__ ${base}/batchalign/pipelines/asr/__pycache__" 2>/dev/null
    
    echo "OK   $host"
}

deploy_local() {
    local site
    site=$(find ~/.local/share/uv/tools/batchalign-next -path '*/batchalign/cli/dispatch.py' 2>/dev/null | head -1)
    
    if [ -z "$site" ]; then
        echo "SKIP ming (local): batchalign-next not found"
        return
    fi
    
    local base
    base=$(echo "$site" | sed 's|/batchalign/cli/dispatch.py||')
    
    cp "$PATCHED_DISPATCH"                "$base/batchalign/cli/dispatch.py"
    cp "$SRC/cli/dispatch_local.py"       "$base/batchalign/cli/dispatch_local.py"
    cp "$SRC/cli/dispatch_common.py"      "$base/batchalign/cli/dispatch_common.py"
    cp "$SRC/cli/file_io.py"              "$base/batchalign/cli/file_io.py"
    cp "$SRC/cli/daemon.py"               "$base/batchalign/cli/daemon.py"
    cp "$SRC/cli/dispatch_server.py"      "$base/batchalign/cli/dispatch_server.py"
    cp "$SRC/serve/fleet.py"              "$base/batchalign/serve/fleet.py"
    cp "$SRC/serve/jobs.py"               "$base/batchalign/serve/jobs.py"
    cp "$SRC/runtime.py"                  "$base/batchalign/runtime.py"
    cp "$SRC/pipelines/asr/whisper.py"    "$base/batchalign/pipelines/asr/whisper.py"
    cp "$SRC/pipelines/asr/rev.py"        "$base/batchalign/pipelines/asr/rev.py"
    cp "$SRC/pipelines/asr/oai_whisper.py" "$base/batchalign/pipelines/asr/oai_whisper.py"
    cp "$SRC/pipelines/asr/whisperx.py"   "$base/batchalign/pipelines/asr/whisperx.py"
    cp "$SRC/pipelines/asr/utils.py"      "$base/batchalign/pipelines/asr/utils.py"
    rm -rf "$base/batchalign/cli/__pycache__" "$base/batchalign/serve/__pycache__" "$base/batchalign/__pycache__" "$base/batchalign/pipelines/asr/__pycache__"
    
    echo "OK   ming (local)"
}

# Deploy to all remotes in parallel
for host in bilbo brian davida frodo study vaishnavi lilly andrew sue; do
    deploy_remote "$host" macw &
done

# Deploy locally
deploy_local &

wait
echo ""
echo "All deployments finished."
