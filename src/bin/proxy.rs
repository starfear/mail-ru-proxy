extern crate sosachnet;

use colored::*;
use rand::{seq::SliceRandom, Rng};
use reqwest::Proxy;
use serde_json::json;
use sosachnet::*;
use std::{
    error::Error,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{fs, prelude::*, spawn, sync::Mutex, time::delay_for};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let super_proxy = Proxy::all("http://127.0.0.1:9050")?;

    let mut proxies = String::new();
    let mut file = fs::File::open("proxies.txt").await?;
    file.read_to_string(&mut proxies).await?;

    let mut lines = proxies.lines();

    let catalog = makaba::catalog(Mirror::Hk, &super_proxy, "b").await?;

    let mut rng = rand::thread_rng();

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    println!("{:?}", "- start!");

    let out = Arc::new(Mutex::new(
        fs::File::create(&format!("{}___good.txt", since_the_epoch.as_secs())).await?,
    ));

    loop {
        delay_for(std::time::Duration::from_millis(10)).await;

        let next = match lines.next() {
            Some(v) => v,
            None => break,
        };

        let url = ["http://", next].concat();

        let r = catalog.threads.choose(&mut rng).unwrap();
        let num = r.num.clone();
        let out = out.clone();

        spawn(async move { test_proxy(url, "b", num, out).await });
    }

    // 5s
    println!("{:?}", "- requests sent... waiting");
    delay_for(std::time::Duration::from_millis(20000)).await;
    println!("{:?}", "- done");

    Ok(())
}

pub async fn test_proxy(mut url: String, board: &str, num: String, out: Arc<Mutex<fs::File>>) {
    let proxy = Proxy::all(&url).expect("failed to build proxy");

    match makaba::can_report(Mirror::Hk, &proxy, board, &num).await {
        Ok(v) => {
            if v == true {
                println!(
                    "{}",
                    "---------------------------------------------------------".blue()
                );
                println!("{} {}", "good".green(), url);
                println!(
                    "{}",
                    "---------------------------------------------------------".blue()
                );
                url.push('\n');
                out.lock()
                    .await
                    .write_all(url.as_bytes())
                    .await
                    .expect("failed to write result");
            } else {
                println!("{} {}", "banned".yellow(), url);
            }
        }

        Err(_) => println!("{} {}", "bad".red(), url),
    };
}
