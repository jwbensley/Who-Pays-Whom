#!/bin/bash

set -eu

# Broken sources
skip_sources=(
  "12276_198.35.53.242"
  "61138_185.121.168.1"
  "61138_2a06_1280_ae01__1"
  "61138_2a06_1280_ae02__1"
  "48390_185.20.3.1"
  "50620_193.107.126.33"
  "53763_23.164.232.192"
  "57777_185.173.128.4"
  "58308_91.109.120"
  "205163_2a06_1281_53__1"
  "206924_185.230.223.3"
  "208038_93.170.122.204"
  "215085_2a06_9f41_a100__1"
  "215085_83.142.31.1"
  "215828_77.90.25.254"
)

for source in $(curl -s https://mrt.bgproutes.io/bgp/ | grep "a href=" | awk -F "\"" '{print $2}' | tr -d "/")
do
  if grep -q "$source" <<< "${skip_sources[*]}"
  then
    echo "Skipping $source"
    continue
  fi
  wget --no-clobber "https://mrt.bgproutes.io/bgp/${source}/2026/02/04/rib.1770163200.mrt.bz2" -O "${1}/${source}_rib.1770163200.mrt.bz2" || true
done
