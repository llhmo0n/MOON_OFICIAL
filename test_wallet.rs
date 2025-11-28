use moon_wallet::Wallet;

fn main() {
    let wallet = Wallet::new();
    println!("\nTU WALLET MOON:");
    println!("Dirección: {}", wallet.address());
    println!("Guarda estas 12 palabras:");
    println!("{}", wallet.mnemonic.phrase());
}
