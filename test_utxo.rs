use moon_utxo::OptimizedUtxoSet;

fn main() {
    let mut utxo = OptimizedUtxoSet::new("./utxo_test");
    
    // Aquí aplicarías bloques reales
    println!("UTXO Set optimizado listo – ocupa solo 10-20 MB");
    println!("Tu balance actual: {} MOON", utxo.get_balance("TU_DIRECCIÓN_AQUÍ"));
}
