#!/bin/bash

REPO_OWNER="kadircy"
REPO_NAME="katype"
BINARY_NAME="katype"
BIN_PATH="/usr/bin/$BINARY_NAME"

RELEASE_URL=$(curl -s "https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest" | \
    grep "browser_download_url" | \
    cut -d '"' -f 4)

echo "Downloading $BINARY_NAME from $RELEASE_URL..."
curl -L -o /tmp/$BINARY_NAME "$RELEASE_URL"

chmod +x /tmp/$BINARY_NAME

echo "Copying $BINARY_NAME to /usr/bin..."
sudo mv /tmp/$BINARY_NAME $BIN_PATH

if [ -f "$BIN_PATH" ]; then
    echo "$BINARY_NAME successfully installed to $BIN_PATH"
else
    echo "Error: Failed to install the binary."
    exit 1
fi

