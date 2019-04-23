# HashCode 2019

hashcode 2019 solver in Rust.

disclaimer: 1st project in Rust.

## Example

```
cargo run --release res/b_lovely_landscapes.txt 1000
```

> `1000` is the chunk size, each chunk is brute-forced using greedy search.

## Scores

| Problem           | Score  | time     | chunk size |
|:------------------|-------:|---------:|-----------:|
| Example           | 0      | 0        | 10         |
| Lovely Landscapes | 106368 | 2min13   | 20000      |
| Memorable moments | 1474   | 0.654s   | 1000       |
| Pet pictures      | 406686 | 6min07s  | 20000      |
| Shiny selfies     | 327529 | 13min54s | 20000      |

total: `840583`

cpu: `Intel i5 760 (4) @ 2.801GHz`

## Crates

* <https://docs.rs/structopt>
* <https://docs.rs/rayon>
* <https://doc.servo.org/fnv/>

Why _Fowler–Noll–Vo_ hash function?

<https://stackoverflow.com/questions/35439376/python-set-intersection-is-faster-then-rust-hashset-intersection>
