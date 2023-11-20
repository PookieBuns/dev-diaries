#!/bin/bash

# Stopping Nginx
echo "Stopping Nginx..."
sudo nginx -s stop

# Killing cargo watch and trunk serve processes
echo "Stopping cargo watch and trunk serve..."
pkill -f cargo-watch
pkill -f trunk

echo "All processes stopped."
