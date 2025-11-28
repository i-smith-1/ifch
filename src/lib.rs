use chrono::{NaiveDate};
use std::f64::consts::E;
use statrs::distribution::{Normal, ContinuousCDF};


// liquidity
pub fn current_r(current_assets: f64, current_liabilities: f64) -> f64 {
    current_assets / current_liabilities
}

pub fn quick_r(current_assets: f64, inventory: f64, current_liabilities: f64) -> f64 {
    (current_assets - inventory) / current_liabilities
}

pub fn cash_r(cash_and_equivalents: f64, current_liabilities: f64) -> f64 {
    cash_and_equivalents / current_liabilities
}

// profitability
pub fn gross_m(gross_profit: f64, revenue: f64) -> f64 {
    gross_profit / revenue
}

pub fn operating_m(operating_income: f64, revenue: f64) -> f64 {
    operating_income / revenue
}

pub fn net_m(net_income: f64, revenue: f64) -> f64 {
    net_income / revenue
}

pub fn r_o_a(net_income: f64, total_assets: f64) -> f64 {
    net_income / total_assets
}

pub fn r_o_e(net_income: f64, shareholders_equity: f64) -> f64 {
    net_income / shareholders_equity
}


// leverage 
pub fn d_t_e(total_debt: f64, shareholders_equity: f64) -> f64 {
    total_debt / shareholders_equity
}

pub fn d_r(total_debt: f64, total_assets: f64) -> f64 {
    total_debt / total_assets
}

pub fn ebit_i_c(ebit: f64, interest_expense: f64) -> f64 {
    ebit / interest_expense
}

// activity 
pub fn inv_t(cost_of_goods_sold: f64, average_inventory: f64) -> f64 {
    cost_of_goods_sold / average_inventory
}

pub fn rec_t(revenue: f64, average_accounts_receivable: f64) -> f64 {
    revenue / average_accounts_receivable
}

pub fn a_t(revenue: f64, total_assets: f64) -> f64 {
    revenue / total_assets
}


// valuation
pub fn p_t_e(share_price: f64, earnings_per_share: f64) -> f64 {
    share_price / earnings_per_share
}

pub fn p_t_b(share_price: f64, book_value_per_share: f64) -> f64 {
    share_price / book_value_per_share
}

pub fn div_y(annual_dividends_per_share: f64, share_price: f64) -> f64 {
    annual_dividends_per_share / share_price
}

// TMV

// assumes that the first outflow is at t=0
pub fn xnpv(cashflows: Vec<(f64, &str)>, discount_rate: f64) -> f64 {
    let mut present_value = 0.0;

    // Parse the date of the first cash flow to use as the start date
    let start_date = NaiveDate::parse_from_str(cashflows[0].1, "%Y-%m-%d").expect("Invalid date format");

    for (cashflow, date_str) in cashflows {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("Invalid date format");
        let days = (date - start_date).num_days() as f64;
        let discount_factor = (1.0 + discount_rate).powf(days / 365.0);
        present_value += cashflow / discount_factor;
    }

    present_value
}

pub fn xirr(cashflows: Vec<(f64, &str)>) -> f64 {
    let mut rate = 0.10; // Initial guess of 5%
    let tolerance = 1e-6;
    let max_iterations = 10000;

    for _ in 0..max_iterations {
        let npv = xnpv(cashflows.clone(), rate);
        let npv_derivative = (xnpv(cashflows.clone(), rate + tolerance) - npv) / tolerance;

        let new_rate = rate - npv / npv_derivative;

        if (new_rate - rate).abs() < tolerance {
            return new_rate;
        }

        rate = new_rate;
    }

    panic!("IRR did not converge");
}

// BSM functions

// Function to calculate N(d1) and N(d2)
pub fn calc_nd(d: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.cdf(d)
}

// Black-Scholes-Merton function
pub fn bsm(
    s: f64,       // Current stock price
    k: f64,       // Option strike price
    t: f64,       // Time to expiration in years
    r: f64,       // Risk-free interest rate
    sigma: f64,   // Volatility
    q: f64
) -> (f64, f64, f64, f64) { // Added fourth return value for put option
    let d1 = (s.ln() - k.ln() + (r - q + sigma.powi(2) / 2.0) * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    let nd1 = calc_nd(d1);
    let nd2 = calc_nd(d2);

    let call_price = s * E.powf(-q * t) * nd1 - k * E.powf(-r * t) * nd2;
    let put_price = k * E.powf(-r * t) * calc_nd(-d2) - s * E.powf(-q * t) * calc_nd(-d1);

    (call_price, put_price, nd1, nd2)
}




// tests

#[cfg(test)]
mod tests {
    use super::*;

    // ratios
    #[test]
    fn test_current_ratio() {
        assert_eq!(current_r(200.0, 100.0), 2.0);
    }

    // TMV
    #[test]
    fn test_present_value_of_cashflows() {
        let cashflows = vec![
            (1000.0, "2026-01-01"),
            (1500.0, "2027-01-01"),
            (2000.0, "2028-01-01"),
        ];
        let discount_rate = 0.05; // 5% annual discount rate

        let pv = xnpv(cashflows, discount_rate);

        // Expected present value calculated manually or with a reliable tool
        let expected_pv = 4242.63;

        assert!((pv - expected_pv).abs() < 0.01); // Allowing a small margin for floating-point precision
    }
    #[test]
    fn test_irr() {
        // Define a set of cash flows (amount, date)
        let cashflows = vec![
            (-1000.0, "2025-01-01"),
            (200.0, "2025-12-31"),
            (300.0, "2026-12-31"),
            (400.0, "2027-12-31"),
            (500.0, "2028-12-31"),
            (600.0, "2029-12-31"),
        ];

        // Calculate the IRR
        let calculated_irr = xirr(cashflows);

        // Expected IRR value (approximately)
        let expected_irr = 0.23300;

        // Assert that the calculated IRR is close to the expected value
        assert!((calculated_irr - expected_irr).abs() < 0.001, "IRR calculation is incorrect");
    }
    #[test]
    fn test_black_scholes() {
        // Define test parameters
        let s = 100.0;      // Current stock price
        let k = 100.0;      // Option strike price
        let t = 1.0;        // Time to expiration in years
        let r = 0.05;       // Risk-free interest rate
        let sigma = 0.2;    // Volatility
        let q = 0.03;       // Dividend Yield

        // Expected results for call and put options
        // These should be calculated using a reliable source or tool
        let expected_call_price = 8.653; // Example value for call
        let expected_put_price = 6.731;   // Example value for put
        let expected_nd1 = 0.579;         // Example value
        let expected_nd2 = 0.500;         // Example value

        // Call the function
        let (call_price, put_price, nd1, nd2) = bsm(s, k, t, r, sigma, q);
        println!("debug:d1 ={}",nd1);
        // Assert results with a small tolerance for floating-point comparisons
        let tolerance = 1e-3;
        assert!((call_price - expected_call_price).abs() < tolerance, "Call price mismatch");
        assert!((put_price - expected_put_price).abs() < tolerance, "Put price mismatch");
        assert!((nd1 - expected_nd1).abs() < tolerance, "N(d1) mismatch");
        assert!((nd2 - expected_nd2).abs() < tolerance, "N(d2) mismatch");
    }
}



