#!/bin/bash


# Check if cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Cargo could not be found. Please install Rust and Cargo."
    exit 1
fi


CURRENT_DIR=$(pwd)

cd imap_service
cargo build --release

cp mailMan_template.service ./mailMan.service
sed -i "s|<BINARY_PATH>|$CURRENT_DIR/target/release/mailMan|g" mailMan.service

sudo mkdir /var/lib/mailMan

sudo cp mailMan.service /etc/systemd/system/


cd $CURRENT_DIR

sudo systemctl daemon-reload
