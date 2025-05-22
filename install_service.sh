#!/bin/bash

# Get the absolute path to the current directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Build the release version
echo "Building release version..."
cd "$SCRIPT_DIR"
cargo build --release

# Create systemd service file with correct paths
cat > "$SCRIPT_DIR/fetch-real-ip.service" << EOL
[Unit]
Description=Telegram Bot for fetching real IP address
After=network.target

[Service]
Type=simple
User=$(whoami)
WorkingDirectory=$SCRIPT_DIR
ExecStart=$SCRIPT_DIR/target/release/fetch-real-ip
Restart=on-failure
RestartSec=10
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOL

# Copy service file to systemd directory
echo "Installing systemd service..."
sudo cp "$SCRIPT_DIR/fetch-real-ip.service" /etc/systemd/system/

# Reload systemd daemon
echo "Reloading systemd daemon..."
sudo systemctl daemon-reload

# Enable and start the service
echo "Enabling and starting the service..."
sudo systemctl enable fetch-real-ip.service
sudo systemctl start fetch-real-ip.service

echo "Service installed and started. Check status with: systemctl status fetch-real-ip.service"
