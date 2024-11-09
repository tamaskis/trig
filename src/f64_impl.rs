use crate::trig_trait::Trig;

impl Trig for f64 {
    fn sin(&self) -> f64 {
        f64::sin(*self)
    }
    fn cos(&self) -> f64 {
        f64::cos(*self)
    }
    fn tan(&self) -> f64 {
        f64::tan(*self)
    }
    fn csc(&self) -> f64 {
        1.0 / self.sin()
    }
    fn sec(&self) -> f64 {
        1.0 / self.cos()
    }
    fn cot(&self) -> f64 {
        1.0 / self.tan()
    }
    fn asin(&self) -> f64 {
        f64::asin(*self)
    }
    fn acos(&self) -> f64 {
        f64::acos(*self)
    }
    fn atan(&self) -> f64 {
        f64::atan(*self)
    }
    fn acsc(&self) -> f64 {
        (1.0 / self).asin()
    }
    fn asec(&self) -> f64 {
        (1.0 / self).acos()
    }
    fn acot(&self) -> f64 {
        (1.0 / self).atan()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use numtest::*;

    #[test]
    fn test_sin() {
        assert_equal_to_atol!(std::f64::consts::FRAC_PI_2.sin(), 1.0, 1e-16);
    }

    #[test]
    fn test_cos() {
        assert_equal_to_atol!((2.0 * std::f64::consts::PI).cos(), 1.0, 1e-16);
    }

    #[test]
    fn test_tan() {
        assert_equal_to_atol!(std::f64::consts::FRAC_PI_4.tan(), 1.0, 1e-15);
    }

    #[test]
    fn test_csc() {
        assert_equal_to_atol!(std::f64::consts::FRAC_PI_2.csc(), 1.0, 1e-16);
    }

    #[test]
    fn test_sec() {
        assert_equal_to_atol!(std::f64::consts::PI.sec(), -1.0, 1e-16);
    }

    #[test]
    fn test_cot() {
        assert_equal_to_atol!(std::f64::consts::FRAC_PI_4.cot(), 1.0, 1e-15);
    }

    #[test]
    fn test_asin() {
        assert_equal_to_atol!(
            std::f64::consts::FRAC_PI_2.sin().asin(),
            std::f64::consts::FRAC_PI_2,
            1e-16
        );
    }

    #[test]
    fn test_acos() {
        assert_equal_to_atol!(
            std::f64::consts::FRAC_PI_4.cos().acos(),
            std::f64::consts::FRAC_PI_4,
            1e-16
        );
    }

    #[test]
    fn test_atan() {
        assert_equal_to_atol!(1.0_f64.tan().atan(), 1.0_f64, 1e-16);
    }

    #[test]
    fn test_acsc() {
        assert_equal_to_atol!(
            std::f64::consts::FRAC_PI_2.csc().acsc(),
            std::f64::consts::FRAC_PI_2,
            1e-16
        );
    }

    #[test]
    fn test_asec() {
        assert_equal_to_atol!(
            std::f64::consts::FRAC_PI_4.sec().asec(),
            std::f64::consts::FRAC_PI_4,
            1e-16
        );
    }

    #[test]
    fn test_acot() {
        assert_equal_to_atol!(1.0_f64.cot().acot(), 1.0_f64, 1e-16);
    }
}
