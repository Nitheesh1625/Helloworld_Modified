#!/bin/bash

clear
cargo clean
cargo run
dmesg | tail
