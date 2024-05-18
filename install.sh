#!/bin/bash

cargo build --release
sudo rm -rf /usr/local/bin/gitr
sudo cp target/release/gitr /usr/local/bin/gitr
