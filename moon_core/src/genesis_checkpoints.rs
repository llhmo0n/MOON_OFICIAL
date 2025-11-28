pub const GENESIS_CHECKPOINTS: &[(&str, u64)] = &[
    ("0000000000000000000000000000000000000000000000000000000000000000", 0),   // bloque 0
    ("00000d0016050fad8b1e38f98fe98e88d8c98e800fd5ba0fb9c806b86371fba1", 1860), // tu último hash real
];

pub fn is_valid_checkpoint(height: u64, hash: &str) -> bool {
    GENESIS_CHECKPOINTS.iter().any(|&(h, ht)| ht == height && h == hash)
}
