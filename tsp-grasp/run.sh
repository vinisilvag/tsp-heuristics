#!/bin/bash

instances=`ls ./instances/*.tsp`

# compiles and creates an optimized binary
cargo build --release

echo "######## GRASP ########"
for instance in $instances; do
  echo $instance
  ./target/release/tsp-grasp -p "$instance"
  echo
done
