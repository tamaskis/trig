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
    fn atan2(&self, other: &f32) -> f32 {
        f32::atan2(*self, *other)
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
    fn deg2rad(&self) -> f32 {
        self * (std::f32::consts::PI / 180.0)
    }
    fn rad2deg(&self) -> f32 {
        self * (180.0 / std::f32::consts::PI)
    }
    fn sind(&self) -> f32 {
        self.deg2rad().sin()
    }
    fn cosd(&self) -> f32 {
        self.deg2rad().cos()
    }
    fn tand(&self) -> f32 {
        self.deg2rad().tan()
    }
    fn cscd(&self) -> f32 {
        self.deg2rad().csc()
    }
    fn secd(&self) -> f32 {
        self.deg2rad().sec()
    }
    fn cotd(&self) -> f32 {
        self.deg2rad().cot()
    }
    fn asind(&self) -> f32 {
        self.asin().rad2deg()
    }
    fn acosd(&self) -> f32 {
        self.acos().rad2deg()
    }
    fn atand(&self) -> f32 {
        self.atan().rad2deg()
    }
    fn atan2d(&self, other: &f32) -> f32 {
        self.atan2(other).rad2deg()
    }
    fn acscd(&self) -> f32 {
        self.acsc().rad2deg()
    }
    fn asecd(&self) -> f32 {
        self.asec().rad2deg()
    }
    fn acotd(&self) -> f32 {
        self.acot().rad2deg()
    }
    fn sinh(&self) -> f32 {
        f32::sinh(*self)
    }
    fn cosh(&self) -> f32 {
        f32::cosh(*self)
    }
    fn tanh(&self) -> f32 {
        f32::tanh(*self)
    }
    fn csch(&self) -> f32 {
        1.0 / self.sinh()
    }
    fn sech(&self) -> f32 {
        1.0 / self.cosh()
    }
    fn coth(&self) -> f32 {
        1.0 / self.tanh()
    }
    fn asinh(&self) -> f32 {
        f32::asinh(*self)
    }
    fn acosh(&self) -> f32 {
        f32::acosh(*self)
    }
    fn atanh(&self) -> f32 {
        f32::atanh(*self)
    }
    fn acsch(&self) -> f32 {
        (1.0 / self).asinh()
    }
    fn asech(&self) -> f32 {
        (1.0 / self).acosh()
    }
    fn acoth(&self) -> f32 {
        (1.0 / self).atanh()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use numtest::*;
    use std::f32::consts::E;

    #[test]
    fn test_sin() {
        assert_eq!(std::f32::consts::FRAC_PI_2.sin(), 1.0);
    }

    #[test]
    fn test_cos() {
        assert_eq!((2.0 * std::f32::consts::PI).cos(), 1.0);
    }

    #[test]
    fn test_tan() {
        assert_eq!(std::f32::consts::FRAC_PI_4.tan(), 1.0);
    }

    #[test]
    fn test_csc() {
        assert_eq!(std::f32::consts::FRAC_PI_2.csc(), 1.0);
    }

    #[test]
    fn test_sec() {
        assert_eq!(std::f32::consts::PI.sec(), -1.0);
    }

    #[test]
    fn test_cot() {
        assert_eq!(std::f32::consts::FRAC_PI_4.cot(), 1.0);
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
        assert_eq!(
            std::f32::consts::FRAC_PI_4.cos().acos(),
            std::f32::consts::FRAC_PI_4
        );
    }

    #[test]
    fn test_atan() {
        assert_eq!(1.0_f32.tan().atan(), 1.0_f32);
    }

    #[test]
    fn test_atan2_45_deg_clockwise() {
        let x = 3.0_f32;
        let y = -3.0_f32;
        let angle_expected = -std::f32::consts::FRAC_PI_4;
        assert_eq!(y.atan2(x), angle_expected);
    }

    #[test]
    fn test_atan2_135_deg_counterclockwise() {
        let x = -3.0_f32;
        let y = 3.0_f32;
        let angle_expected = 3.0 * std::f32::consts::FRAC_PI_4;
        assert_eq!(y.atan2(x), angle_expected);
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
        assert_eq!(
            std::f32::consts::FRAC_PI_4.sec().asec(),
            std::f32::consts::FRAC_PI_4
        );
    }

    #[test]
    fn test_acot() {
        assert_eq!(1.0_f32.cot().acot(), 1.0_f32);
    }

    #[test]
    fn test_deg2rad() {
        assert_eq!(30.0_f32.deg2rad(), std::f32::consts::FRAC_PI_6);
    }

    #[test]
    fn test_rad2deg() {
        assert_eq!(std::f32::consts::FRAC_PI_6.rad2deg(), 30.0);
    }

    #[test]
    fn test_sind() {
        assert_eq!(90.0_f32.sind(), 1.0);
    }

    #[test]
    fn test_cosd() {
        assert_eq!(360.0_f32.cosd(), 1.0);
    }

    #[test]
    fn test_tand() {
        assert_eq!(45.0_f32.tand(), 1.0);
    }

    #[test]
    fn test_cscd() {
        assert_eq!(90.0_f32.cscd(), 1.0);
    }

    #[test]
    fn test_secd() {
        assert_eq!(180.0_f32.secd(), -1.0);
    }

    #[test]
    fn test_cotd() {
        assert_eq!(45.0_f32.cotd(), 1.0);
    }

    #[test]
    fn test_asind() {
        assert_equal_to_atol!(90.0_f32.sind().asind(), 90.0_f32, 1e-5);
    }

    #[test]
    fn test_acosd() {
        assert_eq!(45.0_f32.cosd().acosd(), 45.0_f32);
    }

    #[test]
    fn test_atand() {
        assert_eq!(30.0_f32.tand().atand(), 30.0_f32);
    }

    #[test]
    fn test_atan2d_45_deg_clockwise() {
        let x = 3.0_f32;
        let y = -3.0_f32;
        let angle_expected = -45.0_f32;
        assert_eq!(y.atan2d(&x), angle_expected);
    }

    #[test]
    fn test_atan2d_135_deg_counterclockwise() {
        let x = -3.0_f32;
        let y = 3.0_f32;
        let angle_expected = 135.0_f32;
        assert_eq!(y.atan2d(&x), angle_expected);
    }

    #[test]
    fn test_acscd() {
        assert_equal_to_atol!(90.0_f32.cscd().acscd(), 90.0_f32, 1e-5);
    }

    #[test]
    fn test_asecd() {
        assert_eq!(45.0_f32.secd().asecd(), 45.0_f32);
    }

    #[test]
    fn test_acotd() {
        assert_eq!(30.0_f32.cotd().acotd(), 30.0_f32);
    }

    #[test]
    fn test_sinh() {
        assert_eq!(1.0_f32.sinh(), ((E * E) - 1.0) / (2.0 * E));
    }

    #[test]
    fn test_cosh() {
        assert_equal_to_atol!(1.0_f32.cosh(), ((E * E) + 1.0) / (2.0 * E), 1e-6);
    }

    #[test]
    fn test_tanh() {
        assert_equal_to_atol!(
            1.0_f32.tanh(),
            (1.0 - E.powi(-2)) / (1.0 + E.powi(-2)),
            1e-7
        );
    }

    #[test]
    fn test_csch() {
        assert_equal_to_atol!(1.0_f32.csch(), (2.0 * E) / ((E * E) - 1.0), 1e-7);
    }

    #[test]
    fn test_sech() {
        assert_equal_to_atol!(1.0_f32.sech(), (2.0 * E) / ((E * E) + 1.0), 1e-7);
    }

    #[test]
    fn test_coth() {
        assert_equal_to_atol!(
            1.0_f32.coth(),
            (1.0 + E.powi(-2)) / (1.0 - E.powi(-2)),
            1e-6
        );
    }

    #[test]
    fn test_asinh() {
        assert_equal_to_atol!(1.0_f32.sinh().asinh(), 1.0, 1e-7);
    }

    #[test]
    fn test_acosh() {
        assert_equal_to_atol!(1.0_f32.cosh().acosh(), 1.0, 1e-7);
    }

    #[test]
    fn test_atanh() {
        assert_eq!(1.0_f32.tanh().atanh(), 1.0);
    }

    #[test]
    fn test_acsch() {
        assert_equal_to_atol!(1.0_f32.csch().acsch(), 1.0, 1e-7);
    }

    #[test]
    fn test_asech() {
        assert_equal_to_atol!(0.5_f32.sech().asech(), 0.5, 1e-7);
    }

    #[test]
    fn test_acoth() {
        assert_equal_to_atol!(1.5_f32.coth().acoth(), 1.5, 1e-6);
    }
}
