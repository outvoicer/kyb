#!/bin/bash
cd "/home/www/kyb/target/release"
git pull
cargo build --release
sudo service kyb restart
