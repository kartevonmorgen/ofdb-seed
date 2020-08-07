use ofdb_boundary::*;
use seed::browser::fetch::{fetch, Method, Request, Result};

/// OpenFairDB API
#[derive(Debug, Clone)]
pub struct OfdbApi {
    url: String,
}

impl OfdbApi {
    pub fn new(url: String) -> Self {
        Self { url }
    }
    pub async fn search(&self, txt: &str, bbox: &str) -> Result<SearchResponse> {
        let url = format!("{}/search?text={}&bbox={}", self.url, txt, bbox);
        let response = fetch(url).await?;
        response
            .check_status()? // ensure we've got 2xx status
            .json()
            .await
    }
    pub async fn places(&self, ids: &[String]) -> Result<Vec<Entry>> {
        let ids = ids.join(",");
        let url = format!("{}/entries/{}", self.url, ids);
        let response = fetch(url).await?;
        response
            .check_status()? // ensure we've got 2xx status
            .json()
            .await
    }
    pub async fn create_place(&self, place: &NewPlace) -> Result<()> {
        let url = format!("{}/entries", self.url);
        let request = Request::new(url).method(Method::Post).json(place)?;
        let response = fetch(request).await?;
        response.check_status()?; // ensure we've got 2xx status
        Ok(())
    }
    pub async fn update_place(&self, id: &str, place: &UpdatePlace) -> Result<()> {
        let url = format!("{}/entries/{}", self.url, id);
        let request = Request::new(url).method(Method::Put).json(place)?;
        let response = fetch(request).await?;
        response.check_status()?; // ensure we've got 2xx status
        Ok(())
    }
}
