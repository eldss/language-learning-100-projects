use std::fmt;

const IMPERIAL_CONSTANT: f32 = 703.0;

#[derive(Debug, PartialEq)]
pub enum CalculationError {
    NotPositiveHeight,
    NotPositiveWeight,
}

impl fmt::Display for CalculationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculationError::NotPositiveHeight => f.write_str("Height must be a positive number."),
            CalculationError::NotPositiveWeight => f.write_str("Weight must be a positive number."),
        }
    }
}

/// Calculates BMI based on the input height and weight.
///
/// If is_imperial is false, uses meters and kilograms respectively,
/// otherwise uses inches and pounds.
pub fn calculate_bmi(height: f32, weight: f32, is_imperial: bool) -> Result<f32, CalculationError> {
    if height <= 0.0 {
        return Err(CalculationError::NotPositiveHeight);
    }
    if weight <= 0.0 {
        return Err(CalculationError::NotPositiveWeight);
    }

    let constant = if is_imperial { IMPERIAL_CONSTANT } else { 1.0 };

    Ok(constant * (weight / (height * height)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_bmi_with_not_positive_height_returns_not_positive_height_error() {
        let result = calculate_bmi(-1.0, 1.0, false);
        assert!(result.is_err_and(|error| error == CalculationError::NotPositiveHeight));
    }

    #[test]
    fn calculate_bmi_with_not_positive_weight_returns_not_positive_weight_error() {
        let result = calculate_bmi(1.0, -1.0, false);
        assert!(result.is_err_and(|error| error == CalculationError::NotPositiveWeight));
    }

    #[test]
    fn calculate_bmi_metric_values_returns_correct_output() {
        let metric_cases: Vec<(f32, f32, f32)> = vec![
            (1.50, 45.0, 20.0),
            (1.60, 60.0, 23.43),
            (1.68, 72.5, 25.69),
            (1.75, 80.0, 26.12),
            (1.82, 90.0, 27.17),
        ];
        for (height, weight, expected) in metric_cases {
            let result = calculate_bmi(height, weight, false).expect("calculation should succeed");
            let diff = (expected - result).abs();
            let epsilon = 0.01;
            assert!(
                diff <= epsilon,
                "expected={} actual={} diff={}",
                expected,
                result,
                diff
            );
        }
    }

    #[test]
    fn calculate_bmi_imperial_values_returns_correct_output() {
        let imperial_cases: Vec<(f32, f32, f32)> = vec![
            (60.0, 120.0, 23.44), // 703 * 120 / 60²
            (65.0, 150.0, 24.95),
            (68.0, 140.0, 21.28),
            (70.0, 200.0, 28.69),
            (72.0, 210.0, 28.47),
        ];
        for (height, weight, expected) in imperial_cases {
            let result = calculate_bmi(height, weight, true).expect("calculation should succeed");
            let diff = (expected - result).abs();
            let epsilon = 0.01;
            assert!(
                diff <= epsilon,
                "expected={} actual={} diff={}",
                expected,
                result,
                diff
            );
        }
    }
}
