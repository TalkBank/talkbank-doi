#!/bin/bash

NOTIFY_EMAIL="macw@andrew.cmu.edu,fmc@andrew.cmu.edu"
ACME_ENDPOINT_URL="https://acme.sectigo.com/v2/InCommonRSAOV"
ACME_KEY_ID="ptuSgGTcJy6EopzZJ_OqTw"
ACME_HMAC_KEY="J_mju70oNNdFUf1-966vfWwmcBlzd-FEU-4zZ-9QV8WmVhnx5arq-1Of2FMMmb-kf6FwLQ35gm0tzINMmDZN1Q"
KEY_TYPE="ecdsa"

# Had to remove --non-interactive because wanted domains listed.
sudo certbot certonly --apache --expand --agree-tos --email "$NOTIFY_EMAIL" --server "$ACME_ENDPOINT_URL" --eab-kid "$ACME_KEY_ID" --eab-hmac-key "$ACME_HMAC_KEY" --key-type "$KEY_TYPE"
