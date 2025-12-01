# Advent of Code {{ cookiecutter.year }} Rust

This is my solution to the [Advent of Code {{ cookiecutter.year }}](https://adventofcode.com/{{ cookiecutter.year }}) written in Rust.
It utilizes the cargo-aoc tool for managing the codebase.

## Usage

Usage is simplified and set up using justfile. Some of the useful commands are:

### For the current day:

#### Running the days parts:

```bash
just
```

or

```bash
just run
```

#### Running the benchmarks:

```bash
just bench
```

#### Running the test examples:

```bash
just test
```

### For a specific day:

#### Running the days parts:

```bash
just run <DAY_NUM>
```

#### Running the benchmarks:

```bash
just bench <DAY_NUM>
```

#### Running the test examples:

```bash
just test <DAY_NUM>
```

### For all days:

#### Running all the days parts:

```bash
just run-all
```

#### Running all the benchmarks:

```bash
just bench-all
```

#### Running all the test examples:

```bash
just test-all
```

### General commands:

#### Fetching all the input files:

```bash
just fetch-inputs
```

#### Generating the boilerplate for a new day:

```bash
just gen
```

#### Linting and formatting the code:

```bash
just lint
```

#### Cleaning the target directory:

```bash
just clean
```

#### Comitting the changes with a focused commit message for completing a specific day:

```bash
just git-complete-day <DAY_NUM>
```

### Misc Notes

As the project uses the cargo-aoc tool, any raw commands using the tool can be run. See the [cargo-aoc](https://github.com/gobanos/cargo-aoc) repository for more information.
