/// Trigonometric functions.
pub trait Trig {
    /// Computes the sine of this value, where this value is in radians.
    ///
    /// # Returns
    ///
    /// Sine of this value.
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$ _radians_
    ///
    /// # Range
    ///
    /// $[-1,1]$
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = std::f64::consts::FRAC_PI_2;
    /// let abs_difference = (x.sin() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/Sine.html](https://mathworld.wolfram.com/Sine.html)
    fn sin(&self) -> Self;

    /// Computes the cosine of this value, where this value is in radians.
    ///
    /// # Returns
    ///
    /// Cosine of this value.
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$ _radians_
    ///
    /// # Range
    ///
    /// $[-1,1]$
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = 2.0 * std::f64::consts::PI;
    /// let abs_difference = (x.cos() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/Cosine.html](https://mathworld.wolfram.com/Cosine.html)
    fn cos(&self) -> Self;

    /// Computes the tangent of this value, where this value is in radians.
    ///
    /// # Returns
    ///
    /// Tangent of this value.
    ///
    /// # Domain
    ///
    /// $\mathbb{R}\\;\setminus\\;\left\\{\frac{\pi}{2}+n\pi\mid n\in\mathbb{Z}\right\\}$ _radians_
    ///
    /// # Range
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Warning
    ///
    /// The value of the tangent function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = std::f64::consts::FRAC_PI_4;
    /// let abs_difference = (x.tan() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-15);
    /// ```
    ///
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/Tangent.html](https://mathworld.wolfram.com/Tangent.html)
    fn tan(&self) -> Self;

    /// Computes the cosecant of this value, where this value is in radians.
    ///
    /// # Returns
    ///
    /// Cosecant of this value.
    ///
    /// # Domain
    ///
    /// $\mathbb{R}\\;\setminus\\;\left\\{n\pi\mid n\in\mathbb{Z}\right\\}$ _radians_
    ///
    /// # Range
    ///
    /// $(-\infty,-1]\cup[1,\infty)$
    ///
    /// # Warning
    ///
    /// The value of the cosecant function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = std::f64::consts::FRAC_PI_2;
    /// let abs_difference = (x.csc() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/Cosecant.html](https://mathworld.wolfram.com/Cosecant.html)
    fn csc(&self) -> Self;

    /// Computes the secant of this value, where this value is in radians.
    ///
    /// # Returns
    ///
    /// Secant of this value.
    ///
    /// # Domain
    ///
    /// $\mathbb{R}\\;\setminus\\;\left\\{\frac{\pi}{2}+n\pi\mid n\in\mathbb{Z}\right\\}$ _radians_
    ///
    /// # Range
    ///
    /// $(-\infty,-1]\cup[1,\infty)$
    ///
    /// # Warning
    ///
    /// The value of the secant function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = std::f64::consts::PI;
    /// let abs_difference = (x.sec() + 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/Secant.html](https://mathworld.wolfram.com/Secant.html)
    fn sec(&self) -> Self;

    /// Computes the cotangent of this value, where this value is in radians.
    ///
    /// # Returns
    ///
    /// Cotangent of this value.
    ///
    /// # Domain
    ///
    /// $\mathbb{R}\\;\setminus\\;\left\\{n\pi\mid n\in\mathbb{Z}\right\\}$ _radians_
    ///
    /// # Range
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Warning
    ///
    /// The value of the cotangent function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = std::f64::consts::FRAC_PI_4;
    /// let abs_difference = (x.cot() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-15);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/Cotangent.html](https://mathworld.wolfram.com/Cotangent.html)
    fn cot(&self) -> Self;

    /// Computes the inverse sine of this value, returning the result in radians.
    ///
    /// # Returns
    ///
    /// Inverse sine of this value \[rad\].
    ///
    /// # Domain
    ///
    /// $[-1,1]$
    ///
    /// # Range
    ///
    /// $\left[-\frac{\pi}{2},\frac{\pi}{2}\right]$ _radians_
    ///
    /// # Warning
    ///
    /// The value of the inverse sine function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let f = std::f64::consts::FRAC_PI_2;
    ///
    /// // asin(sin(pi/2))
    /// let abs_difference = (f.sin().asin() - std::f64::consts::FRAC_PI_2).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseSine.html](https://mathworld.wolfram.com/InverseSine.html)
    fn asin(&self) -> Self;

    /// Computes the inverse cosine of this value, returning the result in radians.
    ///
    /// # Returns
    ///
    /// Inverse cosine of this value \[rad\].
    ///
    /// # Domain
    ///
    /// $[-1,1]$
    ///
    /// # Range
    ///
    /// $[0,\pi]$ _radians_
    ///
    /// # Warning
    ///
    /// The value of the inverse cosine function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let f = std::f64::consts::FRAC_PI_4;
    ///
    /// // acos(cos(pi/4))
    /// let abs_difference = (f.cos().acos() - std::f64::consts::FRAC_PI_4).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseCosine.html](https://mathworld.wolfram.com/InverseCosine.html)
    fn acos(&self) -> Self;

    /// Computes the inverse tangent of this value, returning the result in radians.
    ///
    /// # Returns
    ///
    /// Inverse tangent of this value \[rad\].
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Range
    ///
    /// $\left(-\frac{\pi}{2},\frac{\pi}{2}\right)$ _radians_
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let f = 1.0_f64;
    ///
    /// // atan(tan(1))
    /// let abs_difference = (f.tan().atan() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseTangent.html](https://mathworld.wolfram.com/InverseTangent.html)
    fn atan(&self) -> Self;

    /// Computes the inverse cosecant of this value, returning the result in radians.
    ///
    /// # Returns
    ///
    /// Inverse cosecant of this value \[rad\].
    ///
    /// # Domain
    ///
    /// $(-\infty,-1]\cup[1,\infty)$
    ///
    /// # Range
    ///
    /// $\left[-\frac{\pi}{2},0\right)\cup\left(0,\frac{\pi}{2}\right]$ _radians_
    ///
    /// # Warning
    ///
    /// The value of the inverse cosecant function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let f = std::f64::consts::FRAC_PI_2;
    ///
    /// // acsc(csc(pi/2))
    /// let abs_difference = (f.csc().acsc() - std::f64::consts::FRAC_PI_2).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseCosecant.html](https://mathworld.wolfram.com/InverseCosecant.html)
    fn acsc(&self) -> Self;

    /// Computes the inverse secant of this value, returning the result in radians.
    ///
    /// # Returns
    ///
    /// Inverse secant of this value \[rad\].
    ///
    /// # Domain
    ///
    /// $(-\infty,-1]\cup[1,\infty)$
    ///
    /// # Range
    ///
    /// $\left[0,\frac{\pi}{2}\right)\cup\left(\frac{\pi}{2},\pi\right]$ _radians_
    ///
    /// # Warning
    ///
    /// The value of the inverse secant function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let f = std::f64::consts::FRAC_PI_4;
    ///
    /// // asec(sec(pi/4))
    /// let abs_difference = (f.sec().asec() - std::f64::consts::FRAC_PI_4).abs();
    ///
    /// println!("{}", abs_difference);
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseSecant.html](https://mathworld.wolfram.com/InverseSecant.html)
    fn asec(&self) -> Self;

    /// Computes the inverse cotangent of this value, returning the result in radians.
    ///
    /// # Returns
    ///
    /// Inverse cotangent of this value \[rad\].
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Range
    ///
    /// $\left(-\frac{\pi}{2},0\right)\cup\left(0,\frac{\pi}{2}\right]$ _radians_
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let f = 1.0_f64;
    ///
    /// // acot(cot(1))
    /// let abs_difference = (f.cot().acot() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseCotangent.html](https://mathworld.wolfram.com/InverseCotangent.html)
    fn acot(&self) -> Self;
}
