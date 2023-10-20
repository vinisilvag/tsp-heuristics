#!/bin/bash

instances=`ls ./instances/*.tsp`

# compiles and creates an optimized binary
cargo build --release

echo "######## VND ########"
for instance in $instances; do
  echo $instance
  ./target/release/tsp-vnd -p "$instance"
  echo
done
