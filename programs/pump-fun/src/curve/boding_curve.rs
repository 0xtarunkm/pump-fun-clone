#[derive(Clone, Debug, Default, PartialEq)]
pub struct BondingCurve;

impl BondingCurve {
    fn calculate_price_original(&self, amount: u128, initial_supply: u128, base_price: f64) -> f64 {
        let remaining_supply = initial_supply.saturating_sub(amount);
        let supply_ratio = remaining_supply as f64 / initial_supply as f64;
        let k = 20.0;
        let e = std::f64::consts::E;
        let exponent = k * (1.0 - supply_ratio);
        base_price * e.powf(exponent)
    }

    pub fn calculate_price_fixed(amount: u128, available_tokens: u128, base_price: f64) -> f64 {
        let initial_supply = available_tokens;
        let current_supply = initial_supply - available_tokens;

        let mut total_cost = 0.0;
        let steps = 10;
        let amount_per_step = amount / steps as u128;

        for i in 0..steps {
            let current_amount = current_supply + (i as u128 * amount_per_step);
            let step_price =
                Self.calculate_price_original(current_amount, initial_supply, base_price);
            total_cost += step_price * (amount_per_step as f64 / 1_000_000.0);
        }

        total_cost
    }
}
