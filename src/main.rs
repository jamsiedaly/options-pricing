fn main() {
    println!("Hello, world!");
}

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
    let implied_dividends = future_cash_value - forward_price;
    return DividendYield(implied_dividends);
}

#[cfg(test)]
mod tests {
}