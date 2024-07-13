#!/bin/bash


# Check if cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Cargo could not be found. Please install Rust and Cargo."
    exit 1
fi


cargo build --release

cp mailMan_template.service ./mailMan.service
sed -i "s|<BINARY_PATH>|$(pwd)/target/release/mailMan|g" mailMan.service

sudo cp mailMan.service /etc/systemd/system/
sudo systemctl daemon-reload
