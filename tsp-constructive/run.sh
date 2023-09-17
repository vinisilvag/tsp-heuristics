#!/bin/bash

instances=`ls ./instances/*.tsp`
declare -a algorithms=("twice-around-the-tree") #"christofides")

cargo build

for algo in "${algorithms[@]}"; do
  echo $algo
  for instance in $instances; do
    echo $instance
    ./target/debug/tsp-constructive -a $algo -p "$instance"
    echo
  done

  echo "#####################################"
done
