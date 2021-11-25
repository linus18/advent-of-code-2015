# advent-of-code-2015

Second year going at [Advent of Code](https://adventofcode.com/) with Rust, so figured might be good to brush off some 
of the "cobwebs". So looking at older puzzles.

I am using a very handy [cargo aoc](https://github.com/gobanos/cargo-aoc).

- Would be nice to be able to put code from multiple years into a single repo, but it looks it would take a bit of 
effort

# To run:

```
➜  advent-of-code-2015 git:(main) cargo aoc
   Compiling proc-macro2 v1.0.32
   Compiling unicode-xid v0.2.2
   Compiling serde v1.0.130
   ...
   Compiling aoc-autobuild v0.3.0 (/Users/mihajlo/projects/advent-of-code-2015/target/aoc/aoc-autobuild)
    Finished release [optimized] target(s) in 13.54s
     Running `target/release/aoc-autobuild`
AOC 2015
Day 1 - Part 1 : 280
	generator: 8.459µs,
	runner: 26.977µs

Day 1 - Part 2 : 1797
	generator: 892ns,
	runner: 18.699µs

Day 1 - Part 2 - alt1 : 1797
	generator: 843ns,
	runner: 11.468µs
```

# Benchmarking

```
➜  advent-of-code-2015 git:(main) ✗ cargo aoc bench
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
    Updating crates.io index
   Compiling autocfg v1.0.1
   Compiling serde v1.0.130
   Compiling ryu v1.0.5
   ...
    Finished bench [optimized] target(s) in 27.02s
     Running unittests (target/release/deps/aoc_benchmark-49641cbdb7995335)
Day1 - Part1/(default)  time:   [9.0734 us 9.1064 us 9.1411 us]                                    
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

Day1 - Part2/(default)  time:   [10.210 us 10.335 us 10.463 us]                                    
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild
Day1 - Part2/alt1       time:   [6.6124 us 6.8862 us 7.1626 us]   
```