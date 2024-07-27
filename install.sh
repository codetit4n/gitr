#!/bin/bash

cargo build --release
sudo rm -rf /usr/local/bin/gitr
sudo ln -s $(pwd)/target/release/gitr /usr/local/bin/gitr
