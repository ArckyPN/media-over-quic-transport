#!/usr/bin/env bash

cd "$(dirname "$0")"

HOSTS="${HOSTS:-localhost 127.0.0.1 ::1}"

CERT="localhost.crt"
KEY="localhost.key"
FINGI="localhost.txt"

# # generate self signed certificates
openssl req -x509 \
    -newkey rsa:4096 \
    -keyout "$KEY" \
    -out "$CERT" \
    -sha256 \
    -days 14 \
    -noenc \
    -subj '/CN=localhost' -extensions EXT -config <( \
    printf "[dn]\nCN=localhost\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth")

# install crt to local trust store
sudo cp "$CERT" /usr/share/ca-certificates/localhost/
sudo dpkg-reconfigure ca-certificates

# parse the fingerprint of the certificate
openssl x509 -in "$CERT" -outform der | openssl dgst -sha256 -binary | xxd -p -c 256 > "$FINGI"

# convert fingerprint to byte array
RAW_FINGERPRINT=$(cat $FINGI)
FINGERPRINT="["
for (( i=0; i<${#RAW_FINGERPRINT}; i+=2 )); do
    SUB_STR="${RAW_FINGERPRINT:$i:2}"
    HEX=$(printf "%d" "0x$SUB_STR")
    if [[ "$i" == "0" ]]; then
        FINGERPRINT="${FINGERPRINT}${HEX}"
    else
        FINGERPRINT="${FINGERPRINT}, ${HEX}"
    fi
done
FINGERPRINT="${FINGERPRINT}]"

# write fingerprint byte array to file
echo "$FINGERPRINT" > "$FINGI"