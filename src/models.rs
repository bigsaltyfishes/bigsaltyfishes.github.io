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

    /// Search articles using SearchCriteria
    pub fn search_with_criteria(&self, criteria: &SearchCriteria) -> Vec<&SearchableArticle> {
        if criteria.is_empty() {
            return self.articles.iter().collect();
        }

        self.articles
            .iter()
            .filter(|article| {
                // Check categories
                if !criteria.categories.is_empty() {
                    let matches_category = if let Some(ref article_category) = article.article.category {
                        criteria.categories.iter().any(|c| c == article_category)
                    } else {
                        false
                    };
                    if !matches_category {
                        return false;
                    }
                }

                // Check tags
                if !criteria.tags.is_empty() {
                    let matches_tag = criteria.tags.iter().any(|search_tag| {
                        article.article.tags.iter().any(|article_tag| article_tag == search_tag)
                    });
                    if !matches_tag {
                        return false;
                    }
                }

                // Check title parts
                if !criteria.title_parts.is_empty() {
                    let title_lower = article.article.title.to_lowercase();
                    let description_lower = article.article.description.to_lowercase();
                    
                    let matches_content = criteria.title_parts.iter().all(|part| {
                        let part_lower = part.to_lowercase();
                        title_lower.contains(&part_lower) || description_lower.contains(&part_lower)
                    });
                    if !matches_content {
                        return false;
                    }
                }

                true
            })
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

#[derive(Debug, Clone, PartialEq)]
pub struct SearchCriteria {
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub title_parts: Vec<String>,
}

impl SearchCriteria {
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
            tags: Vec::new(),
            title_parts: Vec::new(),
        }
    }

    pub fn parse(pattern: &str) -> Self {
        let mut criteria = Self::new();
        let pattern = pattern.trim();

        if pattern.is_empty() {
            return criteria;
        }

        let mut chars = pattern.chars().peekable();
        let mut current_token = String::new();
        let mut in_quotes = false;
        let mut escape_next = false;

        while let Some(ch) = chars.next() {
            if escape_next {
                current_token.push(ch);
                escape_next = false;
                continue;
            }

            match ch {
                '\\' => {
                    escape_next = true;
                }
                '"' => {
                    in_quotes = !in_quotes;
                }
                ' ' | '\t' | '\n' if !in_quotes => {
                    if !current_token.is_empty() {
                        Self::process_token(&mut criteria, current_token);
                        current_token = String::new();
                    }
                }
                _ => {
                    current_token.push(ch);
                }
            }
        }

        // Process the last token
        if !current_token.is_empty() {
            Self::process_token(&mut criteria, current_token);
        }

        criteria
    }

    fn process_token(criteria: &mut SearchCriteria, token: String) {
        if let Some(category) = token.strip_prefix("category:") {
            if !category.is_empty() {
                criteria.categories.push(category.to_string());
            }
        } else if let Some(tag) = token.strip_prefix("tag:") {
            if !tag.is_empty() {
                criteria.tags.push(tag.to_string());
            }
        } else if !token.is_empty() {
            criteria.title_parts.push(token);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.categories.is_empty() && self.tags.is_empty() && self.title_parts.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_criteria_parsing() {
        // Test empty string
        let criteria = SearchCriteria::parse("");
        assert!(criteria.is_empty());

        // Test simple keyword
        let criteria = SearchCriteria::parse("hello");
        assert_eq!(criteria.title_parts, vec!["hello"]);
        assert!(criteria.categories.is_empty());
        assert!(criteria.tags.is_empty());

        // Test category filter
        let criteria = SearchCriteria::parse("category:Technology");
        assert_eq!(criteria.categories, vec!["Technology"]);
        assert!(criteria.title_parts.is_empty());
        assert!(criteria.tags.is_empty());

        // Test tag filter
        let criteria = SearchCriteria::parse("tag:blog");
        assert_eq!(criteria.tags, vec!["blog"]);
        assert!(criteria.title_parts.is_empty());
        assert!(criteria.categories.is_empty());

        // Test quoted strings
        let criteria = SearchCriteria::parse("category:\"Web Development\"");
        assert_eq!(criteria.categories, vec!["Web Development"]);

        // Test complex query
        let criteria = SearchCriteria::parse("category:Technology tag:blog Hello World");
        assert_eq!(criteria.categories, vec!["Technology"]);
        assert_eq!(criteria.tags, vec!["blog"]);
        assert_eq!(criteria.title_parts, vec!["Hello", "World"]);

        // Test quoted keywords
        let criteria = SearchCriteria::parse("\"Hello World\" category:Tech");
        assert_eq!(criteria.title_parts, vec!["Hello World"]);
        assert_eq!(criteria.categories, vec!["Tech"]);

        // Test multiple categories and tags
        let criteria = SearchCriteria::parse("category:Tech category:Science tag:blog tag:tutorial");
        assert_eq!(criteria.categories, vec!["Tech", "Science"]);
        assert_eq!(criteria.tags, vec!["blog", "tutorial"]);

        // Test escaped quotes
        let criteria = SearchCriteria::parse("\"test \\\"quoted\\\" content\"");
        assert_eq!(criteria.title_parts, vec!["test \"quoted\" content"]);
    }
}
