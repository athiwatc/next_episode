#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;

struct Episode<'a> {
    fullname: &'a str,
    name: &'a str,
    episode: i32,
    season: i32,
}

pub fn possible_next_episode<'a>(current_episode: &str, episodes: &'a [&str]) -> Option<&'a str> {
    parse_episode(current_episode).and_then(|curr_ep| {
        episodes
            .iter()
            .map(|m| parse_episode(m))
            .filter_map(|f| f)
            .filter(|f| f.name == curr_ep.name)
            .filter(|f| {
                (f.episode > curr_ep.episode && f.season == curr_ep.season)
                    || f.season > curr_ep.season
            })
            .sorted_by(|a, b| match Ord::cmp(&a.season, &b.season) {
                Ordering::Equal => Ord::cmp(&a.episode, &b.episode),
                other => other,
            })
            .first()
            .map(|m| m.fullname)
    })
}

fn parse_episode(episode: &str) -> Option<Episode> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"((?i)(.+).s(\d+)e(\d+).*)").unwrap();
    }

    RE.captures(episode).and_then(|caps| {
        match (
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(4).unwrap().as_str().parse::<i32>(),
            caps.get(3).unwrap().as_str().parse::<i32>(),
        ) {
            (fullname, name, Ok(episode), Ok(season)) => Some(Episode {
                fullname,
                name,
                episode,
                season,
            }),
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use possible_next_episode;

    const EP_LIST: &'static [&'static str] = &[
        "SomeSeries.S01E01.1080p.SomeFormat.mkv",
        "SomeSeries.S01E05.720p.SomeFormat2.mkv",
        "SomeSeries.S01E02.720p.Format3.mkv",
        "FavSeries.S01E01.720p.Format1.mkv",
        "FavSeries.S02E02.1080p.Format2.mkv",
        "FavSeries.S03E02.720p.Format3.mkv",
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
        let nxt = possible_next_episode("FavSeries.S01E01.720p.Format1.mkv", &EP_LIST);
        assert_eq!(nxt, Some("FavSeries.S02E02.1080p.Format2.mkv"));
    }

    #[test]
    fn last_ep() {
        let nxt = possible_next_episode("FavSeries.S03E02.720p.Format3.mkv", &EP_LIST);
        assert_eq!(nxt, None);
    }
}
