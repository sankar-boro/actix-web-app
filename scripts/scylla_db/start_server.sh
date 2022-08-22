#!/bin/bash

echo "start scylla server"
sudo systemctl start scylla-server
echo "sleeping for 30 seconds"
sleep 30
echo "Run nodetool status"
nodetool status

# sudo scylla_dev_mode_setup --developer-mode 1
