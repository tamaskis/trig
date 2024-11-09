use crate::trig_trait::Trig;

impl Trig for f32 {
    fn sin(&self) -> f32 {
        f32::sin(*self)
    }
    fn cos(&self) -> f32 {
        f32::cos(*self)
    }
    fn tan(&self) -> f32 {
        f32::tan(*self)
    }
    fn csc(&self) -> f32 {
        1.0 / self.sin()
    }
    fn sec(&self) -> f32 {
        1.0 / self.cos()
    }
    fn cot(&self) -> f32 {
        1.0 / self.tan()
    }
    fn asin(&self) -> f32 {
        f32::asin(*self)
    }
    fn acos(&self) -> f32 {
        f32::acos(*self)
    }
    fn atan(&self) -> f32 {
        f32::atan(*self)
    }
    fn acsc(&self) -> f32 {
        (1.0 / self).asin()
    }
    fn asec(&self) -> f32 {
        (1.0 / self).acos()
    }
    fn acot(&self) -> f32 {
        (1.0 / self).atan()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use numtest::*;

    #[test]
    fn test_sin() {
        assert_equal_to_atol!(std::f32::consts::FRAC_PI_2.sin(), 1.0, 1e-7);
    }

    #[test]
    fn test_cos() {
        assert_equal_to_atol!((2.0 * std::f32::consts::PI).cos(), 1.0, 1e-7);
    }

    #[test]
    fn test_tan() {
        assert_equal_to_atol!(std::f32::consts::FRAC_PI_4.tan(), 1.0, 1e-7);
    }

    #[test]
    fn test_csc() {
        assert_equal_to_atol!(std::f32::consts::FRAC_PI_2.csc(), 1.0, 1e-7);
    }

    #[test]
    fn test_sec() {
        assert_equal_to_atol!(std::f32::consts::PI.sec(), -1.0, 1e-7);
    }

    #[test]
    fn test_cot() {
        assert_equal_to_atol!(std::f32::consts::FRAC_PI_4.cot(), 1.0, 1e-7);
    }

    #[test]
    fn test_asin() {
        assert_equal_to_atol!(
            std::f32::consts::FRAC_PI_2.sin().asin(),
            std::f32::consts::FRAC_PI_2,
            1e-6
        );
    }

    #[test]
    fn test_acos() {
        assert_equal_to_atol!(
            std::f32::consts::FRAC_PI_4.cos().acos(),
            std::f32::consts::FRAC_PI_4,
            1e-7
        );
    }

    #[test]
    fn test_atan() {
        assert_equal_to_atol!(1.0_f32.tan().atan(), 1.0_f32, 1e-7);
    }

    #[test]
    fn test_acsc() {
        assert_equal_to_atol!(
            std::f32::consts::FRAC_PI_2.csc().acsc(),
            std::f32::consts::FRAC_PI_2,
            1e-6
        );
    }

    #[test]
    fn test_asec() {
        assert_equal_to_atol!(
            std::f32::consts::FRAC_PI_4.sec().asec(),
            std::f32::consts::FRAC_PI_4,
            1e-7
        );
    }

    #[test]
    fn test_acot() {
        assert_equal_to_atol!(1.0_f32.cot().acot(), 1.0_f32, 1e-7);
    }
}
