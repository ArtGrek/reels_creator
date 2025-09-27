fn main() -> anyhow::Result<()> {Ok(())}


// HACKSAW
#[cfg(test)]
mod tests_gladius_death_or_glory {
    use reels_creator::games::hacksaw::gladius_death_or_glory;
    #[test]fn test_extract_base_reels() {gladius_death_or_glory::extract_base_reels();}
    #[test]fn test_extract_base_coin_values() {gladius_death_or_glory::extract_base_coin_values();}
}