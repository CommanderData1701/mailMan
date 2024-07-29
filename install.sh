#!/bin/bash


# Check if cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Cargo could not be found. Please install Rust and Cargo."
    exit 1
fi


# Setting up some variables needed to create mailMan as instance for this user
CURRENT_DIR=$(pwd)
CURRENT_USER=$(whoami)
CURRENT_HOME=$(echo ~)

cd imap_service
cargo build --release

cp mailMan_template.service ./mailMan.service
sed -i "s|<HOME>|$CURRENT_HOME|g" mailMan.service
sed -i "s|<USER>|$CURRENT_USER|g" mailMan.service

mkdir $CURRENT_HOME/.mailMan

sudo cp target/release/mailMan_imap_service /usr/local/bin
sudo cp mailMan_$CURRENT_USER.service /etc/systemd/system/


cd ../cli
cargo build --release
sudo cp target/release/mailMan_cli /usr/local/bin


cd $CURRENT_DIR

sudo systemctl daemon-reload
