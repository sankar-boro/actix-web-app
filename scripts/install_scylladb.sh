#!/bin/sh

sudo apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys 5e08fbd8b5d6ec9ca
sudo curl -L --output /etc/apt/sources.list.d/scylla.list http://downloads.scylladb.com/deb/ubuntu/scylla-4.5-$(lsb_release -s -c).list
sudo apt-get update
sudo apt-get install -y scylla
sudo apt-get update
sudo apt-get install -y openjdk-8-jre-headless
sudo update-java-alternatives --jre-headless -s java-1.8.0-openjdk-amd64
sudo scylla_dev_mode_setup --developer-mode 1
sudo systemctl start scylla-server
nodetool status