use super::*;
use reqwest::{Client, Proxy};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct Post {
    pub num: String,
}

#[derive(Debug, Deserialize)]
pub struct Catalog {
    pub threads: Vec<Post>,
}

fn host(mirror: Mirror) -> String {
    ["https://", mirror.unwrap(), "/makaba/makaba.fcgi?json=1"].concat()
}

pub async fn task<D: DeserializeOwned>(
    mirror: Mirror,
    proxy: &Proxy,
    data: &Value,
) -> reqwest::Result<D> {
    client::new(proxy)?
        .post(&host(mirror))
        .form(data)
        .send()
        .await?
        .json()
        .await
}

pub async fn task_text(mirror: Mirror, proxy: &Proxy, data: &Value) -> reqwest::Result<String> {
    client::new(proxy)?
        .post(&host(mirror))
        .form(data)
        .send()
        .await?
        .text()
        .await
}

pub async fn can_report(
    mirror: Mirror,
    proxy: &Proxy,
    board: &str,
    thread: &str,
) -> reqwest::Result<bool> {
    let v = task::<Value>(
        mirror,
        proxy,
        &json!({
            "task": "report",
            "board": board,
            "thread": thread,
            "posts": thread,
            "comment": "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Rerum corrupti, distinctio saepe unde sed officiis animi? Dolor laudantium officia odit amet minus, mollitia qui fugiat veniam. Iste numquam rem voluptas."
        }),
    )
    .await?;

    Ok(match v.get("message") {
        Some(v) => v == "",
        None => false,
    })
}

pub async fn url_get<D: DeserializeOwned>(
    mirror: Mirror,
    proxy: &Proxy,
    url: &str,
) -> reqwest::Result<D> {
    client::new(proxy)?
        .get(&["https://", mirror.unwrap(), "/", url].concat())
        .send()
        .await?
        .json()
        .await
}

pub async fn catalog(mirror: Mirror, proxy: &Proxy, board: &str) -> reqwest::Result<Catalog> {
    url_get(mirror, proxy, &[board, "/catalog.json"].concat()).await
}
