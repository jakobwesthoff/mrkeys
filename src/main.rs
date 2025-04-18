mod keygrabber;

#[tokio::main]
async fn main() {
    let mut rx = keygrabber::get_channel();
    loop {
        let event = rx.recv().await.unwrap();
        println!("{:?}", event);
    }
}
