fn main() -> anyhow::Result<()> {Ok(())}


// HACKSAW
#[cfg(test)]
mod tests_gladius_death_or_glory {
    use reels_creator::games::hacksaw::gladius_death_or_glory;
    
    #[test]fn test_extract_base_coins() {gladius_death_or_glory::extract_base_coins();}
    #[test]fn test_extract_base_coin_values() {gladius_death_or_glory::extract_base_coin_values();}
    #[test]fn test_extract_base_bonus() {gladius_death_or_glory::extract_base_bonus();}
    #[test]fn test_extract_base_collector() {gladius_death_or_glory::extract_base_collector();}
    #[test]fn test_extract_base_multyplier() {gladius_death_or_glory::extract_base_multyplier();}
    #[test]fn test_extract_base_multyplier_values() {gladius_death_or_glory::extract_base_multyplier_values();}
}