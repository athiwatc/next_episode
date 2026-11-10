# next_episode
Get the next possible episode

## Development

Use a current stable Rust toolchain. The Criterion benchmark dependency requires
Rust 1.86 or newer.

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo test --locked --doc
cargo bench --locked --bench my_benchmark -- --test
cargo install cargo-audit --version 0.22.2 --locked
cargo audit
```

Version 0.2.0:
- Improve speed of regex function

```Rust
const EP_LIST: &[&str] = &[
    "SomeSeries.S01E01.1080p.SomeFormat.mkv",
    "SomeSeries.S01E05.720p.SomeFormat2.mkv",
    "SomeSeries.S01E02.720p.Format3.mkv",
    "Fav.Series.S01E01.720p.Format1.mkv",
    "Fav.Series.S02E02.1080p.Format2.mkv",
    "Fav.Series.S03E02.720p.Format3.mkv",
];

#[test]
fn next_ep() {
    let nxt = possible_next_episode("SomeSeries.S01E01.SomeFormat.mkv", EP_LIST);
    assert_eq!(nxt, Some("SomeSeries.S01E02.720p.Format3.mkv"));
}

#[test]
fn next_ep_sprase() {
    let nxt = possible_next_episode("SomeSeries.S01E02.720p.Format3.mkv", EP_LIST);
    assert_eq!(nxt, Some("SomeSeries.S01E05.720p.SomeFormat2.mkv"));
}

#[test]
fn across_season() {
    let nxt = possible_next_episode("Fav.Series.S01E01.720p.Format1.mkv", EP_LIST);
    assert_eq!(nxt, Some("Fav.Series.S02E02.1080p.Format2.mkv"));
}

#[test]
fn last_ep() {
    let nxt = possible_next_episode("Fav.Series.S03E02.720p.Format3.mkv", EP_LIST);
    assert_eq!(nxt, None);
}
```
