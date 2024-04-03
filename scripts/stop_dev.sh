#!/bin/bash

# Killing cargo watch and trunk serve processes
echo "Stopping cargo watch and trunk serve..."
pkill -f cargo-watch
pkill -f trunk

# Stop the database
echo "Stopping database..."
cd db
docker-compose down
cd ..

echo "All processes stopped."
