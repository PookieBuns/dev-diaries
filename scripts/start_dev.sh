#!/bin/bash

# Navigate to the db directory and start the database
echo "Starting database..."
cd db
docker-compose up -d
cd ..

# Navigate to the backend directory and run cargo watch
echo "Starting cargo watch in the backend..."
cd backend
cargo watch -c -x run &
cd ..

# Navigate to the frontend directory and run trunk serve
echo "Starting trunk serve in the frontend..."
cd frontend
trunk serve &
cd ..

# Wait for a moment to ensure that the above commands have time to start
sleep 5

# Navigate to the directory with your nginx.conf and start Nginx
echo "Starting Nginx..."
sudo nginx -c $(pwd)/nginx.conf
