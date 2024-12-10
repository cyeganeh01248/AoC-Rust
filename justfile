default: run

test DAY="":
    #!/usr/bin/env bash
    day={{DAY}}

    if [ "$day" == "" ]; then
        day=$(ls src | grep day | sed 's/day//' | sed 's/.rs//' | sort -h -r | head -n 1)
    fi
    cargo test day${day}

run DAY="":
    #!/usr/bin/env bash
    day={{DAY}}

    if [ "$day" == "" ]; then
        day=$(ls src | grep day | sed 's/day//' | sed 's/.rs//' | sort -h -r | head -n 1)
    fi
    cargo aoc input -d $day
    cargo aoc -d $day

bench DAY="":
    #!/usr/bin/env bash
    day={{DAY}}

    if [ "$day" == "" ]; then
        day=$(ls src | grep day | sed 's/day//' | sed 's/.rs//' | sort -h -r | head -n 1)
    fi

    cargo aoc input -d $day
    cargo aoc bench -d $day

gen:
    cargo aoc input -g

test-all:
    cargo test day

run-all:
    #!/usr/bin/env bash
    for day in $(ls src | grep day | sed 's/day//' | sed 's/.rs//' | sort -h); do
        echo -------------- AoC Day $day --------------
        cargo aoc input -d $day
        cargo aoc -d $day
    done

bench-all:
    #!/usr/bin/env bash
    for day in $(ls src | grep day | sed 's/day//' | sed 's/.rs//' | sort -h); do
        echo -------------- AoC Day $day --------------
        cargo aoc input -d $day
        cargo aoc bench -d $day
    done

fetch-inputs:
    #!/usr/bin/env bash
    for day in $(ls src | grep day | sed 's/day//' | sed 's/.rs//' | sort -h); do
        echo -------------- AoC Day $day --------------
        cargo aoc input -d $day &> /dev/null
        if [ $? -ne 0 ]; then
            echo "Failed to fetch input for day $day"
        else
            echo "Fetched input for day $day"
        fi
    done


run-file:
    #!/usr/bin/env bash
    echo "AOC 2024" | tee results/run-results.txt
    for day in $(ls src | grep day | sed 's/day//' | sed 's/.rs//' | sort -h); do
        echo ------------------------ | tee -a results/run-results.txt
        cargo aoc input -d $day 2> /dev/null
        cargo aoc -d $day 2> /dev/null | grep -E "Day" | tee -a results/run-results.txt
    done
    echo ------------------------ | tee -a results/run-results.txt

bench-file:
    #!/usr/bin/env bash
    echo "AOC 2024" | tee results/bench-results.txt
    for day in $(ls src | grep day | sed 's/day//' | sed 's/.rs//' | sort -h); do
        echo ------------------------ | tee -a results/bench-results.txt
        cargo aoc input -d $day 2> /dev/null
        cargo aoc bench -d $day 2> /dev/null | grep Day | tee -a results/bench-results.txt
    done
    echo ------------------------ | tee -a results/bench-results.txt

clean:
    cargo clean

lint:
    cargo fmt
    cargo clippy --fix --allow-dirty --allow-staged

[confirm("Are you sure you want to commit for the day?")]
git-complete-day DAY:
    #!/usr/bin/env bash
    git add . && \
    git commit -m "Completed day {{DAY}}" && \
    git push
