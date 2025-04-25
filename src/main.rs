mod keygrabber;

#[tokio::main()]
async fn main() {
    let mut rx = keygrabber::subscribe().await;
    tokio::spawn(async move {
        loop {
            let event = rx.recv().await.unwrap();
            println!("{:?}", event);
        }
    })
    .await
    .unwrap();
}
