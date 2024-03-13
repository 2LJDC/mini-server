#!/bin/bash

cargo build --release

docker build -t mini-server .

