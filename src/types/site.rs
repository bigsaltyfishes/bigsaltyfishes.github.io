use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AssetsOptions {
    pub directory: String,
    pub articles: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AuthorOptions {
    pub name: String,
    pub email: String,
    pub github: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct HomeOptions {
    pub welcome_title: String,
    #[serde(default)]
    pub welcome_text: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ArticlesOptions {
    pub maximum_number_per_page: usize,
    pub pagination_size: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Site {
    pub name: String,
    pub copyright_year: u16,
    pub assets: AssetsOptions,
    pub author: AuthorOptions,
    pub home: HomeOptions,
    pub articles: ArticlesOptions,
}

impl Site {
    // Fetch site configuration from a JSON file
    pub async fn fetch() -> Result<Self, String> {
        let response = Request::get("/site.json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        let text = response
            .text()
            .await
            .map_err(|e| format!("Failed to get text: {}", e))?;
        let site =
            serde_json_wasm::from_str(&text).map_err(|e| format!("Failed to parse JSON: {}", e))?;
        Ok(site)
    }

    pub fn long(&self) -> String {
        self.name.clone()
    }

    pub fn short(&self) -> String {
        let mut result = String::new();
        for p in self.name.split_whitespace() {
            p.chars()
                .next()
                .map(|c| result.push(c.to_ascii_uppercase()));
        }

        result
    }
}
