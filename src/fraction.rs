use crate::Complex;

/// Represents a rational number through a fraction, storing the numerator as an `i32`, 
/// and the denominator as a `u32`, for consistency with mathematical standards. 
#[derive(Clone, Copy, Debug)]
pub struct Fraction
{
    numerator: i32,
    denominator: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DivByZeroError;

impl Fraction
{
    /// Creates a fraction that is fully simplified. 
    /// Will return `DivByZeroError` if denominator is 0. 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let simplified = Fraction::from(2, 4).unwrap();
    /// 
    /// assert_eq!(simplified.get_components(), Fraction::from(1, 2).unwrap().get_components());
    /// 
    /// ```
    pub const fn from(numerator: i32, denominator: u32) -> Result<Fraction, DivByZeroError>
    {
        let fraction = Fraction::unsimplified_from(numerator, denominator);

        match fraction
        {
            Ok(value) => Ok(value.simplify()),
            Err(value) => Err(value)
        }
    }
    
    /// Creates a fraction that has no fractional simplification applied to it. 
    /// 
    /// Will return `DivByZeroError` if denominator is 0. 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let unsimplified = Fraction::unsimplified_from(2, 4).unwrap();
    /// 
    /// assert_ne!(unsimplified.get_components(), Fraction::unsimplified_from(1, 2).unwrap().get_components());
    /// ```
    pub const fn unsimplified_from(numerator: i32, denominator: u32) -> Result<Fraction, DivByZeroError>
    {
        if denominator == 0
        {
            return Err(DivByZeroError);
        }

        Ok(Fraction {numerator, denominator})
    }

    /// Creates a fraction with no checks on the input. 
    /// Can cause arithmatic issues if the denominator is 0. 
    /// 
    /// ```
    /// use complex::Fraction;
    /// 
    /// // can be convenient for hardcoding values
    /// let one_half = Fraction::unchecked_from(1, 2);
    /// 
    /// let invalid = Fraction::unchecked_from(1, 0);
    /// 
    /// assert_eq!(invalid.get_denominator(), 0);
    /// ```
    pub const fn unchecked_from(numerator: i32, denominator: u32) -> Fraction
    {
        Fraction {numerator, denominator}
    }

    /// Simplifies a fraction by dividing both the numerator and the denominator
    /// by their greatest common factor. 
    /// Note that fractions created with `Fraction::from` are simplified uppon creation. 
    /// 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let unsimplified = Fraction::unchecked_from(2, 4);
    /// let simplified = unsimplified.simplify();
    /// 
    /// let also_simplified = Fraction::from(2, 4).unwrap();
    /// 
    /// assert_eq!(simplified.get_components(), also_simplified.get_components());
    /// ```
    pub const fn simplify(&self) -> Fraction
    {
        let gcd = gcd(self.numerator.abs() as u32, self.denominator);

        let numerator = self.numerator / gcd as i32;
        let denominator = self.denominator / gcd;
        
        Fraction {numerator, denominator}
    }
    
    /// Creates a fraction with `value` as the numerator and 1 as the denominator. 
    /// The returned fraction will represent the same number as `value`. 
    /// 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let a = Fraction::from_i32(2);
    /// let b = Fraction::unchecked_from(2, 1);
    /// 
    /// assert_eq!(a, b);
    /// ```
    pub const fn from_i32(value: i32) -> Fraction
    {
        Fraction::unchecked_from(value, 1)
    }
    
    /// Returns a tuple with the numerator for the first value, and the denominator
    /// for the second. 
    /// 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let fraction = Fraction::unchecked_from(1, 2);
    /// 
    /// assert_eq!(fraction.get_components(), (1, 2));
    /// ```
    /// 
    /// Can be used to compare fractions by their constituents in stead of by the
    /// value they represend. 
    /// 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let a = Fraction::unchecked_from(1, 2);
    /// let b = Fraction::unchecked_from(1, 2);
    /// let c = Fraction::unchecked_from(2, 4);
    /// 
    /// assert_eq!(a.get_components(), b.get_components());
    /// assert_ne!(a.get_components(), c.get_components());
    /// ```
    pub const fn get_components(&self) -> (i32, u32)
    {
        (self.numerator, self.denominator)
    }

