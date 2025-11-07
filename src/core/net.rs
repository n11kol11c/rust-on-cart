use crate::errors::error::CartError;
use reqwest;

pub async fn fetch_url(url: &str) -> Result<String, CartError> {
    let resp = reqwest::get(url)
        .await
        .map_err(|e| CartError::NetworkError(e.to_string()))?;
    
    let body = resp
        .text()
        .await
        .map_err(|e| CartError::NetworkError(e.to_string()))?;
    
    Ok(body)
}
