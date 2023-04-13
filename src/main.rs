use dotenv::dotenv;
use std::sync::Arc;
use tokio::sync::mpsc;
use ethers::providers::{Provider, Ws, Middleware};
use futures::future::{join_all};
use futures::StreamExt;


#[tokio::main]
async fn main() {

    dotenv().ok();

    let wss = std::env::var( "WSS" ).unwrap();
    println!("{}",wss);
    // example rpc, imagine that WSS are different ;)
    let asia_rpc = &wss;
    let europe_rpc = &wss;
    let america_rpc = &wss;

    // ugly example for the rough idea
    let asia = Arc::new(Provider::<Ws>::connect(asia_rpc).await.unwrap());
    let europe = Arc::new(Provider::<Ws>::connect(europe_rpc).await.unwrap());
    let america = Arc::new(Provider::<Ws>::connect(america_rpc).await.unwrap());

    let (tx, mut rx) = mpsc::channel(32);
    let asia_c = Arc::clone(&asia);
    let tx_asia = tx.clone();
    let asia_handle = tokio::spawn(async move {
        let mut sub = asia_c.subscribe_pending_txs().await.unwrap();

        while let Some(pending) = sub.next().await{
            tx_asia.send(pending).await.unwrap();
            println!("Asia sent!");
        }
    });

    let europe_c = Arc::clone (&europe);
    let europe_tx = tx.clone();
    let europe_handle = tokio::spawn (async move {
        let mut sub = europe_c.subscribe_pending_txs().await.unwrap ( );
        while let Some(pending) = sub.next().await {
            europe_tx.send(pending).await.unwrap();
            println!("Europe sent!");
    }
    });

    let america_c = Arc::clone(&america);
    let america_tx = tx.clone();
    let america_handle = tokio::spawn(async move {
        let mut sub = america_c.subscribe_pending_txs().await.unwrap();
        while let Some(pending) = sub.next().await {
            america_tx.send(pending).await.unwrap(); 
            println!("America sent!");
    }
    });


while let Some(msg) = rx.recv().await {
    println!("{:?}", msg);
}


futures::future::join_all([asia_handle, europe_handle, america_handle]).await;
}