use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ThemeOptions {
    pub background: String,
}

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
    #[serde(default)]
    pub theme: Option<ThemeOptions>,
    pub author: AuthorOptions,
    pub home: HomeOptions,
    pub articles: ArticlesOptions,
}

impl Site {
    // Fetch site configuration from a JSON file
    pub async fn fetch() -> Result<Self, String> {
        let response = Request::get(&public_url("site.json"))
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
            if let Some(c) = p.chars().next() {
                result.push(c.to_ascii_uppercase())
            }
        }

        result
    }

    pub fn background_url(&self) -> Option<String> {
        self.theme
            .as_ref()
            .map(|theme| public_url(&theme.background))
    }

    pub fn article_asset_url(&self, id: &str, path: &str) -> String {
        public_url(&format!(
            "{}/{}/{}/{}",
            self.assets.directory, self.assets.articles, id, path
        ))
    }
}

/// Resolve an application asset against Trunk's public URL base.
///
/// A relative URL keeps local Trunk serving and GitHub Pages sub-path
/// deployments working without baking a deployment-specific prefix into JSON.
pub fn public_url(path: &str) -> String {
    let path = path.trim_start_matches('/');
    let fallback = format!("/{path}");

    let Some(window) = web_sys::window() else {
        return fallback;
    };
    let Some(document) = window.document() else {
        return fallback;
    };
    let Ok(Some(base)) = document.base_uri() else {
        return fallback;
    };

    web_sys::Url::new_with_base(path, &base)
        .map(|url| url.href())
        .unwrap_or(fallback)
}
