#!/usr/bin/env bash
# Verify Tencent ASR word segmentation for Cantonese.
#
# Uses the existing yue_hku_clip.mp3 test fixture (A023, 26s).
# Runs on net via SSH (Tencent credentials required).
#
# Output: raw Tencent ASR result showing whether words are
# pre-segmented (multi-character) or per-character.
#
# Usage: bash scripts/check-media/verify_tencent_cantonese.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE="$(cd "$SCRIPT_DIR/../.." && pwd)"
CLIP="$WORKSPACE/batchalign3/batchalign/tests/support/yue_hku_clip.mp3"
REMOTE_HOST="macw@net"
REMOTE_TMP="/tmp/tencent_cantonese_test"

echo "=== Tencent Cantonese Word Segmentation Verification ==="
echo "Source: $CLIP (A023.mp4, lines 11-16, 26s Cantonese aphasia speech)"
echo ""

# 1. Copy clip to net
echo "Copying clip to net..."
ssh "$REMOTE_HOST" "mkdir -p $REMOTE_TMP"
scp "$CLIP" "$REMOTE_HOST:$REMOTE_TMP/yue_hku_clip.mp3"

# 2. Run batchalign3 transcribe with Tencent engine
echo "Running batchalign3 transcribe with Tencent ASR on net..."
ssh "$REMOTE_HOST" "cd $REMOTE_TMP && batchalign3 --no-open-dashboard transcribe yue_hku_clip.mp3 -o output/ --lang yue --engine-overrides '{\"asr\": \"tencent\"}' -v --workers 1" 2>&1 | tail -15

# 3. Fetch and display the result
echo ""
echo "=== Tencent ASR Output ==="
ssh "$REMOTE_HOST" "cat $REMOTE_TMP/output/yue_hku_clip.cha 2>/dev/null || echo 'Output not found'"

# 4. Analyze word boundaries
echo ""
echo "=== Word Boundary Analysis ==="
ssh "$REMOTE_HOST" "python3 -c \"
import re
with open('$REMOTE_TMP/output/yue_hku_clip.cha') as f:
    for line in f:
        if line.startswith('*'):
            content = line.split('\t', 1)[1].strip() if '\t' in line else ''
            words = content.split()
            cjk_words = [w for w in words if any('\u4e00' <= c <= '\u9fff' for c in w)]
            multi_char = [w for w in cjk_words if len(w) > 1]
            single_char = [w for w in cjk_words if len(w) == 1]
            print(f'  {content[:60]}...' if len(content) > 60 else f'  {content}')
            print(f'    CJK words: {len(cjk_words)} ({len(multi_char)} multi-char, {len(single_char)} single-char)')
            if multi_char:
                print(f'    Multi-char examples: {multi_char[:5]}')
            print()
\"" 2>/dev/null

echo ""
echo "=== Conclusion ==="
ssh "$REMOTE_HOST" "python3 -c \"
import re
multi_total = 0
single_total = 0
with open('$REMOTE_TMP/output/yue_hku_clip.cha') as f:
    for line in f:
        if line.startswith('*'):
            content = line.split('\t', 1)[1].strip() if '\t' in line else ''
            for w in content.split():
                if any('\u4e00' <= c <= '\u9fff' for c in w):
                    if len(w) > 1:
                        multi_total += 1
                    else:
                        single_total += 1
total = multi_total + single_total
if total > 0:
    pct = multi_total * 100 // total
    print(f'Total CJK words: {total} ({multi_total} multi-char = {pct}%, {single_total} single-char)')
    if pct > 50:
        print('VERDICT: Tencent DOES produce word-segmented output for Cantonese')
    else:
        print('VERDICT: Tencent produces mostly per-character output for Cantonese')
else:
    print('No CJK words found in output')
\"" 2>/dev/null

# Cleanup
echo ""
echo "Results saved on net at $REMOTE_TMP/output/"
