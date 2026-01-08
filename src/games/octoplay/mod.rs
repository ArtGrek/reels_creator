pub mod super_grand_link_express_hold_and_win;

#[cfg(test)]
mod tests_super_grand_link_express_hold_and_win {
    use crate::games::octoplay::super_grand_link_express_hold_and_win;
    
    #[test]fn test_extract_spin_combos() {super_grand_link_express_hold_and_win::extract_spin_combos();}
    #[test]fn test_extract_spin_over_bonus() {super_grand_link_express_hold_and_win::extract_spin_over_bonus();}
    #[test]fn test_extract_spin_coin_values() {super_grand_link_express_hold_and_win::extract_spin_coin_values();}
    #[test]fn test_extract_respin_reels() {super_grand_link_express_hold_and_win::extract_respin_reels();}
    #[test]fn test_extract_respin_coin_values() {super_grand_link_express_hold_and_win::extract_respin_coin_values();}
    #[test]fn test_extract_spin_hit() {super_grand_link_express_hold_and_win::extract_spin_hit();}
}