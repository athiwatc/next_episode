# next_episode
Get the next possible episode

```Rust
const EP_LIST: &'static [&'static str] = &[
    "SomeSeries.S01E01.1080p.SomeFormat.mkv",
    "SomeSeries.S01E05.720p.SomeFormat2.mkv",
    "SomeSeries.S01E02.720p.Format3.mkv",
    "Fav.Series.S01E01.720p.Format1.mkv",
    "Fav.Series.S02E02.1080p.Format2.mkv",
    "Fav.Series.S03E02.720p.Format3.mkv",
];

#[test]
fn next_ep() {
    let nxt = possible_next_episode("SomeSeries.S01E01.SomeFormat.mkv", &EP_LIST);
    assert_eq!(nxt, Some("SomeSeries.S01E02.720p.Format3.mkv"));
}

#[test]
fn next_ep_sprase() {
    let nxt = possible_next_episode("SomeSeries.S01E02.720p.Format3.mkv", &EP_LIST);
    assert_eq!(nxt, Some("SomeSeries.S01E05.720p.SomeFormat2.mkv"));
}

#[test]
fn across_season() {
    let nxt = possible_next_episode("Fav.Series.S01E01.720p.Format1.mkv", &EP_LIST);
    assert_eq!(nxt, Some("Fav.Series.S02E02.1080p.Format2.mkv"));
}

#[test]
fn last_ep() {
    let nxt = possible_next_episode("Fav.Series.S03E02.720p.Format3.mkv", &EP_LIST);
    assert_eq!(nxt, None);
}
```
