#!/bin/bash


# Check if cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Cargo could not be found. Please install Rust and Cargo."
    exit 1
fi

ask_yes_no() {
    local prompt="$1"
    local default="$2"
    local response

    # Convert default to lowercase for consistency
    default=${default,,}

    while true; do
        # Prompt user with a message
        read -p "$prompt" response

        # Convert response to lowercase
        response=${response,,}

        # Handle empty input (default case)
        if [[ -z "$response" ]]; then
            response="$default"
        fi

        # Check response and act accordingly
        case "$response" in
            y|yes)
                return 0
                ;;
            n|no)
                return 1
                ;;
            *)
                echo "Please answer 'y' or 'n'."
                ;;
        esac
    done
}


# Setting up some variables needed to create mailMan as instance for this user
CURRENT_DIR=$(pwd)
CURRENT_USER=$(whoami)
CURRENT_HOME=$(echo ~)

cd imap_service

echo "Building and installing mailMan imap service..."
cargo build --release
cp mailMan_template.service ./mailMan.service
sed -i "s|<HOME>|$CURRENT_HOME|g" mailMan.service
sed -i "s|<USER>|$CURRENT_USER|g" mailMan.service

mkdir $CURRENT_HOME/.mailMan

sudo cp target/release/mailMan_imap_service /usr/local/bin
sudo cp mailMan_$CURRENT_USER.service /etc/systemd/system/
echo "DONE!"


echo "Building and installing mailMan cli..."
cd ../cli
cargo build --release
sudo cp target/release/mailMan_cli /usr/local/bin/mailMan
echo "DONE!"


cd $CURRENT_DIR

echo "Reloading systemd daemon..."
sudo systemctl daemon-reload
echo "DONE!"


if ask_yes_no "Do you want to enable mailMan service? [y/N]: " "n"; then
    echo "Enabling mailMan service..."
    sudo systemctl enable mailMan_$CURRENT_USER
    echo "DONE!"
fi


if ask_yes_no "Do you want to start mailMan service? [y/N]: " "n"; then
    echo "Starting mailMan service..."
    sudo systemctl start mailMan_$CURRENT_USER
    echo "DONE!"
fi

echo "Installation finished."
