use crate::{ONE, ZERO};
use rust_decimal::prelude::*;

/// FV - Future Value
///
/// A general future value calculation, similar to the Excel `FV` function.
///
///
/// The future value (FV) is the value of an asset or cash at a specified date in the future based on a certain rate of return.
/// The future value is the amount of money that an investment made today will grow to by a future date.
/// It is calculated by applying a rate of return to the initial investment over a specified period of time.
///
/// # Arguments
/// * `rate` - The interest rate per period
/// * `nper` - The number of compounding periods
/// * `pmt` - The payment amount per period
/// * `pv` (optional) - The present value, default is 0
/// * `due` (optional) - The timing of the payment (false = end of period, true = beginning of period), default is false
/// (ordinary annuity)
///
/// At least one of `pmt` or `pv` should be non-zero.
///
/// # Returns
/// * The future value (FV)
///
/// # Example
/// * 5% interest rate
/// * 10 compounding periods
/// * $100 payment per period
/// ```
/// use rust_finprim::tvm::fv;
/// use rust_decimal_macros::*;
///
/// let rate = dec!(0.05); let nper = dec!(10); let pmt = dec!(-100);
/// fv(rate, nper, pmt, None, None);
/// ```
/// Internal FV calculation with mathematically correct signs
pub fn fv_internal(rate: Decimal, nper: Decimal, pmt: Decimal, pv: Option<Decimal>, due: Option<bool>) -> Decimal {
    let pv = pv.unwrap_or(ZERO);
    let due = due.unwrap_or(false);

    if rate == ZERO {
        // Simplified formula when rate is zero
        return pmt * nper + pv;
    }

    let nth_power = (ONE + rate).powd(nper);
    let factor = (nth_power - ONE) / rate;
    let pv_grown = pv * nth_power;

    if due {
        pmt * factor * (ONE + rate) + pv_grown
    } else {
        pmt * factor + pv_grown
    }
}

/// FV - Future Value (Excel-compatible)
///
/// Excel-compatible FV function that matches Excel's sign convention.
/// In Excel, the result represents the future value you'll receive (typically positive
/// when you make payments/investments).
///
/// # Arguments
/// * `rate` - The interest rate per period
/// * `nper` - The number of compounding periods
/// * `pmt` - The payment amount per period
/// * `pv` (optional) - The present value, default is 0
/// * `due` (optional) - The timing of the payment (false = end of period, true = beginning of period), default is false
///
/// # Returns
/// * The future value (FV) using Excel's sign convention
pub fn fv(rate: Decimal, nper: Decimal, pmt: Decimal, pv: Option<Decimal>, due: Option<bool>) -> Decimal {
    // Calculate the mathematically correct result
    let result = fv_internal(rate, nper, pmt, pv, due);
    
    // Excel uses a sign convention where the result is negated to represent
    // the future value you'll receive rather than the mathematical cash flow result
    -result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(feature = "std"))]
    extern crate std;
    use rust_decimal_macros::*;
    #[cfg(not(feature = "std"))]
    use std::assert;
    #[cfg(not(feature = "std"))]
    use std::prelude::v1::*;

    #[test]
    fn test_fv() {
        struct TestCase {
            rate: Decimal,
            nper: Decimal,
            pmt: Decimal,
            pv: Option<Decimal>,
            due: Option<bool>,
            expected: Decimal,
            description: &'static str,
        }
        impl TestCase {
            fn new(
                rate: f64,
                nper: f64,
                pmt: f64,
                pv: Option<f64>,
                due: Option<bool>,
                expected: f64,
                description: &'static str,
            ) -> TestCase {
                TestCase {
                    rate: Decimal::from_f64(rate).unwrap(),
                    nper: Decimal::from_f64(nper).unwrap(),
                    pmt: Decimal::from_f64(pmt).unwrap(),
                    pv: pv.map(Decimal::from_f64).unwrap_or(None),
                    due,
                    expected: Decimal::from_f64(expected).unwrap(),
                    description,
                }
            }
        }

        let cases = [
            TestCase::new(
                0.05,
                10.0,
                -100.0,
                None,
                None,
                -1257.78925,
                "Standard case with 5% rate, 10 periods, and $100 pmt",
            ),
            TestCase::new(
                0.05,
                10.0,
                -100.0,
                None,
                Some(true),
                -1320.67872,
                "Payment at the beg of period should result in higher future value",
            ),
            TestCase::new(0.0, 10.0, -100.0, None, None, -1000.0, "Zero interest rate no growth"),
            TestCase::new(
                0.05,
                10.0,
                -100.0,
                Some(1000.0),
                None,
                371.10537,
                "Initial investment should result in higher future value",
            ),
            // Microsoft Excel example: FV(0.06/12, 10, -200, -500, 1) = $2,581.40
            // But mathematically it should be -2581.40 (negative because both pmt and pv are negative)
            // Excel uses a different sign convention where the result is negated
            TestCase::new(
                0.06/12.0,
                10.0,
                -200.0,
                Some(-500.0),
                Some(true),
                -2581.4033741,
                "Microsoft Excel example: FV(0.06/12, 10, -200, -500, 1) - mathematically correct result",
            ),
        ];

        for case in &cases {
            let calculated_fv = fv_internal(case.rate, case.nper, case.pmt, case.pv, case.due);
            assert!(
                (calculated_fv - case.expected).abs() < dec!(1e-5),
                "Failed on case: {}. Expected {}, got {}",
                case.description,
                case.expected,
                calculated_fv
            );
        }
    }

    #[test]
    fn test_fv_excel_compatible() {
        // Test Excel-compatible FV function with Microsoft Excel examples
        
        // Microsoft Example 1: FV(0.06/12, 10, -200, -500, 1) = $2,581.40
        let result1 = fv(
            Decimal::from_f64(0.06/12.0).unwrap(),
            Decimal::from_f64(10.0).unwrap(),
            Decimal::from_f64(-200.0).unwrap(),
            Some(Decimal::from_f64(-500.0).unwrap()),
            Some(true)
        );
        assert!(
            (result1 - dec!(2581.40)).abs() < dec!(0.01),
            "Microsoft Example 1 failed. Expected 2581.40, got {}",
            result1
        );

        // Microsoft Example 2: FV(0.12/12, 12, -1000) = $12,682.50
        let result2 = fv(
            Decimal::from_f64(0.12/12.0).unwrap(),
            Decimal::from_f64(12.0).unwrap(),
            Decimal::from_f64(-1000.0).unwrap(),
            None,
            None
        );
        assert!(
            (result2 - dec!(12682.50)).abs() < dec!(0.01),
            "Microsoft Example 2 failed. Expected 12682.50, got {}",
            result2
        );

        // Standard case: FV(0.05, 10, -100) should be positive
        let result3 = fv(
            dec!(0.05),
            dec!(10),
            dec!(-100),
            None,
            None
        );
        assert!(
            (result3 - dec!(1257.78925)).abs() < dec!(0.01),
            "Standard case failed. Expected 1257.78925, got {}",
            result3
        );
    }
}
