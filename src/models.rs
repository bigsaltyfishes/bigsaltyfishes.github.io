use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Article {
    pub title: String,
    pub description: String,
}

impl Article {
    pub async fn fetch_metadata(id: &str) -> Result<Self, ()> {
        let url = format!("/assets/articles/{}/meta.json", id);
        let response = gloo_net::http::Request::get(&url)
            .send()
            .await
            .map_err(|_| ())?;
        let article: Self =
            serde_json_wasm::from_str(&response.text().await.map_err(|_| ())?).map_err(|_| ())?;
        Ok(article)
    }

    pub async fn fetch(id: &str) -> Result<(Self, String), ()> {
        let url = format!("/assets/articles/{}/index.md", id);
        let response = gloo_net::http::Request::get(&url)
            .send()
            .await
            .map_err(|_| ())?;
        let markdown = response.text().await.map_err(|_| ())?;
        let metadata = Self::fetch_metadata(id).await?;
        Ok((metadata, markdown))
    }
}

#[derive(Debug)]
pub struct ArticleIndex {
    pub common: HashMap<String, Article>,
    pub special: HashMap<String, Article>,
}

impl ArticleIndex {
    /// Fetch the article index from the server.
    /// This will load both the common and special articles.
    pub async fn fetch() -> Result<Self, ()> {
        const PREFIX: &str = "/assets/articles";
        let common_resp = gloo_net::http::Request::get(&format!("{}/index.json", PREFIX))
            .send()
            .await
            .map_err(|_| ())?;
        let special_resp = gloo_net::http::Request::get(&format!("{}/special.json", PREFIX))
            .send()
            .await
            .map_err(|_| ())?;

        Ok(Self {
            common: serde_json_wasm::from_str(&common_resp.text().await.map_err(|_| ())?)
                .map_err(|_| ())?,
            special: serde_json_wasm::from_str(&special_resp.text().await.map_err(|_| ())?)
                .map_err(|_| ())?,
        })
    }

    /// Find an article by its ID.
    pub fn find_article(&self, id: &str) -> Option<&Article> {
        self.common.get(id).or_else(|| self.special.get(id))
    }
}
