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
    /// let x = std::f64::consts::FRAC_PI_2;
    /// let f = x.sin().asin();
    /// let abs_difference = (f - x).abs();
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
    /// let x = std::f64::consts::FRAC_PI_4;
    /// let f = x.cos().acos();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-16);
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
    /// let x = 1.0_f64;
    /// let f = x.tan().atan();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseTangent.html](https://mathworld.wolfram.com/InverseTangent.html)
    fn atan(&self) -> Self;

    /// Computes the four-quadrant inverse tangent of `self` (`y`) and `other` (`x`), returning the
    /// result in radians.
    ///
    /// The four-quadrant inverse tangent computes the angle, measured counterclockwise, between the
    /// $+x$-axis and the ray from the origin to the point $(x,y)$.
    ///
    /// # Arguments
    ///
    /// * `other` - Other value (`x`) for computing the four-quadrant inverse with `self` (`y`).
    ///
    /// # Returns
    ///
    /// Four-quadrant inverse tangent of `self` (`y`) and `other` (`x`) \[rad\].
    ///
    /// # Domain
    ///
    /// * `self` (`y`): $(-\infty,\infty)$
    /// * `other` (`x`): $(-\infty,\infty)$
    ///
    /// # Range
    ///
    /// $[-\pi,\pi]$ _radians_
    ///
    /// # Warning
    ///
    /// In the examples below, we pass `x` to `atan2` by value instead of as the reference `&x`.
    /// This is because the standard [`f32::atan2`] and [`f64::atan2`] method will be used for
    /// [`f32`]s and [`f64`]s, respectively, instead of the [`Trig::atan2`] method.
    ///
    /// # Examples
    ///
    /// ## 45 degrees clockwise
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = 3.0_f64;
    /// let y = -3.0_f64;
    /// let angle_expected = -std::f64::consts::FRAC_PI_4;
    /// let abs_difference = (y.atan2(x) - angle_expected).abs();
    ///
    /// assert!(abs_difference < 1.0e-16);
    /// ```
    ///
    /// ## 135 degrees counterclockwise
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = -3.0_f64;
    /// let y = 3.0_f64;
    /// let angle_expected = 3.0 * std::f64::consts::FRAC_PI_4;
    /// let abs_difference = (y.atan2(x) - angle_expected).abs();
    ///
    /// assert!(abs_difference < 1.0e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://en.wikipedia.org/wiki/Atan2](https://en.wikipedia.org/wiki/Atan2)
    fn atan2(&self, other: &Self) -> Self;

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
    /// let x = std::f64::consts::FRAC_PI_2;
    /// let f = x.csc().acsc();
    /// let abs_difference = (f - x).abs();
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
    /// let x = std::f64::consts::FRAC_PI_4;
    /// let f = x.sec().asec();
    /// let abs_difference = (f - x).abs();
    ///
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
    /// let x = 1.0_f64;
    /// let f = x.cot().acot();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseCotangent.html](https://mathworld.wolfram.com/InverseCotangent.html)
    fn acot(&self) -> Self;

    /// Convert this value from degrees to radians.
    ///
    /// # Returns
    ///
    /// This value (originally in degrees) in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = 30.0_f64;
    /// let abs_difference = (x.deg2rad() - std::f64::consts::FRAC_PI_6).abs();
    ///
    /// assert!(abs_difference < 1e-15);
    /// ```
    fn deg2rad(&self) -> Self;

    /// Convert this value from radians to degrees.
    ///
    /// # Returns
    ///
    /// This value (originally in radians) in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = std::f64::consts::FRAC_PI_6;
    /// let abs_difference = (x.rad2deg() - 30.0).abs();
    ///
    /// assert!(abs_difference < 1e-14);
    /// ```
    fn rad2deg(&self) -> Self;

    /// Computes the sine of this value, where this value is in degrees.
    ///
    /// # Returns
    ///
    /// Sine of this value.
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$ _degrees_
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
    /// let x = 90.0_f64;
    /// let abs_difference = (x.sind() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/Sine.html](https://mathworld.wolfram.com/Sine.html)
    fn sind(&self) -> Self;

    /// Computes the cosine of this value, where this value is in degrees.
    ///
    /// # Returns
    ///
    /// Cosine of this value.
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$ _degrees_
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
    /// let x = 360.0_f64;
    /// let abs_difference = (x.cosd() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/Cosine.html](https://mathworld.wolfram.com/Cosine.html)
    fn cosd(&self) -> Self;

    /// Computes the tangent of this value, where this value is in degrees.
    ///
    /// # Returns
    ///
    /// Tangent of this value.
    ///
    /// # Domain
    ///
    /// $\mathbb{R}\\;\setminus\\;\left\\{90+180n\mid n\in\mathbb{Z}\right\\}$ _degrees_
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
    /// let x = 45.0_f64;
    /// let abs_difference = (x.tand() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-15);
    /// ```
    ///
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/Tangent.html](https://mathworld.wolfram.com/Tangent.html)
    fn tand(&self) -> Self;

    /// Computes the cosecant of this value, where this value is in degrees.
    ///
    /// # Returns
    ///
    /// Cosecant of this value.
    ///
    /// # Domain
    ///
    /// $\mathbb{R}\\;\setminus\\;\left\\{180n\mid n\in\mathbb{Z}\right\\}$ _degrees_
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
    /// let x = 90.0_f64;
    /// let abs_difference = (x.cscd() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/Cosecant.html](https://mathworld.wolfram.com/Cosecant.html)
    fn cscd(&self) -> Self;

    /// Computes the secant of this value, where this value is in degrees.
    ///
    /// # Returns
    ///
    /// Secant of this value.
    ///
    /// # Domain
    ///
    /// $\mathbb{R}\\;\setminus\\;\left\\{90+180n\mid n\in\mathbb{Z}\right\\}$ _degrees_
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
    /// let x = 180.0_f64;
    /// let abs_difference = (x.secd() + 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/Secant.html](https://mathworld.wolfram.com/Secant.html)
    fn secd(&self) -> Self;

    /// Computes the cotangent of this value, where this value is in degrees.
    ///
    /// # Returns
    ///
    /// Cotangent of this value.
    ///
    /// # Domain
    ///
    /// $\mathbb{R}\\;\setminus\\;\left\\{180n\mid n\in\mathbb{Z}\right\\}$ _degrees_
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
    /// let x = 45.0_f64;
    /// let abs_difference = (x.cotd() - 1.0).abs();
    ///
    /// assert!(abs_difference < 1e-15);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/Cotangent.html](https://mathworld.wolfram.com/Cotangent.html)
    fn cotd(&self) -> Self;

    /// Computes the inverse sine of this value, returning the result in degrees.
    ///
    /// # Returns
    ///
    /// Inverse sine of this value \[deg\].
    ///
    /// # Domain
    ///
    /// $[-1,1]$
    ///
    /// # Range
    ///
    /// $\left[-90,90\right]$ _degrees_
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
    /// let x = 90.0_f64;
    /// let f = x.sind().asind();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseSine.html](https://mathworld.wolfram.com/InverseSine.html)
    fn asind(&self) -> Self;

    /// Computes the inverse cosine of this value, returning the result in degrees.
    ///
    /// # Returns
    ///
    /// Inverse cosine of this value \[deg\].
    ///
    /// # Domain
    ///
    /// $[-1,1]$
    ///
    /// # Range
    ///
    /// $\[0,180\]$ _degrees_
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
    /// let x = 45.0_f64;
    /// let f = x.cosd().acosd();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseCosine.html](https://mathworld.wolfram.com/InverseCosine.html)
    fn acosd(&self) -> Self;

    /// Computes the inverse tangent of this value, returning the result in degrees.
    ///
    /// # Returns
    ///
    /// Inverse tangent of this value \[deg\].
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Range
    ///
    /// $\left(-\frac{\pi}{2},\frac{\pi}{2}\right)$ _degrees_
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = 30.0_f64;
    /// let f = x.tand().atand();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-14);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseTangent.html](https://mathworld.wolfram.com/InverseTangent.html)
    fn atand(&self) -> Self;

    /// Computes the four-quadrant inverse tangent of `self` (`y`) and `other` (`x`), returning the
    /// result in degrees.
    ///
    /// The four-quadrant inverse tangent computes the angle, measured counterclockwise, between the
    /// $+x$-axis and the ray from the origin to the point $(x,y)$.
    ///
    /// # Arguments
    ///
    /// * `other` - Other value (`x`) for computing the four-quadrant inverse with `self` (`y`).
    ///
    /// # Returns
    ///
    /// Four-quadrant inverse tangent of `self` (`y`) and `other` (`x`) \[deg\].
    ///
    /// # Domain
    ///
    /// * `self` (`y`): $(-\infty,\infty)$
    /// * `other` (`x`): $(-\infty,\infty)$
    ///
    /// # Range
    ///
    /// $[-180,180]$ _degrees_
    ///
    /// # Examples
    ///
    /// ## 45 degrees clockwise
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = 3.0_f64;
    /// let y = -3.0_f64;
    /// let angle_expected = -45.0_f64;
    /// let abs_difference = (y.atan2d(&x) - angle_expected).abs();
    ///
    /// assert!(abs_difference < 1.0e-16);
    /// ```
    ///
    /// ## 135 degrees counterclockwise
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = -3.0_f64;
    /// let y = 3.0_f64;
    /// let angle_expected = 135.0_f64;
    /// let abs_difference = (y.atan2d(&x) - angle_expected).abs();
    ///
    /// assert!(abs_difference < 1.0e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://en.wikipedia.org/wiki/Atan2](https://en.wikipedia.org/wiki/Atan2)
    fn atan2d(&self, other: &Self) -> Self;

    /// Computes the inverse cosecant of this value, returning the result in degrees.
    ///
    /// # Returns
    ///
    /// Inverse cosecant of this value \[deg\].
    ///
    /// # Domain
    ///
    /// $(-\infty,-1]\cup[1,\infty)$
    ///
    /// # Range
    ///
    /// $\left[-90,0\right)\cup\left(0,90\right]$ _degrees_
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
    /// let x = 90.0_f64;
    /// let f = x.cscd().acscd();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseCosecant.html](https://mathworld.wolfram.com/InverseCosecant.html)
    fn acscd(&self) -> Self;

    /// Computes the inverse secant of this value, returning the result in degrees.
    ///
    /// # Returns
    ///
    /// Inverse secant of this value \[deg\].
    ///
    /// # Domain
    ///
    /// $(-\infty,-1]\cup[1,\infty)$
    ///
    /// # Range
    ///
    /// $\left[0,90\right)\cup\left(90,180\right]$ _degrees_
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
    /// let x = 45.0_f64;
    /// let f = x.secd().asecd();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseSecant.html](https://mathworld.wolfram.com/InverseSecant.html)
    fn asecd(&self) -> Self;

    /// Computes the inverse cotangent of this value, returning the result in degrees.
    ///
    /// # Returns
    ///
    /// Inverse cotangent of this value \[deg\].
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Range
    ///
    /// $\left(-90,0\right)\cup\left(0,90\right]$ _degrees_
    ///
    /// # Examples
    ///
    /// ```
    /// use trig::Trig;
    ///
    /// let x = 30.0_f64;
    /// let f = x.cotd().acotd();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-14);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseCotangent.html](https://mathworld.wolfram.com/InverseCotangent.html)
    fn acotd(&self) -> Self;

    /// Computes the hyperbolic sine of this value.
    ///
    /// # Returns
    ///
    /// Hyperbolic sine of this value.
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Range
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::E;
    /// use trig::Trig;
    ///
    /// let x = 1.0_f64;
    ///
    /// let f = x.sinh();
    /// let g = ((E * E) - 1.0) / (2.0 * E);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/HyperbolicSine.html](https://mathworld.wolfram.com/HyperbolicSine.html)
    fn sinh(&self) -> Self;

    /// Computes the hyperbolic cosine of this value.
    ///
    /// # Returns
    ///
    /// Hyperbolic cosine of this value.
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Range
    ///
    /// $[1,\infty)$
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::E;
    /// use trig::Trig;
    ///
    /// let x = 1.0_f64;
    ///
    /// let f = x.cosh();
    /// let g = ((E * E) + 1.0) / (2.0 * E);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference < 1e-15);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/HyperbolicCosine.html](https://mathworld.wolfram.com/HyperbolicCosine.html)
    fn cosh(&self) -> Self;

    /// Computes the hyperbolic tangent of this value.
    ///
    /// # Returns
    ///
    /// Hyperbolic tangent of this value.
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Range
    ///
    /// $(-1,1)$
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::E;
    /// use trig::Trig;
    ///
    /// let x = 1.0_f64;
    ///
    /// let f = x.tanh();
    /// let g = (1.0 - E.powi(-2)) / (1.0 + E.powi(-2));
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/HyperbolicTangent.html](https://mathworld.wolfram.com/HyperbolicTangent.html)
    fn tanh(&self) -> Self;

    /// Computes the hyperbolic cosecant of this value.
    ///
    /// # Returns
    ///
    /// Hyperbolic cosecant of this value.
    ///
    /// # Domain
    ///
    /// $(-\infty,0)\cup(0,\infty)$
    ///
    /// # Range
    ///
    /// $(-\infty,0)\cup(0,\infty)$
    ///
    /// # Warning
    ///
    /// The value of the hyperbolic cosecant function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::E;
    /// use trig::Trig;
    ///
    /// let x = 1.0_f64;
    ///
    /// let f = x.csch();
    /// let g = (2.0 * E) / ((E * E) - 1.0);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/HyperbolicCosecant.html](https://mathworld.wolfram.com/HyperbolicCosecant.html)
    fn csch(&self) -> Self;

    /// Computes the hyperbolic secant of this value.
    ///
    /// # Returns
    ///
    /// Hyperbolic secant of this value.
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Range
    ///
    /// $(0,1]$
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::E;
    /// use trig::Trig;
    ///
    /// let x = 1.0_f64;
    ///
    /// let f = x.sech();
    /// let g = (2.0 * E) / ((E * E) + 1.0);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/HyperbolicSecant.html](https://mathworld.wolfram.com/HyperbolicSecant.html)
    fn sech(&self) -> Self;

    /// Computes the hyperbolic cotangent of this value.
    ///
    /// # Returns
    ///
    /// Hyperbolic cotangent of this value.
    ///
    /// # Domain
    ///
    /// $(-\infty,0)\cup(0,\infty)$
    ///
    /// # Range
    ///
    /// $(-\infty,-1)\cup(1,\infty)$
    ///
    /// # Warning
    ///
    /// The value of the hyperbolic cotangent function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::E;
    /// use trig::Trig;
    ///
    /// let x = 1.0_f64;
    ///
    /// let f = x.coth();
    /// let g = (1.0 + E.powi(-2)) / (1.0 - E.powi(-2));
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/HyperbolicCotangent.html](https://mathworld.wolfram.com/HyperbolicCotangent.html)
    fn coth(&self) -> Self;

    /// Computes the inverse hyperbolic sine of this value.
    ///
    /// # Returns
    ///
    /// Inverse hyperbolic sine of this value.
    ///
    /// # Domain
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Range
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::E;
    /// use trig::Trig;
    ///
    /// let x = 1.0_f64;
    /// let f = x.sinh().asinh();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseHyperbolicSine.html](https://mathworld.wolfram.com/InverseHyperbolicSine.html)
    fn asinh(&self) -> Self;

    /// Computes the inverse hyperbolic cosine of this value.
    ///
    /// # Returns
    ///
    /// Inverse hyperbolic cosine of this value.
    ///
    /// # Domain
    ///
    /// $(1,\infty)$
    ///
    /// # Range
    ///
    /// $[0,\infty)$
    ///
    /// # Warning
    ///
    /// The value of the inverse hyperbolic cosine function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::E;
    /// use trig::Trig;
    ///
    /// let x = 1.0_f64;
    /// let f = x.cosh().acosh();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseHyperbolicCosine.html](https://mathworld.wolfram.com/InverseHyperbolicCosine.html)
    fn acosh(&self) -> Self;

    /// Computes the inverse hyperbolic tangent of this value.
    ///
    /// # Returns
    ///
    /// Inverse hyperbolic tangent of this value.
    ///
    /// # Domain
    ///
    /// $(-1,1)$
    ///
    /// # Range
    ///
    /// $(-\infty,\infty)$
    ///
    /// # Warning
    ///
    /// The value of the inverse hyperbolic tangent function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::E;
    /// use trig::Trig;
    ///
    /// let x = 1.0_f64;
    /// let f = x.tanh().atanh();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-15);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseHyperbolicTangent.html](https://mathworld.wolfram.com/InverseHyperbolicTangent.html)
    fn atanh(&self) -> Self;

    /// Computes the inverse hyperbolic cosecant of this value.
    ///
    /// # Returns
    ///
    /// Inverse hyperbolic cosecant of this value.
    ///
    /// # Domain
    ///
    /// $(\infty,0)\cup(0,\infty)$
    ///
    /// # Range
    ///
    /// $(\infty,0)\cup(0,\infty)$
    ///
    /// # Warning
    ///
    /// The value of the inverse hyperbolic cosecant function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::E;
    /// use trig::Trig;
    ///
    /// let x = 1.0_f64;
    /// let f = x.csch().acsch();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-16);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseHyperbolicCosecant.html](https://mathworld.wolfram.com/InverseHyperbolicCosecant.html)
    fn acsch(&self) -> Self;

    /// Computes the inverse hyperbolic secant of this value.
    ///
    /// # Returns
    ///
    /// Inverse hyperbolic secant of this value.
    ///
    /// # Domain
    ///
    /// $(0,1]$
    ///
    /// # Range
    ///
    /// $[0,\infty)$
    ///
    /// # Warning
    ///
    /// The value of the inverse hyperbolic secant function at points outside its domain is `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::E;
    /// use trig::Trig;
    ///
    /// let x = 0.5_f64;
    /// let f = x.sech().asech();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-15);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseHyperbolicSecant.html](https://mathworld.wolfram.com/InverseHyperbolicSecant.html)
    fn asech(&self) -> Self;

    /// Computes the inverse hyperbolic cotangent of this value.
    ///
    /// # Returns
    ///
    /// Inverse hyperbolic cotangent of this value.
    ///
    /// # Domain
    ///
    /// $(-\infty,-1)\cup(1,\infty)$
    ///
    /// # Range
    ///
    /// $(-\infty,0)\cup(0,\infty)$
    ///
    /// # Warning
    ///
    /// The value of the inverse hyperbolic cotangent function at points outside its domain is
    /// `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::E;
    /// use trig::Trig;
    ///
    /// let x = 1.5_f64;
    /// let f = x.coth().acoth();
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference < 1e-15);
    /// ```
    ///
    /// # References
    ///
    /// * [https://mathworld.wolfram.com/InverseHyperbolicCotangent.html](https://mathworld.wolfram.com/InverseHyperbolicCotangent.html)
    fn acoth(&self) -> Self;
}
