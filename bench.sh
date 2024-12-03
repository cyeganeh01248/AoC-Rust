#!/usr/bin/env bash


for file in $(ls src | grep day); do
    day=${file:3:1}
    echo -------------- AoC Day $day --------------
    cargo aoc input -d $day
    cargo aoc -d $day
    cargo aoc bench -d $day
done
# for i in $(seq 1 $(( $(ls src | grep day | wc -l) ))); do
#     echo $i
# done
