use moon_core::mine::mine_forever;

#[tokio::main]
async fn main() {
    println!("MOON – NODO OFICIAL INICIADO");
    mine_forever().await;
}
