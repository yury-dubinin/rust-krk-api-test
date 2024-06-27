use cucumber::{ then, when, World as _};
use reqwest::Client;
use tokio::time::sleep;
use std::time::Duration;
use serde_json::Value;
use std::env;
use base64::{engine::general_purpose::STANDARD as base64_engine, Engine as _};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(cucumber::World, Debug, Default)]
pub struct ApiWorld {
    client: Client,

    response_status_time: Option<u16>,
    response_body_time: Option<String>,
    response_status_pair: Option<u16>,
    response_body_pair: Option<String>,
    response_status_orders: Option<u16>,
    response_body_orders: Option<String>,
}

#[when("I get the XBTUSD trading pair")]
pub async fn i_get_the_trading_pair(world: &mut ApiWorld) {
    sleep(Duration::from_secs(2)).await;
    let response = world
        .client
        .get(format!("https://api.kraken.com/0/public/Ticker?pair={}", "XBTUSD"))
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    let body = response.text().await.unwrap();
    world.response_status_pair = Some(status);
    world.response_body_pair = Some(body);
}

#[when("I get the server time")]
pub async fn i_get_the_time(world: &mut ApiWorld) {
    sleep(Duration::from_secs(1)).await;
    let response = world
        .client
        .get("https://api.kraken.com/0/public/Time")
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    let body_time = response.text().await.unwrap();
    world.response_status_time = Some(status);
    world.response_body_time = Some(body_time);
}

#[when("I get all open orders")]
pub async fn i_get_the_orders(world: &mut ApiWorld) {
    let api_key = env::var("API_KEY").unwrap_or("+API_KEY".to_string());
    let secret = env::var("PRIVATE_KEY").unwrap_or("+PRIVATE_KEY".to_string());
    let urlpath = "/0/private/OpenOrders";
    // nonce from compute 1719502312486, must be > 1616492376594003
    let nonce = compute_nonce() * 1000;
    println!("nonce -> {}",nonce);
    let sign = compute_signature(&secret, urlpath, &nonce).expect("No signature!");
    let response = world
        .client
        .post("https://api.kraken.com/0/private/OpenOrders")
        .header("API-Key", api_key)
        .header("API-Sign", sign)
        .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
        .body(format!("nonce={}", nonce))
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    let body_orders = response.text().await.unwrap();
    assert_eq!(status, 200);
    world.response_status_orders = Some(status);
    world.response_body_orders = Some(body_orders);
}

#[then("all open orders should be retrieved")]
pub async fn the_orders_should_be_retrieved(world: &mut ApiWorld) {
    let status = world.response_status_orders.unwrap();
    let response_body = world.response_body_orders.as_ref().unwrap();
    let obj: Value = serde_json::from_str(response_body).expect("Parse has failed!");
    let err: Option<&Value> = obj.get("error");
    let _res: Option<&Value> = obj.get("result");
    // Assert no errors and status is 200
    assert_eq!(option_value_to_string(err), "[]");
    assert_eq!(status, 200);
    //println!("Result: {:?}", res);
    println!("Get orders response_body: {}", serde_json::to_string_pretty(&obj).unwrap());
}

#[then("the XBTUSD information should be retrieved")]
pub async fn the_trading_pair_should_be_retrieved(world: &mut ApiWorld) {
    let status = world.response_status_pair.unwrap();
    let response_body = world.response_body_pair.as_ref().unwrap();
    let obj: Value = serde_json::from_str(response_body).expect("REASON");
    let err: Option<&Value> = obj.get("error");
    let _res: Option<&Value> = obj.get("result");
    // Assert no errors and status is 200
    assert_eq!(option_value_to_string(err), "[]");
    assert_eq!(status, 200);
    //println!("Result: {:?}", res);
    println!("XBTUSD response_body: {}", serde_json::to_string_pretty(&obj).unwrap());
}

#[then("the server time should be retrieved")]
pub async fn the_server_time_should_be_retrieved(world: &mut ApiWorld) {
    let status = world.response_status_time.unwrap();
    let response_body = world.response_body_time.as_ref().unwrap();
    let objt: Value = serde_json::from_str(response_body).expect("Parse has failed!");
    let errt: Option<&Value> = objt.get("error");
    let _rest: Option<&Value> = objt.get("result");
    // Assert no errors and status is 200
    assert_eq!(option_value_to_string(errt), "[]");
    assert_eq!(status, 200);
    //println!("Result: {:?}", rest);
    println!("time response_body: {}", serde_json::to_string_pretty(&objt).unwrap());
}

#[tokio::main]
async fn main() {
    ApiWorld::run("tests/features/api.features").await;
}

fn option_value_to_string(opt_value: Option<&Value>) -> String {
    match opt_value {
        Some(value) => value.to_string(),
        None => String::new(),
    }
}

fn sha256(input: String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().to_vec()
}

fn sha512(input: Vec<u8>, secret: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut mac = Hmac::<Sha512>::new_from_slice(secret)?;
    mac.update(&input);
    Ok(mac.finalize().into_bytes().to_vec())
}

pub(crate) fn compute_nonce() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.as_millis() as u64
}

pub fn compute_signature(
    api_secret: &str,
    path: &str,
    nonce: &u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut query = String::from("");
    let n = nonce.to_string();
    query.push_str(&n);
    let ns = format!("nonce={}", nonce);
    query.push_str(&ns);

    let mut sha256_res = sha256(query);

    let mut to_hash = vec![];

    to_hash.append(&mut path.as_bytes().to_owned());
    to_hash.append(&mut sha256_res);

    let secret = base64_engine.decode(api_secret)?;
    let sha512_res = sha512(to_hash, &secret)?;

    Ok(base64_engine.encode(sha512_res))
}
