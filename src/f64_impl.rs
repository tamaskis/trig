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
    fn deg2rad(&self) -> f64 {
        self * (std::f64::consts::PI / 180.0)
    }
    fn rad2deg(&self) -> f64 {
        self * (180.0 / std::f64::consts::PI)
    }
    fn sind(&self) -> f64 {
        self.deg2rad().sin()
    }
    fn cosd(&self) -> f64 {
        self.deg2rad().cos()
    }
    fn tand(&self) -> f64 {
        self.deg2rad().tan()
    }
    fn cscd(&self) -> f64 {
        self.deg2rad().csc()
    }
    fn secd(&self) -> f64 {
        self.deg2rad().sec()
    }
    fn cotd(&self) -> f64 {
        self.deg2rad().cot()
    }
    fn asind(&self) -> f64 {
        self.asin().rad2deg()
    }
    fn acosd(&self) -> f64 {
        self.acos().rad2deg()
    }
    fn atand(&self) -> f64 {
        self.atan().rad2deg()
    }
    fn acscd(&self) -> f64 {
        self.acsc().rad2deg()
    }
    fn asecd(&self) -> f64 {
        self.asec().rad2deg()
    }
    fn acotd(&self) -> f64 {
        self.acot().rad2deg()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use numtest::*;

    #[test]
    fn test_sin() {
        assert_eq!(std::f64::consts::FRAC_PI_2.sin(), 1.0);
    }

    #[test]
    fn test_cos() {
        assert_eq!((2.0 * std::f64::consts::PI).cos(), 1.0);
    }

    #[test]
    fn test_tan() {
        assert_equal_to_atol!(std::f64::consts::FRAC_PI_4.tan(), 1.0, 1e-15);
    }

    #[test]
    fn test_csc() {
        assert_eq!(std::f64::consts::FRAC_PI_2.csc(), 1.0);
    }

    #[test]
    fn test_sec() {
        assert_eq!(std::f64::consts::PI.sec(), -1.0);
    }

    #[test]
    fn test_cot() {
        assert_equal_to_atol!(std::f64::consts::FRAC_PI_4.cot(), 1.0, 1e-15);
    }

    #[test]
    fn test_asin() {
        assert_eq!(
            std::f64::consts::FRAC_PI_2.sin().asin(),
            std::f64::consts::FRAC_PI_2
        );
    }

    #[test]
    fn test_acos() {
        assert_eq!(
            std::f64::consts::FRAC_PI_4.cos().acos(),
            std::f64::consts::FRAC_PI_4
        );
    }

    #[test]
    fn test_atan() {
        assert_eq!(1.0_f64.tan().atan(), 1.0_f64);
    }

    #[test]
    fn test_acsc() {
        assert_eq!(
            std::f64::consts::FRAC_PI_2.csc().acsc(),
            std::f64::consts::FRAC_PI_2
        );
    }

    #[test]
    fn test_asec() {
        assert_eq!(
            std::f64::consts::FRAC_PI_4.sec().asec(),
            std::f64::consts::FRAC_PI_4
        );
    }

    #[test]
    fn test_acot() {
        assert_eq!(1.0_f64.cot().acot(), 1.0_f64);
    }

    #[test]
    fn test_deg2rad() {
        assert_equal_to_atol!(30.0_f64.deg2rad(), std::f64::consts::FRAC_PI_6, 1e-15);
    }

    #[test]
    fn test_rad2deg() {
        assert_equal_to_atol!(std::f64::consts::FRAC_PI_6.rad2deg(), 30.0, 1e-14);
    }

    #[test]
    fn test_sind() {
        assert_eq!(90.0_f64.sind(), 1.0);
    }

    #[test]
    fn test_cosd() {
        assert_eq!(360.0_f64.cosd(), 1.0);
    }

    #[test]
    fn test_tand() {
        assert_equal_to_atol!(45.0_f64.tand(), 1.0, 1e-15);
    }

    #[test]
    fn test_cscd() {
        assert_eq!(90.0_f64.cscd(), 1.0);
    }

    #[test]
    fn test_secd() {
        assert_eq!(180.0_f64.secd(), -1.0);
    }

    #[test]
    fn test_cotd() {
        assert_equal_to_atol!(45.0_f64.cotd(), 1.0, 1e-15);
    }

    #[test]
    fn test_asind() {
        assert_eq!(90.0_f64.sind().asind(), 90.0_f64);
    }

    #[test]
    fn test_acosd() {
        assert_eq!(45.0_f64.cosd().acosd(), 45.0_f64);
    }

    #[test]
    fn test_atand() {
        assert_equal_to_atol!(30.0_f64.tand().atand(), 30.0_f64, 1e-14);
    }

    #[test]
    fn test_acscd() {
        assert_eq!(90.0_f64.cscd().acscd(), 90.0_f64);
    }

    #[test]
    fn test_asecd() {
        assert_eq!(45.0_f64.secd().asecd(), 45.0_f64);
    }

    #[test]
    fn test_acotd() {
        assert_equal_to_atol!(30.0_f64.cotd().acotd(), 30.0_f64, 1e-14);
    }
}
