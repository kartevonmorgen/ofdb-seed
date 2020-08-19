use ofdb_boundary::*;
use seed::browser::fetch::{fetch, Header, Method, Request, Result};

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
    pub async fn get_places_clearance_with_api_token(
        &self,
        api_token: &str,
    ) -> Result<Vec<PendingClearanceForPlace>> {
        let url = format!("{}/places/clearance", self.url);
        let request = Request::new(url)
            .method(Method::Get)
            .header(Header::bearer(api_token));
        let response = fetch(request).await?;
        let result = response.check_status()?.json().await?;
        Ok(result)
    }
    pub async fn get_place_history_with_api_token(
        &self,
        api_token: &str,
        id: &str,
    ) -> Result<PlaceHistory> {
        let url = format!("{}/places/{}/history", self.url, id);
        let request = Request::new(url)
            .method(Method::Get)
            .header(Header::bearer(api_token));
        let response = fetch(request).await?;
        let result = response
            .check_status()? // ensure we've got 2xx status
            .json()
            .await?;
        Ok(result)
    }
    pub async fn post_places_clearance_with_api_token(
        &self,
        api_token: &str,
        clearances: Vec<ClearanceForPlace>,
    ) -> Result<ResultCount> {
        let url = format!("{}/places/clearance", self.url);
        let request = Request::new(url)
            .method(Method::Post)
            .header(Header::bearer(api_token))
            .json(&clearances)?;
        let response = fetch(request).await?;
        let result = response
            .check_status()? // ensure we've got 2xx status
            .json()
            .await?;
        Ok(result)
    }
}
