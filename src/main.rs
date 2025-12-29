fn main() -> anyhow::Result<()> {Ok(())}


// HACKSAW
#[cfg(test)]
mod tests_gladius_death_or_glory {
    use reels_creator::games::hacksaw::gladius_death_or_glory;
    
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

//OCTOPLAY
#[cfg(test)]
mod tests_super_grand_link_express_hold_and_win {
    use reels_creator::games::octoplay::super_grand_link_express_hold_and_win;
    
    #[test]fn test_extract_spin_combos() {super_grand_link_express_hold_and_win::extract_spin_combos();}
    #[test]fn test_extract_spin_over_bonus() {super_grand_link_express_hold_and_win::extract_spin_over_bonus();}
    #[test]fn test_extract_spin_coin_values() {super_grand_link_express_hold_and_win::extract_spin_coin_values();}
    #[test]fn test_extract_respin_coin_values() {super_grand_link_express_hold_and_win::extract_respin_coin_values();}
}