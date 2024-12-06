#!/bin/bash

# Define variables
POSTGRES_IMAGE="docker.io/library/postgres:latest"
CONTAINER_NAME="postgres-container"
POSTGRES_USER="postgres"
POSTGRES_PASSWORD="postgres"
POSTGRES_DB="eshop"
POSTGRES_PORT="5432"

# Pull the latest PostgreSQL image
echo "Pulling the latest PostgreSQL image..."
podman pull $POSTGRES_IMAGE

# Run the container with the --replace option and persistent data
echo "Running PostgreSQL container with persistent storage..."
podman run -d \
  --name $CONTAINER_NAME \
  --replace \
  -e POSTGRES_USER=$POSTGRES_USER \
  -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD \
  -e POSTGRES_DB=$POSTGRES_DB \
  -p $POSTGRES_PORT:5432 \
  $POSTGRES_IMAGE

# Check if the container is running
if podman ps | grep -q $CONTAINER_NAME; then
  echo "PostgreSQL container is running."
  echo "Connection details:"
  echo "Host: localhost"
  echo "Port: $POSTGRES_PORT"
  echo "Database: $POSTGRES_DB"
  echo "User: $POSTGRES_USER"
  echo "Data Directory: $LOCAL_DATA_DIR"
else
  echo "Failed to start PostgreSQL container."
fi
