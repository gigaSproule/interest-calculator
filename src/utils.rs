/// Converts a float to a two decimal place float, rounded up.
///
/// # Arguments
///
/// * `value` - A float to ensure is rounded up to two decimal places
///
/// # Examples
///
/// ```
/// use utils::to_currency;
/// let currency = to_currency(1.111);
/// ```
pub(crate) fn to_currency(value: f64) -> f64 {
    (value * 100.0).ceil() / 100.0
}

/// Converts an APR interest rate into a simple mortgage rate.
///
/// # Arguments
///
/// * `apr` - A float representation of the APR interest rate
///
/// # Examples
///
/// ```
/// use utils::to_annual_interest_rate;
/// let annual_interest_rate = to_annual_interest_rate(0.06);
/// ```
pub(crate) fn to_annual_interest_rate(apr: f64) -> f64 {
    (1.0 + apr / 12.0).powf(12.0) - 1.0
}

/// Calculates the monthly interest charge that will be paid.
///
/// # Arguments
///
/// * `principal` - A float of the amount left on the mortgage
/// * `annual_interest_rate` - A float representation of the annual interest rate
///
/// # Examples
///
/// ```
/// use utils::monthly_interest_charge;
/// let monthly_interest_charge = monthly_interest_charge(300000.0, 0.06);
/// ```
pub(crate) fn monthly_interest_charge(principal: f64, annual_interest_rate: f64) -> f64 {
    principal * (annual_interest_rate / 12.0)
}

#[cfg(test)]
mod tests {

    use super::{monthly_interest_charge, to_annual_interest_rate, to_currency};

    #[test]
    fn to_currency_returns_to_2_decimal_places_rounded_up() {
        assert_eq!(to_currency(1.1111), 1.12);
    }

    #[test]
    fn to_annual_interest_rate_converts_properly() {
        assert_eq!(to_annual_interest_rate(0.06), 0.06167781186449828);
    }

    #[test]
    fn monthly_interest_charge_is_calculated_properly() {
        assert_eq!(monthly_interest_charge(300000.0, 0.06), 1500.0);
    }
}
