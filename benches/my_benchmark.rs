#[macro_use]
extern crate criterion;
extern crate next_episode;

use criterion::Criterion;
use next_episode::possible_next_episode;

pub const EP_LIST: &'static [&'static str] = &[
    "SomeSeries.S01E01.1080p.SomeFormat.mkv",
    "SomeSeries.S01E05.720p.SomeFormat2.mkv",
    "SomeSeries.S01E07.720p.SomeFormat2.mkv",
    "SomeSeries.S01E08.720p.SomeFormat2.mkv",
    "SomeSeries.S01E09.720p.SomeFormat2.mkv",
    "SomeSeries.S01E010.720p.SomeFormat2.mkv",
    "SomeSeries.S01E02.720p.Format3.mkv",
    "Fav.Series.S01E01.720p.Format1.mkv",
    "Fav.Series.S02E02.1080p.Format2.mkv",
    "Fav.Series.S03E02.720p.Format3.mkv",
];

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("next ep", |b| {
        b.iter(|| possible_next_episode("SomeSeries.S01E01.SomeFormat.mkv", &EP_LIST))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
