use reqwest::{Client};
use rocket::serde::json::{Value};

pub async fn request_for_fact(
    client: &Client,
    max_length: Option<i64>,
) -> Result<Value, reqwest::Error> {
    let url = "https://catfact.ninja/fact";
    let query = vec![("max_length", max_length)];
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());
    headers.insert(
        "X-CSRF-TOKEN",
        "bOoe5ykULsD3NHzovQf9wQIGmfg4vsAwFGLEVPWV".parse().unwrap(),
    );

    let response = client
        .get(url)
        .headers(headers)
        .query(&query)
        .send()
        .await?;
    let fact = response.json::<Value>().await?;
    Ok(fact)
}

pub async fn request_opensea_listings(
    client: &Client,
    chain: Option<&str>,
    asset_contract_address: Option<&str>,
    limit: Option<&str>,
    token_ids: Option<&str>,
    maker: Option<&str>,
    taker: Option<&str>,
    order_by: Option<&str>,
    order_direction: Option<&str>,
    listed_after: Option<&str>,
    listed_before: Option<&str>,
) -> Result<Value, reqwest::Error> {
    let url = format!(
        "https://opensea15.p.rapidapi.com/v2/orders/{}/seaport/listings",
        chain.unwrap().to_string()
    );
    let query = vec![
        ("asset_contract_address", asset_contract_address),
        ("limit", limit),
        ("token_ids", token_ids),
        ("maker", maker),
        ("taker", taker),
        ("order_by", order_by),
        ("order_direction", order_direction),
        ("listed_after", listed_after),
        ("listed_before", listed_before),
    ];
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "X-RapidAPI-Key",
        "5f12d8eb3bmsh214ef05fe9ec8e7p13f5a3jsneffa551725d8"
            .parse()
            .unwrap(),
    );
    headers.insert(
        "X-RapidAPI-Host",
        "opensea15.p.rapidapi.com".parse().unwrap(),
    );
    headers.insert("accept", "application/json".parse().unwrap());
    let response = client
        .get(url)
        .headers(headers)
        .query(&query)
        .send()
        .await?;
    let listings = response.json::<Value>().await?;
    Ok(listings)
}