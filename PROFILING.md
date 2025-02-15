# Profiling

Why is your program slow? Profiling can help you find out!
It's easy to do in Rust.
I personally use two tools: `hyperfine` and `cargo-flamegraph`.
The former is a benchmarking tool, and the latter is a profiling tool.
That's all I need to find out how to make my program faster. 

## First steps

First, make sure you build the release version of the program:

```sh
cargo build --release
```

## Using hyperfine

To benchmark CLI commands, you can use [`hyperfine`](https://github.com/sharkdp/hyperfine). For example, to benchmark the `ff` command:

```sh
hyperfine 'target/release/ff ..'
```

The output will look like this:

```sh
Benchmark #1: target/release/ff ..
  Time (mean ± σ):      1.3 ms ±   0.1 ms    [User: 1.1 ms, System: 0.2 ms]
  Range (min … max):    1.1 ms …   1.6 ms    100 runs
```


## Using cargo-flamegraph

To profile the program, you can use [`cargo-flamegraph`](https://github.com/killercup/cargo-flamegraph). 

```sh
# Profile `ff` using the current directory
sudo cargo flamegraph --release -- .   
```

The output is an SVG file that you can open in a browser. 
