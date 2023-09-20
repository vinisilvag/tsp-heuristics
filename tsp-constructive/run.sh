#!/bin/bash

instances=`ls ./instances/*.tsp`
declare -a algorithms=("twice-around-the-tree") #"christofides")

# compiles and creates an optimized binary
cargo build --release

# iterates through each implemented algorithm
for algo in "${algorithms[@]}"; do
  echo "######## $algo ########"
  # runs each algorithm for each problem instance
  for instance in $instances; do
    echo $instance
    ./target/release/tsp-constructive -a $algo -p "$instance"
    echo
  done

  echo
done
