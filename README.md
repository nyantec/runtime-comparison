# Comparison of Rust async runtimes

For a very specific use case, we needed to know the minimum memory overhead achievable with the various async runtimes. Here are the results, as of Nov 08 2021:

| runtime   | RES      | VIRT     | binary size |
| --------- | -------- | -------- | ----------- |
| async-std | 2548     | 8672     | 1.0M        |
| smol      | 2412     | 6412     | 696K        |
| tokio     | 436      | 4404     | 780K        |
| none      | 424      | 4200     | 452K        |

All tests were run in release mode and with `MALLOC_ARENA_MAX=1`.
