#!/bin/bash

instances=`ls ./instances/*.tsp`
declare -a algorithms=("twice-around-the-tree") #"christofides")

# compile and create an optimized binary
cargo build --release

# iterate trhough the algorithms that were implemented
for algo in "${algorithms[@]}"; do
  echo "----- $algo -----"
  # execute each algorithm for every instance
  for instance in $instances; do
    echo $instance
    ./target/release/tsp-constructive -a $algo -p "$instance"
    echo
  done

  echo
done
