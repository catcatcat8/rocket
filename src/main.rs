#[macro_use]
extern crate rocket;

use rocket::http::uri::Origin;
use rocket::serde::json::{Value};
use rocket::{http::Status, response::Redirect};
use serde::{Deserialize, Serialize};

mod requests;

const CATS_PREFIX: Origin<'static> = uri!("/cats");

#[get("/")]
fn ping() -> &'static str {
    "pong"
}

#[derive(Serialize, Deserialize)]
struct CatFact {
    fact: String,
    length: i64,
}

#[get("/")]
fn index() -> Redirect {
    let max_length: Option<i64> = None;
    let fail: Option<&str> = None;
    Redirect::to(uri!(CATS_PREFIX, fact(max_length, fail)))
}

#[get("/fact?<max_length>&<fail>")]
async fn fact(max_length: Option<i64>, fail: Option<&str>) -> Result<Value, Status> {
    if let Some(_fail) = fail {
        return Err(Status::NotFound);
    }

    let client = reqwest::Client::new();
    requests::request_for_fact(&client, max_length)
        .await
        .or(Err(Status::NoContent))
}

#[get("/?<chain>&<asset_contract_address>&<limit>&<token_ids>&<maker>&<taker>&<order_by>&<order_direction>&<listed_after>&<listed_before>")]
async fn listings(
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
) -> Result<Value, Status> {
    if chain.is_none() {
        return Err(Status::BadRequest);
    } else {
        let client = reqwest::Client::new();
        requests::request_opensea_listings(
            &client,
            chain,
            asset_contract_address,
            limit,
            token_ids,
            maker,
            taker,
            order_by,
            order_direction,
            listed_after,
            listed_before,
        )
        .await
        .or(Err(Status::NoContent))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/listings", routes![listings])
        .mount("/ping", routes![ping])
        .mount(CATS_PREFIX, routes![fact])
}
