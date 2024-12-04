test:
    cargo test

run:
    cargo aoc input
    cargo aoc

bench:
    cargo aoc input
    cargo aoc bench

gen:
    cargo aoc input -g

run-all:
    #!/usr/bin/env bash
    for file in $(ls src | grep day); do
        day=${file:3:1}
        echo -------------- AoC Day $day --------------
        cargo aoc input -d $day
        cargo aoc -d $day
    done

bench-all:
    #!/usr/bin/env bash
    for file in $(ls src | grep day); do
        day=${file:3:1}
        echo -------------- AoC Day $day --------------
        cargo aoc input -d $day
        cargo aoc bench -d $day
    done

fetch-inputs:
    #!/usr/bin/env bash
    for file in $(ls src | grep day); do
        day=${file:3:1}
        echo -------------- AoC Day $day --------------
        cargo aoc input -d $day
        echo $?
    done

clean:
    cargo clean

lint:
    cargo fmt
    cargo clippy --fix --allow-dirty --allow-staged

[confirm("Are you sure you want to commit for the day?")]
git-complete-day DAY:
    #!/usr/bin/env bash
    git add .
    git commit -m "Completed AoC Day {{DAY}}"
    git push
