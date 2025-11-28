use moon_wallet::Wallet;

#[tokio::main]
async fn main() {
    println!("CREANDO TU WALLET MOON...\n");

    let wallet = Wallet::new();

    println!("¡WALLET CREADA CON ÉXITO!");
    println!("════════════════════════════════════════════════");
    println!("DIRECCIÓN MOON: {}", wallet.address());
    println!("════════════════════════════════════════════════");
    println!("12 PALABRAS (GUÁRDALAS EN UN LUGAR SEGURO):");
    println!("{}", wallet.mnemonic.phrase());
    println!("════════════════════════════════════════════════");
    println!("¡ERES EL DUEÑO DE MOON!");
}
