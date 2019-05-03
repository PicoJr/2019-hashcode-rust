# HashCode 2019

hashcode 2019 solver in Rust.

disclaimer: 1st project in Rust.

## Example

```
cargo run --release res/b_lovely_landscapes.txt 1000
```

> `1000` is the chunk size, each chunk is brute-forced using greedy search.

## Scores

| Problem           | H     | V     | Score  | time     | chunk size |
|:------------------|:------|:------|-------:|---------:|-----------:|
| Example           | 2     | 2     | 1      | 0        | 10         |
| Lovely Landscapes | 80000 | 0     | 202854 | 9min17   | 80000      |
| Memorable moments | 500   | 500   | 1474   | 0.654s   | 1000       |
| Pet pictures      | 30000 | 60000 | 406686 | 6min07s  | 20000      |
| Shiny selfies     | 0     | 80000 | 327529 | 13min54s | 20000      |

total: `938543`

cpu: `Intel i5 760 (4) @ 2.801GHz`

## Optimizations / Heuristics

* _tags_ are re-indexed (replaced with integers) during initialization. (integers have much smaller overhead)
* horizontal slides are favored over vertical slides when score is the same.
rational: verticals are more versatiles than horizontals, so they are kept for later use.

### Score Function

score function only uses **1** operation on sets:

```rust
    pub fn score(tags_set: &Tags, other_tags_set: &Tags) -> usize {
        let same = FnvHashSet::intersection(tags_set, other_tags_set).count();
        if same == 0 {
            return 0;
        };
        let unique = tags_set.len() - same;
        let other_unique = other_tags_set.len() - same;
        cmp::min(cmp::min(unique, other_unique), same)
    }
```

vs (unoptimized)

```rust
    pub fn score(tags_set: &Tags, other_tags_set: &Tags) -> usize {
        let same = FnvHashSet::intersection(tags_set, other_tags_set).count();
        if same == 0 {
            return 0;
        };
        let unique = FnvHashSet::difference(tags_set, other_tags_set).count();
        if unique == 0 {
            return 0;
        };
        let other_unique = FnvHashSet::difference(other_tags_set, tags_set).count();
        if other_unique == 0 {
            return 0;
        };
        cmp::min(cmp::min(unique, other_unique), same)
    }
```


## Crates

* <https://docs.rs/structopt>
* <https://docs.rs/rayon>
* <https://doc.servo.org/fnv/>

Why _Fowler–Noll–Vo_ hash function?

<https://stackoverflow.com/questions/35439376/python-set-intersection-is-faster-then-rust-hashset-intersection>
