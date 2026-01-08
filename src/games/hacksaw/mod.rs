pub mod gladius_death_or_glory;

#[cfg(test)]
mod tests_gladius_death_or_glory {
    use crate::games::hacksaw::gladius_death_or_glory;
    
    #[test]fn test_extract_spin_coins() {gladius_death_or_glory::extract_spin_coins();}
    #[test]fn test_extract_spin_coin_cell() {gladius_death_or_glory::extract_spin_coin_cell();}
    #[test]fn test_extract_spin_coin_values() {gladius_death_or_glory::extract_spin_coin_values();}
    #[test]fn test_extract_spin_bonus() {gladius_death_or_glory::extract_spin_bonus();}
    #[test]fn test_extract_spin_collector() {gladius_death_or_glory::extract_spin_collector();}
    #[test]fn test_extract_spin_multypliers() {gladius_death_or_glory::extract_spin_multypliers();}
    
    #[test]fn test_extract_fs_spin_coins() {gladius_death_or_glory::extract_fs_spin_coins();}
    #[test]fn test_extract_fs_spin_coin_cell() {gladius_death_or_glory::extract_fs_spin_coin_cell();}
    #[test]fn test_extract_fs_spin_coin_values() {gladius_death_or_glory::extract_fs_spin_coin_values();}
    #[test]fn test_extract_fs_spin_collector() {gladius_death_or_glory::extract_fs_spin_collector();}
    #[test]fn test_extract_fs_spin_multypliers() {gladius_death_or_glory::extract_fs_spin_multypliers();}
}