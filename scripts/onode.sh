#!/bin/bash

su core

cd /mnt/hgfs/esr22-23 # /media/sf_esr22-23 12

cargo run --bin oNode ip
