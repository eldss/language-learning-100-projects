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

    #[test]
    fn calculate_bmi_with_zero_height_returns_not_positive_height_error() {
        let result = calculate_bmi(0.0, 70.0, false);
        assert!(result.is_err_and(|error| error == CalculationError::NotPositiveHeight));
    }

    #[test]
    fn calculate_bmi_with_zero_weight_returns_not_positive_weight_error() {
        let result = calculate_bmi(1.75, 0.0, false);
        assert!(result.is_err_and(|error| error == CalculationError::NotPositiveWeight));
    }

    #[test]
    fn calculate_bmi_with_both_inputs_negative_returns_height_error_first() {
        // When both inputs are invalid, height is checked first
        let result = calculate_bmi(-1.0, -1.0, false);
        assert!(result.is_err_and(|error| error == CalculationError::NotPositiveHeight));
    }

    #[test]
    fn calculate_bmi_with_both_inputs_zero_returns_height_error_first() {
        // When both inputs are invalid, height is checked first
        let result = calculate_bmi(0.0, 0.0, false);
        assert!(result.is_err_and(|error| error == CalculationError::NotPositiveHeight));
    }

    #[test]
    fn calculate_bmi_with_very_small_positive_values_succeeds() {
        // Test with very small but valid positive values
        let result = calculate_bmi(0.001, 0.001, false);
        assert!(result.is_ok());
        let bmi = result.unwrap();
        assert!(bmi > 0.0);
        assert!(bmi.is_finite());
    }

    #[test]
    fn calculate_bmi_with_very_large_values_succeeds() {
        // Test with very large but realistic values
        let result = calculate_bmi(2.5, 200.0, false); // 2.5m tall, 200kg
        assert!(result.is_ok());
        let bmi = result.unwrap();
        assert_eq!((bmi * 100.0).round() / 100.0, 32.0);
    }

    #[test]
    fn calculate_bmi_with_extreme_large_values_produces_finite_result() {
        // Test with extremely large values that could cause overflow
        let result = calculate_bmi(10.0, 10000.0, false);
        assert!(result.is_ok());
        let bmi = result.unwrap();
        assert!(bmi.is_finite());
        assert_eq!(bmi, 100.0);
    }

    #[test]
    fn calculate_bmi_metric_underweight_boundary() {
        // BMI exactly at underweight threshold (18.5)
        // Using height 1.70m: weight = 18.5 * 1.70² = 53.465
        let result = calculate_bmi(1.70, 53.465, false);
        assert!(result.is_ok());
        let bmi = result.unwrap();
        assert!((bmi - 18.5).abs() < 0.01, "BMI should be ~18.5, got {}", bmi);
    }

    #[test]
    fn calculate_bmi_metric_normal_weight_boundary() {
        // BMI exactly at normal/overweight threshold (25.0)
        // Using height 1.70m: weight = 25.0 * 1.70² = 72.25
        let result = calculate_bmi(1.70, 72.25, false);
        assert!(result.is_ok());
        let bmi = result.unwrap();
        assert!((bmi - 25.0).abs() < 0.01, "BMI should be ~25.0, got {}", bmi);
    }

    #[test]
    fn calculate_bmi_metric_overweight_boundary() {
        // BMI exactly at overweight/obese threshold (30.0)
        // Using height 1.70m: weight = 30.0 * 1.70² = 86.7
        let result = calculate_bmi(1.70, 86.7, false);
        assert!(result.is_ok());
        let bmi = result.unwrap();
        assert!((bmi - 30.0).abs() < 0.01, "BMI should be ~30.0, got {}", bmi);
    }

    #[test]
    fn calculate_bmi_imperial_underweight_boundary() {
        // BMI exactly at underweight threshold (18.5) in imperial
        // Formula: BMI = 703 * weight / height²
        // Solving for weight: weight = BMI * height² / 703
        // Using height 70in: weight = 18.5 * 70² / 703 = 128.66
        let result = calculate_bmi(70.0, 128.66, true);
        assert!(result.is_ok());
        let bmi = result.unwrap();
        assert!((bmi - 18.5).abs() < 0.05, "BMI should be ~18.5, got {}", bmi);
    }

    #[test]
    fn calculate_bmi_imperial_normal_weight_boundary() {
        // BMI exactly at normal/overweight threshold (25.0) in imperial
        // Formula: BMI = 703 * weight / height²
        // Solving for weight: weight = BMI * height² / 703
        // Using height 70in: weight = 25.0 * 70² / 703 = 173.94
        let result = calculate_bmi(70.0, 173.94, true);
        assert!(result.is_ok());
        let bmi = result.unwrap();
        assert!((bmi - 25.0).abs() < 0.05, "BMI should be ~25.0, got {}", bmi);
    }

    #[test]
    fn calculate_bmi_with_very_similar_values_shows_precision() {
        // Test that small differences in weight are detectable
        let bmi1 = calculate_bmi(1.75, 70.0, false).unwrap();
        let bmi2 = calculate_bmi(1.75, 70.1, false).unwrap();
        assert!(bmi1 != bmi2, "BMI should differ for different weights");
        assert!(bmi2 > bmi1, "Higher weight should produce higher BMI");
    }

    #[test]
    fn calculate_bmi_metric_and_imperial_equivalent_values() {
        // 70 inches = 1.778 meters, 154 pounds = 69.85 kg
        let metric_bmi = calculate_bmi(1.778, 69.85, false).unwrap();
        let imperial_bmi = calculate_bmi(70.0, 154.0, true).unwrap();

        // These should be very close (within rounding error)
        let diff = (metric_bmi - imperial_bmi).abs();
        assert!(
            diff < 0.1,
            "Equivalent metric and imperial values should produce similar BMI. Metric: {}, Imperial: {}, Diff: {}",
            metric_bmi,
            imperial_bmi,
            diff
        );
    }

    #[test]
    fn calculate_bmi_with_minimal_height_difference() {
        // Test that small height differences are detectable
        let bmi1 = calculate_bmi(1.750, 70.0, false).unwrap();
        let bmi2 = calculate_bmi(1.751, 70.0, false).unwrap();
        assert!(bmi1 != bmi2, "BMI should differ for different heights");
        assert!(bmi1 > bmi2, "Smaller height should produce higher BMI");
    }

    #[test]
    fn error_display_not_positive_height() {
        let error = CalculationError::NotPositiveHeight;
        assert_eq!(
            error.to_string(),
            "Height must be a positive number.",
            "Error message should match expected format"
        );
    }

    #[test]
    fn error_display_not_positive_weight() {
        let error = CalculationError::NotPositiveWeight;
        assert_eq!(
            error.to_string(),
            "Weight must be a positive number.",
            "Error message should match expected format"
        );
    }

    #[test]
    fn error_debug_format() {
        let height_error = CalculationError::NotPositiveHeight;
        let weight_error = CalculationError::NotPositiveWeight;

        // Verify Debug trait works (used in error propagation)
        assert_eq!(format!("{:?}", height_error), "NotPositiveHeight");
        assert_eq!(format!("{:?}", weight_error), "NotPositiveWeight");
    }

    #[test]
    fn calculate_bmi_formula_correctness_metric() {
        // Verify the formula: BMI = weight(kg) / height(m)²
        let height = 1.75;
        let weight = 70.0;
        let expected = weight / (height * height);

        let result = calculate_bmi(height, weight, false).unwrap();
        assert_eq!(result, expected, "Metric BMI formula should be exact");
    }

    #[test]
    fn calculate_bmi_formula_correctness_imperial() {
        // Verify the formula: BMI = 703 * weight(lb) / height(in)²
        let height = 70.0;
        let weight = 154.0;
        let expected = 703.0 * weight / (height * height);

        let result = calculate_bmi(height, weight, true).unwrap();
        assert_eq!(result, expected, "Imperial BMI formula should be exact");
    }

    #[test]
    fn calculate_bmi_with_fractional_inputs() {
        // Test with various fractional values to ensure precision
        let test_cases = vec![
            (1.675, 65.5, false),
            (1.823, 78.3, false),
            (68.5, 142.7, true),
            (71.25, 188.6, true),
        ];

        for (height, weight, is_imperial) in test_cases {
            let result = calculate_bmi(height, weight, is_imperial);
            assert!(
                result.is_ok(),
                "Fractional inputs should calculate successfully: h={}, w={}, imperial={}",
                height,
                weight,
                is_imperial
            );
            let bmi = result.unwrap();
            assert!(bmi > 0.0 && bmi.is_finite());
        }
    }
}
