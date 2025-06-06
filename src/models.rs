use dioxus::hooks::use_context;
use std::collections::HashMap;
use web_time::Instant;

use serde::Deserialize;

use crate::types::site::Site;

#[derive(Debug, Clone, Deserialize)]
pub struct Article {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub date: Option<String>,
}

impl Article {
    pub async fn fetch_metadata(id: &str, site: &Site) -> Result<Self, ()> {
        let url = format!(
            "/{}/{}/{}/meta.json",
            site.assets.directory, site.assets.articles, id
        );
        let response = gloo_net::http::Request::get(&url)
            .send()
            .await
            .map_err(|_| ())?;
        let article: Self =
            serde_json_wasm::from_str(&response.text().await.map_err(|_| ())?).map_err(|_| ())?;
        Ok(article)
    }

    pub async fn fetch(id: &str, site: &Site) -> Result<(Self, String), ()> {
        let url = format!(
            "/{}/{}/{}/index.md",
            site.assets.directory, site.assets.articles, id
        );
        let response = gloo_net::http::Request::get(&url)
            .send()
            .await
            .map_err(|_| ())?;
        let markdown = response.text().await.map_err(|_| ())?;
        let metadata = Self::fetch_metadata(id, site).await?;
        Ok((metadata, markdown))
    }
}

#[derive(Debug)]
pub struct ArticleIndex {
    pub common: HashMap<String, Article>,
    pub special: HashMap<String, Article>,
}

#[derive(Debug, Clone)]
pub struct SearchableArticle {
    pub id: String,
    pub article: Article,
}

impl PartialEq for SearchableArticle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone)]
pub struct ArticleSearchIndex {
    pub articles: Vec<SearchableArticle>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    create_time: Instant,
}

impl ArticleIndex {
    /// Fetch the article index from the server.
    /// This will load both the common and special articles.
    pub async fn fetch(site: &Site) -> Result<Self, ()> {
        let prefix = format!("/{}/{}", site.assets.directory, site.assets.articles);
        let common_resp = gloo_net::http::Request::get(&format!("{}/index.json", prefix))
            .send()
            .await
            .map_err(|_| ())?;
        let special_resp = gloo_net::http::Request::get(&format!("{}/special.json", prefix))
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

    /// Convert to searchable index
    pub fn to_search_index(&self) -> ArticleSearchIndex {
        let mut articles = Vec::new();
        let mut categories = std::collections::HashSet::new();
        let mut tags = std::collections::HashSet::new();

        // Process common articles,
        // special articles are not included in the search index
        for (id, article) in &self.common {
            articles.push(SearchableArticle {
                id: id.clone(),
                article: article.clone(),
            });

            if let Some(ref category) = article.category {
                categories.insert(category.clone());
            }

            for tag in &article.tags {
                tags.insert(tag.clone());
            }
        }

        // Sort articles by title
        articles.sort_by(|a, b| a.article.title.cmp(&b.article.title));

        ArticleSearchIndex {
            articles,
            categories: categories.into_iter().collect(),
            tags: tags.into_iter().collect(),
            create_time: Instant::now(),
        }
    }
}

impl ArticleSearchIndex {
    /// Search articles by query string (searches title and description)
    pub fn search(&self, query: &str) -> Vec<&SearchableArticle> {
        if query.trim().is_empty() {
            return self.articles.iter().collect();
        }

        let query_lower = query.to_lowercase();
        self.articles
            .iter()
            .filter(|article| {
                article.article.title.to_lowercase().contains(&query_lower)
                    || article
                        .article
                        .description
                        .to_lowercase()
                        .contains(&query_lower)
            })
            .collect()
    }

    /// Filter articles by category
    pub fn filter_by_category(&self, category: &str) -> Vec<&SearchableArticle> {
        self.articles
            .iter()
            .filter(|article| {
                article.article.category.as_ref().map(|c| c.as_str()) == Some(category)
            })
            .collect()
    }

    /// Filter articles by tag
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&SearchableArticle> {
        self.articles
            .iter()
            .filter(|article| article.article.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Get paginated articles
    pub fn paginate<'a>(
        articles: &'a [&'a SearchableArticle],
        page: usize,
        per_page: usize,
    ) -> Vec<&'a SearchableArticle> {
        let start = page * per_page;
        let end = std::cmp::min(start + per_page, articles.len());

        if start >= articles.len() {
            Vec::new()
        } else {
            articles[start..end].to_vec()
        }
    }

    /// Get total number of pages
    pub fn total_pages(total_articles: usize, per_page: usize) -> usize {
        (total_articles + per_page - 1) / per_page
    }
}

impl PartialEq for ArticleSearchIndex {
    /// Compare based on creation time
    fn eq(&self, other: &Self) -> bool {
        self.create_time == other.create_time
    }
}
