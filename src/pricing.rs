struct ForwardPrice(f64);
struct SpotPrice(f64);
struct InterestRate(f64);
struct TimeToMaturity(f64);
struct DividendYield(f64);

fn calculate_forward_price(
    spot_price: SpotPrice,
    interest_rate: InterestRate,
    time_to_maturity: TimeToMaturity,
    expected_dividend: DividendYield
) -> ForwardPrice {
    let interest_earned = interest_rate.0 * time_to_maturity.0;
    let interest = 1.0 + interest_earned;
    let future_cash_value = spot_price.0 * interest;
    let future_price = future_cash_value - expected_dividend.0;
    return ForwardPrice(future_price);
}

fn calculate_implied_interest(
    forward_price: ForwardPrice,
    spot_price: SpotPrice,
    time_to_maturity: TimeToMaturity,
    expected_dividend: DividendYield
) -> InterestRate {
    let total_future_value = forward_price.0 + expected_dividend.0;
    let margin = (total_future_value / spot_price.0) - 1.0;
    let interest_rate = margin / time_to_maturity.0;
    return InterestRate(interest_rate);
}

fn calculate_implied_spot_price(
    forward_price: ForwardPrice,
    interest_rate: InterestRate,
    time_to_maturity: TimeToMaturity,
    expected_dividend: DividendYield
) -> SpotPrice {
    let total_future_value = forward_price.0 + expected_dividend.0;
    let interest_adjustment = 1.0 + (interest_rate.0 * time_to_maturity.0);
    let price = total_future_value / interest_adjustment;
    return SpotPrice(price);
}

fn calculate_implied_dividend(
    forward_price: ForwardPrice,
    spot_price: SpotPrice,
    interest_rate: InterestRate,
    time_to_maturity: TimeToMaturity
) -> DividendYield {
    let interest_adjustment = 1.0 + (interest_rate.0 * time_to_maturity.0);
    let future_cash_value = spot_price.0 * interest_adjustment;
    let implied_dividends = future_cash_value - forward_price.0;
    return DividendYield(implied_dividends);
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_calculate_forward_price() {
        let spot_price = SpotPrice(67.0);
        let interest_rate = InterestRate(0.0707);
        let time_to_maturity = TimeToMaturity(8.0/12.0);
        let expected_dividend = DividendYield(0.66);
        let forward_price = calculate_forward_price(spot_price, interest_rate, time_to_maturity, expected_dividend).0;
        approx_eq!(f64, forward_price, 69.5, ulps = 5);
    }

    #[test]
    fn test_calculate_implied_interest_rate() {
        let forward_price = ForwardPrice(69.5);
        let spot_price = SpotPrice(67.0);
        let time_to_maturity = TimeToMaturity(8.0/12.0);
        let expected_dividend = DividendYield(0.66);
        let interest_rate = calculate_implied_interest(forward_price, spot_price, time_to_maturity, expected_dividend).0;
        approx_eq!(f64, interest_rate, 0.0707, ulps = 5);
    }

    #[test]
    fn test_calculate_implied_spot_price() {
        let forward_price = ForwardPrice(69.5);
        let interest_rate = InterestRate(0.0707);
        let time_to_maturity = TimeToMaturity(8.0/12.0);
        let expected_dividend = DividendYield(0.66);
        let spot_price = calculate_implied_spot_price(forward_price, interest_rate, time_to_maturity, expected_dividend).0;
        approx_eq!(f64, spot_price, 67.0, ulps = 5);
    }

    #[test]
    fn test_calculate_implied_dividend() {
        let forward_price = ForwardPrice(69.5);
        let spot_price = SpotPrice(67.0);
        let interest_rate = InterestRate(0.0707);
        let time_to_maturity = TimeToMaturity(8.0/12.0);
        let expected_dividend = calculate_implied_dividend(forward_price, spot_price, interest_rate, time_to_maturity).0;
        approx_eq!(f64, expected_dividend, 0.66, ulps = 5);
    }
}