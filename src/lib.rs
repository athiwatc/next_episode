#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;

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
            .filter_map(|m| parse_episode(m))
            .filter(|f| f.name == curr_ep.name)
            .filter(|f| {
                (f.episode > curr_ep.episode && f.season == curr_ep.season)
                    || f.season > curr_ep.season
            })
            .sorted_by_key(|k| (k.season, k.episode))
            .next()
            .map(|m| m.fullname)
    })
}

fn parse_episode(episode: &str) -> Option<Episode<'_>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"((?i)^(.+).s(\d+)e(\d+).*)$").unwrap();
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
    use super::*;

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

    #[test]
    fn episodes_are_ordered_numerically() {
        let episodes = [
            "Series.S01E10.mkv",
            "Series.S02E01.mkv",
            "Series.S01E03.mkv",
        ];
        assert_eq!(
            possible_next_episode("Series.S01E02.mkv", &episodes),
            Some("Series.S01E03.mkv")
        );
    }

    #[test]
    fn invalid_candidates_are_ignored() {
        let episodes = [
            "not an episode",
            "Series.S01E999999999999999999999.mkv",
            "Series.S999999999999999999999E01.mkv",
            "Other.Series.S01E02.mkv",
            "Series.S01E02.mkv",
        ];
        assert_eq!(
            possible_next_episode("Series.S01E01.mkv", &episodes),
            Some("Series.S01E02.mkv")
        );
    }

    #[test]
    fn invalid_current_episode_returns_none() {
        assert_eq!(possible_next_episode("not an episode", EP_LIST), None);
        assert_eq!(
            possible_next_episode("Series.S01E999999999999999999999.mkv", EP_LIST),
            None
        );
    }

    #[test]
    fn empty_episode_list_returns_none() {
        assert_eq!(possible_next_episode("Series.S01E01.mkv", &[]), None);
    }

    #[test]
    fn lowercase_episode_markers_are_supported() {
        let episodes = ["Series.s01e02.mkv"];
        assert_eq!(
            possible_next_episode("Series.S01E01.mkv", &episodes),
            Some("Series.s01e02.mkv")
        );
    }

    #[test]
    fn equal_episode_numbers_preserve_input_order() {
        let episodes = ["Series.S01E02.720p.mkv", "Series.S01E02.1080p.mkv"];
        assert_eq!(
            possible_next_episode("Series.S01E01.mkv", &episodes),
            Some("Series.S01E02.720p.mkv")
        );
    }
}
