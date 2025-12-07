#!/bin/bash

DOMAIN="www.template.com"                  # Your domain
EMAIL="template@gmail.com"                    # For Let's Encrypt
CERT_DIR="$HOME/askama-template/private"       # Where your server expects certs
# ============================

echo "[+] Stopping custom web server..."
./server_control.sh stop

echo "[+] Force-renewing certificate for $DOMAIN..."
certbot certonly --standalone --non-interactive --agree-tos --email "$EMAIL" -d "$DOMAIN" --force-renewal

echo "[+] Moving certs to $CERT_DIR..."
mkdir -p "$CERT_DIR"
cp "/etc/letsencrypt/live/$DOMAIN/fullchain.pem" "$CERT_DIR/cert.pem"
cp "/etc/letsencrypt/live/$DOMAIN/privkey.pem" "$CERT_DIR/key.pem"

echo "[+] Restarting custom web server..."
./server_control.sh start

echo "[✅] DONE! Certs renewed and moved to $CERT_DIR"
