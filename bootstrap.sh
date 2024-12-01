#!/usr/bin/bash

for i in {1..25}; do
  touch "day$i/input.txt"
  touch "day$i/test.txt"
done

rm generate.sh
rm bootstrap.sh
