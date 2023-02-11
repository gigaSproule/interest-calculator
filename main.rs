use crate::utils::{monthly_interest_charge, to_annual_interest_rate, to_currency};

mod utils;

fn calculate_monthly_payment(principal: f64, annual_interest_rate: f64, months_left: f64) -> f64 {
    let monthly_interest_rate: f64 = annual_interest_rate / 12.0;
    principal * (monthly_interest_rate * (1.0 + monthly_interest_rate).powf(months_left))
        / ((1.0 + monthly_interest_rate).powf(months_left) - 1.0)
}

fn main() {
    let initial_principal: f64 = 301000.0;
    let initial_term_months: f64 = 300.0;
    let current_principal: f64 = 222765.16;
    let current_annual_interest_rate: f64 = 0.0299;
    let new_annual_interest_rate: f64 = 0.0599;

    /*
    APR Interest Rate
    */
    let apr_converted_interest_rate = to_annual_interest_rate(current_annual_interest_rate);
    let apr_converted_initial_monthly_interest_charge =
        monthly_interest_charge(initial_principal, apr_converted_interest_rate);
    println!(
        "APR converted initial monthly interest charge: £{:.2}",
        to_currency(apr_converted_initial_monthly_interest_charge)
    );
    let apr_converted_monthly_interest_charge =
        monthly_interest_charge(current_principal, apr_converted_interest_rate);
    println!(
        "APR converted monthly interest charge: £{:.2}",
        to_currency(apr_converted_monthly_interest_charge)
    );
    let apr_converted_monthly_payment = calculate_monthly_payment(
        initial_principal,
        apr_converted_interest_rate,
        initial_term_months,
    );
    println!(
        "APR converted monthly payment: £{:.2}",
        to_currency(apr_converted_monthly_payment)
    );
    let apr_converted_monthly_principal =
        apr_converted_monthly_payment - apr_converted_monthly_interest_charge;
    println!(
        "APR converted monthly principal: £{:.2}",
        to_currency(apr_converted_monthly_principal)
    );

    /*
    Initial amounts
    */
    let monthly_payment = calculate_monthly_payment(
        initial_principal,
        current_annual_interest_rate,
        initial_term_months,
    );
    println!("Monthly payment: £{:.2}", to_currency(monthly_payment));
    let initial_monthly_interest_charge =
        monthly_interest_charge(initial_principal, current_annual_interest_rate);
    println!(
        "Initial monthly interest charge: £{:.2}",
        to_currency(initial_monthly_interest_charge)
    );
    let initial_monthly_principal = monthly_payment - initial_monthly_interest_charge;
    println!(
        "Initial monthly principal: £{:.2}",
        to_currency(initial_monthly_principal)
    );

    /*
    Current Interest Rate
    */
    let current_monthly_interest_charge =
        monthly_interest_charge(current_principal, current_annual_interest_rate);
    println!(
        "Current monthly interest charge: £{:.2}",
        to_currency(current_monthly_interest_charge)
    );
    let current_monthly_principal = monthly_payment - current_monthly_interest_charge;
    println!(
        "Current monthly principal: £{:.2}",
        to_currency(current_monthly_principal)
    );

    /*
    New Interest Rate
    */
    let new_monthly_interest_charge =
        monthly_interest_charge(current_principal, new_annual_interest_rate);
    println!(
        "New monthly interest charge: £{:.2}",
        to_currency(new_monthly_interest_charge)
    );
    let new_monthly_principal = current_monthly_principal + new_monthly_interest_charge;
    println!(
        "New monthly principal: £{:.2}",
        to_currency(new_monthly_principal)
    );
}
