/// Converts a temperature in celsius to fahrenheit.
pub fn c_to_f(celsius: f64) -> f64 {
    celsius * (9.0 / 5.0) + 32.0
}

/// Converts a temperature in fahrenheit to celsius.
pub fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

#[cfg(test)]
mod test {
    // Just super basic tests for a super basic program.
    use super::*;

    #[test]
    fn converts_celsius_to_fahrenheit() {
        let q_and_a: Vec<(f64, f64)> = vec![(0.0, 32.0)];

        for (input, output) in q_and_a {
            let result = c_to_f(input);
            assert_eq!(result, output);
        }
    }

    #[test]
    fn converts_fahrenheit_to_celsius() {
        let q_and_a: Vec<(f64, f64)> = vec![(32.0, 0.0)];

        for (input, output) in q_and_a {
            let result = f_to_c(input);
            assert_eq!(result, output);
        }
    }
}
