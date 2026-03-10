#!/bin/bash

NOTIFY_EMAIL="macw@andrew.cmu.edu,fmc@andrew.cmu.edu"
ACME_ENDPOINT_URL="https://acme.sectigo.com/v2/InCommonRSAOV"
ACME_KEY_ID="[REDACTED — see ~/.talkbank-secrets/acme-certbot.env]"
ACME_HMAC_KEY="[REDACTED — see ~/.talkbank-secrets/acme-certbot.env]"
KEY_TYPE="ecdsa"

# Had to remove --non-interactive because wanted domains listed.
sudo certbot certonly --apache --expand --agree-tos --email "$NOTIFY_EMAIL" --server "$ACME_ENDPOINT_URL" --eab-kid "$ACME_KEY_ID" --eab-hmac-key "$ACME_HMAC_KEY" --key-type "$KEY_TYPE"
