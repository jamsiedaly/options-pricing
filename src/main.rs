fn main() {
    println!("Hello, world!");
}

struct ForwardPrice(f64);
struct StockPrice(f64);
struct InterestRate(f64);
struct TimeToMaturity(f64);
struct DividendYield(f64);

fn calculate_forward_price(
    stock_price: StockPrice,
    interest_rate: InterestRate,
    time_to_maturity: TimeToMaturity,
    expected_dividend: DividendYield
) -> ForwardPrice {
    let interest_earned = interest_rate.0 * time_to_maturity.0;
    let interest = 1.0 + interest_earned;
    let future_cash_value = stock_price.0 * interest;
    let future_price = future_cash_value - expected_dividend.0;
    return ForwardPrice(future_price);
}