    /// Returns the numerator of this fraction. 
    /// 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let fraction = Fraction::unchecked_from(1, 2);
    /// 
    /// assert_eq!(fraction.get_numerator(), 1);
    /// ```
    pub const fn get_numerator(&self) -> i32
    {
        self.numerator
    }

    /// Returbs the denominator of this fraction
    /// 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let fraction = Fraction::unchecked_from(1, 2);
    /// 
    /// assert_eq!(fraction.get_denominator(), 2);
    /// ```
    pub const fn get_denominator(&self) -> u32
    {
        self.denominator
    }

    /// Returns the numerator divided by the denominator, as an `f64`. 
    /// 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let fraction = Fraction::unchecked_from(1, 3);
    /// let float_value = fraction.to_f64();
    /// 
    /// assert_eq!(float_value, 1.0 / 3.0);
    /// ```
    pub fn to_f64(&self) -> f64
    {
        (self.numerator as f64) / (self.denominator as f64)
    }

    /// Finds the closest fractional value to `value`, with a tolerance of
    /// `error`. 
    /// 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let fraction = Fraction::from_f64(0.33333, 0.00001);
    /// 
    /// assert_eq!(fraction, Fraction::unchecked_from(1, 3));
    /// ```
    pub fn from_f64(value: f64, error: f64) -> Fraction
    {
        let integer_part = value.floor();
        let decimal_part = value - integer_part;

        if decimal_part < error
        {
            return Fraction::from_i32(integer_part as i32);
        }
        else if decimal_part > 1.0 - error
        {
            return Fraction::from_i32(integer_part as i32 + 1);
        }

        let mut lower = Fraction::from_i32(0);
        let mut upper = Fraction::from_i32(1);

        loop
        {
            let middle = Fraction::unchecked_from
            (
                lower.numerator + upper.numerator,
                lower.denominator + upper.denominator
            );

            if (middle.numerator as f64) > middle.denominator as f64 * (decimal_part + error)
            {
                upper = middle;
            }
            else if (middle.numerator as f64) < middle.denominator as f64 * (decimal_part - error)
            {
                lower = middle;
            }
            else
            {
                return Fraction::from
                (
                    integer_part as i32 * middle.denominator as i32 + middle.numerator,
                    middle.denominator
                )
                .expect("Denominator will not be 0");
            }
        }
    }
}

impl std::fmt::Display for Fraction
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        if self.denominator != 1
        {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
        else
        {
            write!(f, "{}", self.numerator)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFractionError;

impl std::str::FromStr for Fraction
{
    type Err = ParseFractionError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let (numerator_str, denominator_str) = s
            .split_once('/')
            .ok_or(ParseFractionError)?;

        let numerator = numerator_str.trim().parse::<i32>().map_err(|_| ParseFractionError)?;
        let denominator = denominator_str.trim().parse::<u32>().map_err(|_| ParseFractionError)?;

        Fraction::from(numerator, denominator).map_err(|_| ParseFractionError)
    }
}

impl PartialEq for Fraction
{
    fn eq(&self, other: &Self) -> bool
    {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Fraction {}

impl PartialOrd for Fraction
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        (self.numerator * other.denominator as i32).cmp(&(other.numerator * self.denominator as i32))
    }
}

impl std::ops::Add<Fraction> for Fraction
{
    type Output = Fraction;

    fn add(self, rhs: Fraction) -> Self::Output
    {
        let denominator_gcd = gcd(self.denominator, rhs.denominator);

        let numerator = self.numerator * (rhs.denominator / denominator_gcd) as i32
            + rhs.numerator * (self.denominator / denominator_gcd) as i32;

        let denominator = self.denominator * rhs.denominator / denominator_gcd;

        Fraction::from(numerator, denominator).expect("Fraction should not have 0 for denominator")
    }
}

impl std::ops::AddAssign for Fraction
{
    fn add_assign(&mut self, rhs: Self)
    {
        *self = *self + rhs;
    }
}

impl std::ops::Neg for Fraction
{
    type Output = Fraction;

    fn neg(self) -> Self::Output
    {
        Fraction::unchecked_from(-self.numerator, self.denominator)
    }
}

impl std::ops::Sub<Fraction> for Fraction
{
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Self::Output
    {
        self + (-rhs)
    }
}

impl std::ops::SubAssign for Fraction
{
    fn sub_assign(&mut self, rhs: Self)
    {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<Fraction> for Fraction
{
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Self::Output
    {
        Fraction::from(self.numerator * rhs.numerator, self.denominator * rhs.denominator)
            .expect("Fraction should not have 0 for denominator")
    }
}

impl std::ops::MulAssign for Fraction
{
    fn mul_assign(&mut self, rhs: Self)
    {
        *self = *self * rhs;
    }
}

impl Fraction
{
    /// Returns a number representing the sign of the fraction. 
    /// - `0` if the number is 0
    /// - `1` if the number is positive
    /// - `-1` if the number is negative
    /// 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let fraction = Fraction::unchecked_from(-5, 2);
    /// 
    /// assert_eq!(fraction.signum(), -1);
    /// ```
    pub const fn signum(self) -> i32
    {
        self.numerator.signum()
    }

    /// Returns a fraction with the numerator and denominator of `self` switched,
    /// perserving the sign of the numerator, returning a `DivByZeroError` if the
    /// denominator is zero. 
    /// 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let fraction = Fraction::unchecked_from(1, 2);
    /// 
    /// assert_eq!(fraction.reciprocal().unwrap(), Fraction::unchecked_from(2, 1));
    /// ```
    pub const fn reciprocal(self) -> Result<Fraction, DivByZeroError>
    {
        Fraction::unsimplified_from(self.denominator as i32 * self.numerator.signum(), self.numerator.abs() as u32)
    }

    /// Returns the absolute value of the fraction. 
    /// 
    /// ```
    /// use complex::Fraction;
    /// 
    /// let fraction = Fraction::unchecked_from(-1, 2);
    /// 
    /// assert_eq!(fraction.abs(), Fraction::unchecked_from(1, 2));
    /// ```
    pub const fn abs(self) -> Fraction
    {
        Fraction::unchecked_from(self.numerator.abs(), self.denominator)
    }

    pub fn sqrt(self) -> Complex
    {
        let value = Fraction::from_f64(self.abs().to_f64().sqrt(), 0.000000000001);

        if self.numerator < 0
        {
            Complex::from_fraction_imaginary(Fraction::from_i32(0))
        }
        else
        {
            Complex::from_fraction(value)
        }
    }
}

impl std::ops::Div<Fraction> for Fraction
{
    type Output = Fraction;

    fn div(self, rhs: Fraction) -> Self::Output
    {
        self * rhs.reciprocal().expect("Divide by 0")
    }
}

impl std::ops::DivAssign for Fraction
{
    fn div_assign(&mut self, rhs: Self)
    {
        *self = *self / rhs;
    }
}

/// computes the greatest common divisor between the two numbers
const fn gcd(a: u32, b: u32) -> u32
{
    let (mut small, mut large) = get_ordering(a, b);

    let mut i = 1;
    let mut result = 1;

    while i <= small
    {
        if small % i == 0 && large % i == 0
        {
            small /= i;
            large /= i;
            result *= i;
            i = 1;
        }

        i += 1;
    }

    result
}

/// returns a pair with the smallest value first
const fn get_ordering(a: u32, b: u32) -> (u32, u32)
{
    if a < b
    {
        (a, b)
    }
    else
    {
        (b, a)
    }
}
