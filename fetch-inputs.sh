#!/usr/bin/env bash


for file in $(ls src | grep day); do
    day=${file:3:1}
    echo -------------- AoC Day $day --------------
    cargo aoc input -d $day
done
