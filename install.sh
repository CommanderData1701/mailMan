#!/bin/bash


# Check if cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Cargo could not be found. Please install Rust and Cargo."
    exit 1
fi


CURRENT_DIR=$(pwd)
CURRENT_USER=$(whoami)
CURRENT_HOME=$(echo ~)

cd imap_service
cargo build --release

cp mailMan_template.service ./mailMan.service
sed -i "s|<BINARY_PATH>|$(pwd)/target/release/mailMan|g" mailMan.service
sed -i "s|<HOME>|$CURRENT_HOME|g" mailMan.service
sed -i "s|<USER>|$CURRENT_USER|g" mailMan.service

sudo mkdir /var/lib/mailMan

sudo cp mailMan_$(CURRENT_USER).service /etc/systemd/system/


cd $CURRENT_DIR

sudo systemctl daemon-reload
