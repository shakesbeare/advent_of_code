# Naive version
Benchmark 1: cargo run -- -d 4
  Time (mean ± σ):     27.938 s ±  0.407 s    [User: 27.088 s, System: 0.608 s]
  Range (min … max):   27.654 s … 28.826 s    10 runs

# Optimized version
Benchmark 1: cargo run -- -d 4
  Time (mean ± σ):      52.0 ms ±   0.7 ms    [User: 34.5 ms, System: 11.3 ms]
  Range (min … max):    50.9 ms …  54.2 ms    54 runs
