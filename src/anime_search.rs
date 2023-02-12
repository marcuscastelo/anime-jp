#[derive(Debug)]
pub struct AnimeSearchResult {
    pub anime_name: String,
    pub anime_raw_magnet: String,
}

pub fn search_anime(anime_name: String) -> AnimeSearchResult {
    println!("Searching for anime: {}", anime_name);
    return AnimeSearchResult{
        anime_name: anime_name,
        anime_raw_magnet: String::from(""),
    };
}

#[test]
fn test_anime_name() {
    let anime_name = String::from("One Piece");
    let result = search_anime(anime_name);
    assert_eq!(result.anime_name, "One Piece");
}

#[test]
fn test_anime_raw_magnet() {
    let anime_name = String::from("One Piece");
    let result = search_anime(anime_name);
    assert_eq!(result.anime_raw_magnet, "");
}