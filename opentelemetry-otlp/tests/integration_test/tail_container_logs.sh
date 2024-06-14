#!/bin/bash

# List of container names to monitor
containers=("otel-collector")

# Function to tail logs for a given container name
tail_logs() {
  container_name=$1
  while true; do
    echo "Waiting for container: $container_name to start"
    
    # Wait until the container is running
    until [ "$(docker inspect -f {{.State.Running}} $container_name 2>/dev/null)" == "true" ]; do
      sleep 1
    done
    
    echo "Tailing logs for container: $container_name"
    docker logs -f $container_name
    
    echo "Container $container_name stopped, waiting to restart"
    
    # Wait until the container has fully stopped before retrying
    while [ "$(docker inspect -f {{.State.Running}} $container_name 2>/dev/null)" != "false" ]; do
      sleep 1
    done
  done
}

# Start tailing logs for each container in the background
for container in "${containers[@]}"; do
  tail_logs $container &
done

# Wait for all background processes to finish
wait