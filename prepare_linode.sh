#!/bin/bash
apt update
apt install -y libsasl2-dev pkg-config build-essential libudev-dev libssl-dev protobuf-compiler libprotobuf-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

sed -i 's/nameserver 127.0.0.53/nameserver 1.1.1.1/g' /etc/resolv.conf
