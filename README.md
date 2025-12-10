# Iain's Financial Calculation Helper Rust Project

Welcome to my **Financial Calculation Rust Project**! This project provides a set of tools for performing basic financial calculations using the Rust programming language.

## Features

- **Basic Financial Ratios**: Liquidity, Solvency, Profitability, and Activity.
- **Time Value of Money Calculations**: NPV and IRR for series of cashflows and dates.
- **Black-Scholes-Merton Option Pricing**: Calculate call and put option prices using the Black-Scholes-Merton model, including support for dividend yield.
- **Free Cash Flow to Firm (FCFF) Calculations**: Calculate FCFF using different approaches.
- **Weighted Average Cost of Capital (WACC) Calculations**: Includes functions to calculate WACC and related metrics.
- **Valuation Models**: Gordon Growth Models for valuation.
- **Flexible and Extensible**: Easily extend the library to include additional financial models and calculations.

## Installation

To use this project, you need to have Rust and Cargo installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## Usage

| **Function Name** | **Category**       | **Arguments** | **Result** | **Description**                                                                 |
|-------------------|--------------------|---------------|------------|---------------------------------------------------------------------------------|
| `current_r`       | Liquidity          | `current_assets: f64`, `current_liabilities: f64` | `f64` | Calculates the current ratio: $$\frac{\text{current assets}}{\text{current liabilities}}$$ |
| `quick_r`         | Liquidity          | `current_assets: f64`, `inventory: f64`, `current_liabilities: f64` | `f64` | Calculates the quick ratio: $$\frac{\text{current assets} - \text{inventory}}{\text{current liabilities}}$$ |
| `acid_r`          | Liquidity          | `cash: f64`, `inventory: f64`, `accounts_recievable: f64`, `current_liabilities: f64` | `f64` | Calculates the acid-test (quick) ratio: $$\frac{\text{cash} + \text{inventory} + \text{accounts receivable}}{\text{current liabilities}}$$ |
| `cash_r`          | Liquidity          | `cash_and_equivalents: f64`, `current_liabilities: f64` | `f64` | Calculates the cash ratio: $$\frac{\text{cash and equivalents}}{\text{current liabilities}}$$ |
| `gross_m`         | Profitability      | `gross_profit: f64`, `revenue: f64` | `f64` | Calculates the gross margin: $$\frac{\text{gross profit}}{\text{revenue}}$$ |
| `operating_m`     | Profitability      | `operating_income: f64`, `revenue: f64` | `f64` | Calculates the operating margin: $$\frac{\text{operating income}}{\text{revenue}}$$ |
| `net_m`           | Profitability      | `net_income: f64`, `revenue: f64` | `f64` | Calculates the net margin: $$\frac{\text{net income}}{\text{revenue}}$$ |
| `r_o_a`           | Profitability      | `net_income: f64`, `total_assets: f64` | `f64` | Calculates the return on assets: $$\frac{\text{net income}}{\text{total assets}}$$ |
| `r_o_e`           | Profitability      | `net_income: f64`, `shareholders_equity: f64` | `f64` | Calculates the return on equity: $$\frac{\text{net income}}{\text{shareholders' equity}}$$ |
| `d_t_e`           | Leverage           | `total_debt: f64`, `shareholders_equity: f64` | `f64` | Calculates the debt-to-equity ratio: $$\frac{\text{total debt}}{\text{shareholders' equity}}$$ |
| `d_r`             | Leverage           | `total_debt: f64`, `total_assets: f64` | `f64` | Calculates the debt ratio: $$\frac{\text{total debt}}{\text{total assets}}$$ |
| `ebit_i_c`        | Leverage           | `ebit: f64`, `interest_expense: f64` | `f64` | Calculates the EBIT interest coverage ratio: $$\frac{\text{EBIT}}{\text{interest expense}}$$ |
| `inv_t`           | Activity           | `cost_of_goods_sold: f64`, `average_inventory: f64` | `f64` | Calculates the inventory turnover: $$\frac{\text{cost of goods sold}}{\text{average inventory}}$$ |
| `rec_t`           | Activity           | `revenue: f64`, `average_accounts_receivable: f64` | `f64` | Calculates the receivables turnover: $$\frac{\text{revenue}}{\text{average accounts receivable}}$$ |
| `a_t`             | Activity           | `revenue: f64`, `total_assets: f64` | `f64` | Calculates the asset turnover: $$\frac{\text{revenue}}{\text{total assets}}$$ |
| `p_t_e`           | Valuation          | `share_price: f64`, `earnings_per_share: f64` | `f64` | Calculates the price-to-earnings ratio: $$\frac{\text{share price}}{\text{earnings per share}}$$ |
| `p_t_b`           | Valuation          | `share_price: f64`, `book_value_per_share: f64` | `f64` | Calculates the price-to-book ratio: $$\frac{\text{share price}}{\text{book value per share}}$$ |
| `div_y`           | Valuation          | `annual_dividends_per_share: f64`, `share_price: f64` | `f64` | Calculates the dividend yield: $$\frac{\text{annual dividends per share}}{\text{share price}}$$ |
| `fcff_ni`         | FCFF               | `net_income: f64`, `non_cash_charges: f64`, `interest: f64`, `tax_rate: f64`, `capex: f64`, `change_in_working_capital: f64` | `f64` | Calculates FCFF using net income. |
| `fcff_cfo`        | FCFF               | `cfo: f64`, `interest_expense: f64`, `tax_rate: f64`, `capex: f64` | `f64` | Calculates FCFF using cash flow from operations. |
| `fcff_ebit`       | FCFF               | `ebit: f64`, `tax_rate: f64`, `depreciation: f64`, `capex: f64`, `change_in_working_capital: f64` | `f64` | Calculates FCFF using EBIT. |
| `fcff_ebitda`     | FCFF               | `ebitda: f64`, `tax_rate: f64`, `depreciation: f64`, `capex: f64`, `change_in_working_capital: f64` | `f64` | Calculates FCFF using EBITDA. |
| `wacc_coe`        | WACC               | `coe: f64`, `we: f64`, `tax_rate: f64`, `cod: f64`, `wd: f64`, `cop: f64`, `wp: f64` | `f64` | Calculates WACC using cost of equity. |
| `coe`             | WACC               | `rfr: f64`, `equity_beta: f64`, `mrp: f64` | `f64` | Calculates the cost of equity using CAPM. |
| `wacc_beta`       | WACC               | `equity_beta: f64`, `rfr: f64`, `mrp: f64`, `we: f64`, `tax_rate: f64`, `cod: f64`, `wd: f64`, `cop: f64`, `wp: f64` | `f64` | Calculates WACC using equity beta. |
| `mrp`             | WACC               | `equity_market_return: f64`, `rfr: f64` | `f64` | Calculates the market risk premium. |
| `equity_beta`     | WACC               | `equity: f64`, `debt: f64`, `asset_beta: f64`, `tax_rate: f64` | `f64` | Calculates equity beta. |
| `asset_beta`      | WACC               | `equity: f64`, `debt: f64`, `equity_beta: f64`, `tax_rate: f64` | `f64` | Calculates asset beta. |
| `xnpv`            | TMV                | `cashflows: Vec<(f64, &str)>`, `discount_rate: f64` | `f64` | Calculates the net present value of a series of cash flows given a discount rate. |
| `xirr`            | TMV                | `cashflows: Vec<(f64, &str)>` | `f64` | Calculates the internal rate of return for a series of cash flows. |
| `ggm_p1`          | Valuation Models   | `cashflow_0: f64`, `required_rate_of_return: f64`, `growth_rate: f64` | `Option<f64>` | Calculates the Gordon Growth Model (single-stage). |
| `ggm_p2`          | Valuation Models   | `cashflow_0: f64`, `required_rate_of_return: f64`, `growth_rate_1: f64`, `growth_rate_2: f64`, `periods: u32` | `Option<f64>` | Calculates the Gordon Growth Model (two-stage). |
| `calc_nd`         | BSM                | `d: f64` | `f64` | Calculates the cumulative distribution function for a normal distribution. |
| `bsm`             | BSM                | `s: f64`, `k: f64`, `t: f64`, `r: f64`, `sigma: f64`, `q: f64` | `(f64, f64, f64, f64)` | Calculates the Black-Scholes-Merton option pricing for call and put options, returning call price, put price, N(d1), and N(d2). |

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions or suggestions, please contact me at [i.smith.code@proton.me](mailto:i.smith.code@proton.me).
