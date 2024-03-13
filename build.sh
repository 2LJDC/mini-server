#!/bin/bash

cargo build --release

docker build -t easy-dockerfile .

