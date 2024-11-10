pub fn calculate_price(current_supply: u128, initial_supply: u128, base_price: f64) -> f64 {
    // let initial_supply = 800_000_000_000u128;
    // let base_price = 5.0;

    let remaining_supply = initial_supply.saturating_sub(current_supply);
    let supply_ratio = remaining_supply as f64 / initial_supply as f64;

    let k = 20.0;
    let e = std::f64::consts::E;
    let exponent = k * (1.0 - supply_ratio);

    base_price * e.powf(exponent)
}