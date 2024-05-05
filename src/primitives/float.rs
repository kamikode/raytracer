// Choose between f32 and f64.
pub type Float = f64;

macro_rules! approx_eq {
    ($x:expr, $y:expr) => {
        Float::abs($x - $y) <= 1e-8 + 1e-5 * Float::abs($y)
    };
    ($x:expr, $y:expr, atol = $atol:expr) => {
        Float::abs($x - $y) <= $atol + 1e-5 * Float::abs($y)
    };
    ($x:expr, $y:expr, rtol = $rtol:expr) => {
        Float::abs($x - $y) <= 1e-8 + $rtol * Float::abs($y)
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
    fn approx_eq() {
        // These should all return true.
        assert!(approx_eq!(1e-9, 0.0));
        assert!(approx_eq!(1.0 + 1e-6, 1.0));
        assert!(approx_eq!(1e-6, 0.0, atol = 1e-5));
        assert!(approx_eq!(1.0 + 1e-3, 1.0, rtol = 1e-2));
        assert!(approx_eq!(1e-6, 0.0, atol = 1e-5, rtol = 0.0));
        assert!(approx_eq!(1.0 + 1e-3, 1.0, rtol = 1e-2, atol = 0.0));
        // These should all return false.
        assert!(!approx_eq!(1e-7, 0.0));
        assert!(!approx_eq!(1.0 + 1e-4, 1.0));
    }
}
