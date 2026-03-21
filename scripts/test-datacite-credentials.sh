#!/usr/bin/env bash
# Test DataCite credentials by attempting an authenticated metadata query.
#
# Usage:
#   ./test-datacite-credentials.sh                  # test environment
#   ./test-datacite-credentials.sh --production     # production environment

set -euo pipefail

if [ "${1:-}" = "--production" ]; then
    API="https://api.datacite.org"
    CLIENT_ID="SML.TALKBANK"
    # Use a known production DOI for the test
    TEST_DOI="10.21415/T56W31"
    echo "Testing PRODUCTION credentials ($CLIENT_ID)"
else
    API="https://api.test.datacite.org"
    CLIENT_ID="TALKBANK.TALKBANK"
    # Use the test DOI from 2018
    TEST_DOI="10.21415/T5002T"
    echo "Testing TEST credentials ($CLIENT_ID)"
fi

echo -n "Password: "
read -rs PASSWORD
echo ""

# Test with a PUT that updates metadata to its current value (idempotent).
# If auth fails, we get 401. If succeeds, we get 200.
# First, GET the current record.
echo "Fetching $TEST_DOI..."
CURRENT=$(curl -s --max-time 10 "${API}/dois/${TEST_DOI}" 2>&1)

if [ -z "$CURRENT" ]; then
    echo "ERROR: Could not fetch DOI. API may be down."
    exit 1
fi

# Now try a PUT with the same data (requires auth)
echo "Testing write access..."
STATUS=$(curl -s --max-time 10 -o /dev/null -w "%{http_code}" \
    -X PUT \
    -H "Content-Type: application/vnd.api+json" \
    -u "${CLIENT_ID}:${PASSWORD}" \
    -d "$CURRENT" \
    "${API}/dois/${TEST_DOI}")

case "$STATUS" in
    200)
        echo "SUCCESS — credentials are valid (read + write)."
        ;;
    422)
        echo "SUCCESS — credentials are valid (auth passed, 422 = validation error on test data, which is expected)."
        ;;
    401)
        echo "FAILED — wrong password or invalid client ID."
        ;;
    403)
        echo "FAILED — authentication worked but no write permission for this DOI."
        ;;
    *)
        echo "UNEXPECTED — HTTP status $STATUS"
        ;;
esac
