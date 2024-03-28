# test-benchmark-iai
Performance bugs are bugs!

## About 
This is test project that implements benchmaring of a function. This project uses [iai](https://crates.io/crates/iai) crate



## Prerequisits
To build and run this project you need to have the following installed on your system:

- Rust (latest stable) – [How to install Rust](https://www.rust-lang.org/en-US/install.html)
- Valgrind (install it with `sudo apt install valgrind`)
- Clone this repo 
```
$ git clone git@github.com:soleinik/test-benchmark-iai.git && cd test-benchmark-iai
```


## How to run
Excecute `cargo bench` in project's root folder

1. First time (for base line)

```
$ cargo bench
   Compiling iai v0.1.1
   Compiling test-benchmark-iai v0.1.0 (/home/me/work/rust/test-benchmark-iai)
    Finished bench [optimized] target(s) in 0.29s
     Running unittests src/lib.rs (target/release/deps/test_benchmark_iai-464df9d66a5e1ef1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/my_benchmark.rs (target/release/deps/my_benchmark-7e7ff4d46eff9905)
iai_benchmark_short
  Instructions:                1787
  L1 Accesses:                 2504
  L2 Accesses:                    0
  RAM Accesses:                   3
  Estimated Cycles:            2609

iai_benchmark_long
  Instructions:            27046772
  L1 Accesses:             37816928
  L2 Accesses:                    1
  RAM Accesses:                   3
  Estimated Cycles:        37817038


```
2. Second time for deltas
```
$ cargo bench
    Finished bench [optimized] target(s) in 0.00s
     Running unittests src/lib.rs (target/release/deps/test_benchmark_iai-464df9d66a5e1ef1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/my_benchmark.rs (target/release/deps/my_benchmark-7e7ff4d46eff9905)
iai_benchmark_short
  Instructions:                1787 (No change)
  L1 Accesses:                 2504 (No change)
  L2 Accesses:                    0 (No change)
  RAM Accesses:                   3 (No change)
  Estimated Cycles:            2609 (No change)

iai_benchmark_long
  Instructions:            27046772 (No change)
  L1 Accesses:             37816928 (No change)
  L2 Accesses:                    1 (No change)
  RAM Accesses:                   3 (No change)
  Estimated Cycles:        37817038 (No change)

```