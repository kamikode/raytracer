// Choose between f32 and f64.
pub type Float = f64;

#[cfg(test)]
macro_rules! approx_eq {
    ($x:expr, $y:expr) => {
        Float::abs($x - $y) <= Float::EPSILON.sqrt() + 1e-5 * Float::abs($y)
    };
    ($x:expr, $y:expr, atol = $atol:expr) => {
        Float::abs($x - $y) <= $atol + 1e-5 * Float::abs($y)
    };
    ($x:expr, $y:expr, rtol = $rtol:expr) => {
        Float::abs($x - $y) <= Float::EPSILON.sqrt() + $rtol * Float::abs($y)
    };
    ($x:expr, $y:expr, atol = $atol:expr, rtol = $rtol:expr) => {
        Float::abs($x - $y) <= $atol + $rtol * Float::abs($y)
    };
    ($x:expr, $y:expr, rtol = $rtol:expr, atol = $atol:expr) => {
        Float::abs($x - $y) <= $atol + $rtol * Float::abs($y)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approx_eq_is_true_when_values_are_close() {
        assert!(approx_eq!(1e-9, 0.0));
        assert!(approx_eq!(1.0 + 1e-6, 1.0));
        assert!(approx_eq!(1e-6, 0.0, atol = 1e-5));
        assert!(approx_eq!(1.0 + 1e-3, 1.0, rtol = 1e-2));
        assert!(approx_eq!(1e-6, 0.0, atol = 1e-5, rtol = 0.0));
        assert!(approx_eq!(1.0 + 1e-3, 1.0, rtol = 1e-2, atol = 0.0));
    }

    #[test]
    #[should_panic]
    fn approx_eq_is_false_when_difference_exceeds_atol() {
        assert!(approx_eq!(1e-7, 0.0)); // Should panic.
    }

    #[test]
    #[should_panic]
    fn approx_eq_is_false_when_difference_exceeds_rtol() {
        assert!(approx_eq!(1.0 + 1e-4, 1.0)); // Should panic.
    }
}
