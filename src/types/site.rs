use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Clone)]
pub struct SiteContext(pub Site); // true for dark mode

#[derive(Debug, Clone, Deserialize)]
pub struct Site {
    pub name: String,
}

impl Site {
    // Fetch site configuration from a JSON file
    pub async fn fetch() -> Result<Self, Box<dyn std::error::Error>> {
        let response = Request::get("/assets/site.json")
            .send()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        let text = response
            .text()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        let site = serde_json_wasm::from_str(&text)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
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

impl Default for Site {
    fn default() -> Self {
        Self {
            name: "Default Site".to_string(),
        }
    }
}
