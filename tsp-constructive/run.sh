#!/bin/bash

instances=`ls ./instances/*.tsp`

# compiles and creates an optimized binary
cargo build --release

echo "######## twice-around-the-tree ########"
for instance in $instances; do
  echo $instance
  ./target/release/tsp-constructive -p "$instance"
  echo
done
