use regex::{Regex, CaptureMatches};

pub trait ScrapperData {
    fn from_captures(capture: CaptureMatches) -> Vec<Self> where Self: Sized;
}
pub struct HttpScrapper<T> where T: ScrapperData {
    inner_regex: Regex,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> HttpScrapper<T> where T: ScrapperData {
    pub fn new(inner_regex: Regex) -> Self {
        HttpScrapper { 
            inner_regex,
            _phantom: std::marker::PhantomData,
         }
    }

    pub fn fetch_raw_data(&self, uri: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = reqwest::blocking::get(uri)?;
        let response_text = response.text()?;
        Ok(response_text)
    }

    pub fn scrap_raw_data(&self, data: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        let capture_matches = self.inner_regex.captures_iter(&data);
        let data = T::from_captures(capture_matches);
        Ok(data)
    }

    pub fn scrap_page(&self, uri: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        let raw_data = self.fetch_raw_data(uri)?;
        let data = self.scrap_raw_data(&raw_data)?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::{Regex, HttpScrapper, ScrapperData, CaptureMatches};

    #[derive(Debug, PartialEq, Eq)]
    struct FullMatch {
        data: String,
    }

    impl FullMatch {
        fn new(data: &str) -> Self {
            FullMatch { data: String::from(data) }
        }
    }

    impl ScrapperData for FullMatch {
        fn from_captures(capture: CaptureMatches) -> Vec<Self> where Self: Sized {
            capture.map(|capture| {
                let data = capture[0].to_string();
                FullMatch { data }
            }).collect()
        }
    }

    #[test]
    fn keeps_inner_regex() {
        let inner_regex = Regex::new(r"1234").unwrap();
        let scrapper = HttpScrapper::<FullMatch>::new(inner_regex);
        assert_eq!(scrapper.inner_regex.as_str(), r"1234");
    }

    #[test]
    fn scrapes_page() {
        let inner_regex = Regex::new(r"google").unwrap();
        let scrapper = HttpScrapper::<FullMatch>::new(inner_regex);
        let data = scrapper.scrap_page("https://www.google.com").unwrap();
        assert!(data.contains(&FullMatch::new("google")));
        assert!(!data.contains(&FullMatch::new("search")));
    }
}