use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use lazy_static::lazy_static;
use priority_queue::PriorityQueue;
use regex::Regex;
use std::collections::LinkedList;
use thiserror::Error;

lazy_static! {
    static ref MAGNET_REGEX: Regex = match Regex::new(
        r#"(?m)href="(/view/[^"]+?)" title="([^"]+?)"(?:.|[\n\r ])+?(magnet:[^"]+)(?:.|[\n\r ])+?text-center(?:.|[\n\r ])+?text-center(?:.|[\n\r ])+?text-center">(\d+)"#,
    ) {
        Ok(regex) => regex,
        Err(error) => panic!("Failed to create regex for magnet link, error: {}", error),
    };
}

#[derive(Error, Debug)]
pub enum ResponseParsingError {
    #[error("Regex capture count mismatch: expected {0}, actual {1}")]
    RegexCaptureCountMismatch(usize /* expected */, usize /* actual */),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AnimeIndexer {
    pub name: String,
    pub url: String,
}

pub fn fetch_indexers() -> Result<LinkedList<AnimeIndexer>, ResponseParsingError> {
    const ANIME_LIST_URL: &str = "https://kitsunekko.net/dirlist.php?dir=subtitles%2Fjapanese%2F";
    let regex = Regex::new(r#"<tr><td colspan="2"><a href="/([^"]+).+?<strong>([^<]+)"#).unwrap();
    let response_text = reqwest::blocking::get(ANIME_LIST_URL)
        .unwrap()
        .text()
        .unwrap();

    let mut anime_list = LinkedList::new();
    for capture in regex.captures_iter(&response_text) {
        if capture.len() != 3 {
            return Err(ResponseParsingError::RegexCaptureCountMismatch(
                3,
                capture.len(),
            ));
        }

        let anime_url = format!("https://kitsunekko.net/{}", capture[1].to_string());
        let anime_name = capture[2].to_string();
        anime_list.push_back(AnimeIndexer {
            name: anime_name,
            url: anime_url,
        });
    }

    return Ok(anime_list);
}

pub fn fuzzy_match_indexers(
    anime_name: &str,
    indexes: LinkedList<AnimeIndexer>,
) -> Vec<AnimeIndexer> {
    let matcher = SkimMatcherV2::default();
    let mut matches = PriorityQueue::new();
    for index in indexes {
        let score = if index.name.to_lowercase() == anime_name.to_lowercase() {
            Some(std::i64::MAX)
        } else {
            matcher.fuzzy_match(&index.name.to_lowercase(), &anime_name.to_lowercase())
        };

        match score {
            Some(score) => {
                let size_diff = (index.name.len() as i64 - anime_name.len() as i64).abs();
                let score = score as i64 - size_diff;
                matches.push(index, score);
            }
            None => continue,
        };
    }

    return matches.into_sorted_vec();
}

pub fn fetch_best_indexers_for(
    anime_name: &str,
) -> Result<Vec<AnimeIndexer>, ResponseParsingError> {
    let indexers = fetch_indexers()?;
    let sorted_indexers = fuzzy_match_indexers(anime_name, indexers);
    return Ok(sorted_indexers);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! bocchi_the_mock {
        () => {
            LinkedList::from_iter(vec![
                AnimeIndexer {
                    name: "Bocchi the Rock! 2".to_string(),
                    url: "https://kitsunekko.net/bocchi-the-rock-2".to_string(),
                },
                AnimeIndexer {
                    name: "Bocchi the Rock!".to_string(),
                    url: "https://kitsunekko.net/bocchi-the-rock".to_string(),
                },
            ])
        };
    } 

    #[test]
    fn test_fetch_indexers() {
        let anime_list = fetch_indexers().unwrap();
        assert!(anime_list.len() > 0);
        assert!(anime_list.front().unwrap().name.len() > 0);
        assert!(anime_list.front().unwrap().url.len() > 0);

        let contains_bocchi = anime_list
            .iter()
            .any(|anime| anime.name == "Bocchi the Rock!");
        assert!(
            contains_bocchi,
            "Bocchi the Rock! is not in the list of available animes"
        );
    }

    #[test]
    fn test_fuzzy_match_indexers_exact_match_season_1() {
        let mock_anime_list = bocchi_the_mock!();

        let matches = fuzzy_match_indexers("Bocchi the Rock!", mock_anime_list);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].name, "Bocchi the Rock!");
        assert_eq!(matches[1].name, "Bocchi the Rock! 2");
    }

    #[test]
    fn test_fetch_best_indexers_for_relife() {
        let matches = fetch_best_indexers_for("relife").unwrap();
        assert!(matches.len() > 2);
        assert_eq!(matches[0].name, "ReLIFE");
        assert_eq!(matches[1].name, "ReLife Kanketsu Hen");
    }

    #[test]
    fn test_fuzzy_match_indexers_incomplete_equal_score() {
        let mock_anime_list = bocchi_the_mock!();
        let matches = fuzzy_match_indexers("Bocchi", mock_anime_list);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].name, "Bocchi the Rock!");
        assert_eq!(matches[1].name, "Bocchi the Rock! 2");
    }

    #[test]
    fn test_fuzzy_match_indexers_incomplete_better_score_exclusive() {
        let mock_anime_list = bocchi_the_mock!();
        let matches = fuzzy_match_indexers("Bocchi 2", mock_anime_list);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].name, "Bocchi the Rock! 2");
    }
}
