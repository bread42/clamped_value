//! Provides a generic `ClampedValue` struct that stores a value and ensures that it is
//! always within the specified minimum and maximum values.

use num_traits::{SaturatingAdd, SaturatingMul, SaturatingSub};
use std::ops::{AddAssign, Div, DivAssign, MulAssign, Sub, SubAssign};

/// A value that is clamped between a minimum and maximum value.
#[derive(Debug)]
pub struct ClampedValue<T: PartialOrd + Clone> {
    value: T,
    min: T,
    max: T,
}

impl<T: PartialOrd + Clone> ClampedValue<T> {
    /// Creates a new `ClampedValue<T>`.
    ///
    /// # Panics
    ///
    /// Panics if either:
    /// - `min` is larger than `max`
    /// - `value` is not within `min` and `max`
    pub fn new(min: T, value: T, max: T) -> Self {
        if min > max {
            panic!("Cannot create a clamped value where the minimum is larger than the maximum");
        } else if value < min || value > max {
            panic!("Cannot create a clamped value where the value is not within the minimum and maximum");
        }

        Self { value, min, max }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn min(&self) -> &T {
        &self.min
    }

    pub fn max(&self) -> &T {
        &self.max
    }

    /// Sets the minimum to `new_min`.
    ///
    /// # Panics
    ///
    /// Panics if either:
    /// - `new_min` is larger than the maximum
    /// - `new_min` is larger than the current value
    pub fn set_min(&mut self, new_min: T) {
        if new_min > self.max {
            panic!("Cannot set the minimum to a value that is larger than the maximum");
        } else if new_min > self.value {
            panic!("Cannot set the minimum to a value that is larger than the current value");
        }

        self.min = new_min;
    }

    /// Sets the maximum to `new_max`.
    ///
    /// # Panics
    ///
    /// Panics if either:
    /// - `new_max` is smaller than the minimum
    /// - `new_max` is smaller than the current value
    pub fn set_max(&mut self, new_max: T) {
        if new_max < self.min {
            panic!("Cannot set the maximum to a value that is smaller than the minimum");
        } else if new_max < self.value {
            panic!("Cannot set the maximum to a value that is smaller than the current value")
        }

        self.max = new_max;
    }

    /// Sets the value to `new_value`, saturating at min or max if `new_value` is outside those bounds.
    pub fn set(&mut self, new_value: T) {
        self.value = new_value;
        self.clamp();
    }

    // clamps self.value in between self.min and self.max
    fn clamp(&mut self) {
        if self.value < self.min {
            self.value = self.min.clone();
        } else if self.value > self.max {
            self.value = self.max.clone();
        }
    }
}

impl<T> ClampedValue<T>
where
    T: Into<f32> + Sub<Output = T> + PartialOrd + Clone,
{
    /// Returns an f32 ranging from 0.0 to 1.0, representing the current value
    /// in relation to the minimum and maximum, where 0.0 is the minimum and
    /// 1.0 is the maximum
    /// 
    /// # Examples
    /// 
    /// ```
    /// use clamped_values::ClampedValue;
    /// 
    /// let clamped_value = ClampedValue::<u8>::new(50, 75, 100);
    /// 
    /// assert_eq!(clamped_value.percent_f32(), 0.5);
    /// ```
    pub fn percent_f32(&self) -> f32 {
        self.percent::<f32>()
    }
}

impl<T> ClampedValue<T>
where
    T: Into<f64> + Sub<Output = T> + PartialOrd + Clone,
{
    /// Returns an f64 ranging from 0.0 to 1.0, representing the current value
    /// in relation to the minimum and maximum, where 0.0 is the minimum and
    /// 1.0 is the maximum
    /// 
    /// # Examples
    /// 
    /// ```
    /// use clamped_values::ClampedValue;
    /// 
    /// let clamped_value = ClampedValue::<u8>::new(50, 75, 100);
    /// 
    /// assert_eq!(clamped_value.percent_f64(), 0.5);
    /// ``` 
    pub fn percent_f64(&self) -> f64 {
        self.percent::<f64>()
    }
}

// generic version of the percent code so that we can use the same logic for f32 and f64
impl<T: Sub<Output = T> + PartialOrd + Clone> ClampedValue<T> {
    fn percent<U>(&self) -> U
    where
        U: Div<Output = U>,
        T: Into<U>,
    {
        // we can sub these values by self.min without worrying about overflow due to the fact that
        // self.min is ALWAYS smaller than or equal to self.value and self.max
        (self.value.clone() - self.min.clone()).into()
            / (self.max.clone() - self.min.clone()).into()
    }
}

// For the following three impl blocks, the "Saturating" version of the operation is implemented as opposed to
// the regular operation due to the fact that the regular operations allow the possibility of
// overflowing (in debug) or wrapping (in release), which is unexpected behaviour.

impl<T: SaturatingAdd + PartialOrd + Clone> AddAssign<T> for ClampedValue<T> {
    /// Adds `rhs` to the current value, saturating at the minimum or maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// use clamped_values::ClampedValue;
    ///
    /// let mut clamped_value = ClampedValue::new(0, 5, 10);
    ///
    /// clamped_value += 3;
    ///
    /// assert_eq!(*clamped_value.value(), 8);
    /// ```
    fn add_assign(&mut self, rhs: T) {
        self.value = self.value.saturating_add(&rhs);
        self.clamp();
    }
}

impl<T: SaturatingSub + PartialOrd + Clone> SubAssign<T> for ClampedValue<T> {
    /// Subtracts `rhs` from the value, saturating at the minimum or maximum.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use clamped_values::ClampedValue;
    /// 
    /// let mut clamped_value = ClampedValue::new(0, 5, 10);
    /// 
    /// clamped_value -= 4;
    /// 
    /// assert_eq!(*clamped_value.value(), 1);
    /// ```
    fn sub_assign(&mut self, rhs: T) {
        self.value = self.value.saturating_sub(&rhs);
        self.clamp();
    }
}

impl<T: SaturatingMul + PartialOrd + Clone> MulAssign<T> for ClampedValue<T> {
    /// Multiplies the value by `rhs`, saturating at the minimum or maximum.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use clamped_values::ClampedValue;
    /// 
    /// let mut clamped_value = ClampedValue::new(0, 2, 10);
    /// 
    /// clamped_value *= 3;
    /// 
    /// assert_eq!(*clamped_value.value(), 6);
    /// ``` 
    fn mul_assign(&mut self, rhs: T) {
        self.value = self.value.saturating_mul(&rhs);
        self.clamp();
    }
}

impl<T: Div<Output = T> + PartialOrd + Clone> DivAssign<T> for ClampedValue<T> {
    /// Divides the value by `rhs`, saturating at the minimum or maximum.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use clamped_values::ClampedValue;
    /// 
    /// let mut clamped_value = ClampedValue::new(0, 8, 10);
    /// 
    /// clamped_value /= 2;
    /// 
    /// assert_eq!(*clamped_value.value(), 4);
    /// ``` 
    fn div_assign(&mut self, rhs: T) {
        self.value = self.value.clone() / rhs;
        self.clamp();
    }
}

#[cfg(test)]
mod tests {
    use crate::ClampedValue;

    #[test]
    fn new() {
        ClampedValue::new(10, 20, 30);
    }

    #[test]
    #[should_panic]
    fn new_min_larger_than_max() {
        ClampedValue::new(30, 10, 20);
    }

    #[test]
    #[should_panic]
    fn new_value_outside_min_max() {
        ClampedValue::new(10, 40, 30);
    }

    #[test]
    fn set() {
        let mut clamped_value = ClampedValue::new(10, 50, 110);

        clamped_value.set_min(22);
        assert_eq!(*clamped_value.min(), 22);

        clamped_value.set_max(99);
        assert_eq!(*clamped_value.max(), 99);

        clamped_value.set(55);
        assert_eq!(*clamped_value.value(), 55);

        clamped_value.set(1000);
        assert_eq!(*clamped_value.value(), *clamped_value.max());

        clamped_value.set(-1000);
        assert_eq!(*clamped_value.value(), *clamped_value.min());
    }

    #[test]
    #[should_panic]
    fn set_min_larger_than_max() {
        ClampedValue::new(10, 20, 30).set_min(40);
    }

    #[test]
    #[should_panic]
    fn set_min_larger_than_value() {
        ClampedValue::new(10, 20, 30).set_min(25);
    }

    #[test]
    #[should_panic]
    fn set_max_smaller_than_min() {
        ClampedValue::new(10, 20, 30).set_max(0);
    }

    #[test]
    #[should_panic]
    fn set_max_smaller_than_value() {
        ClampedValue::new(10, 20, 30).set_max(15);
    }

    #[test]
    fn percent() {
        // works with all positive numbers
        let c = ClampedValue::<u8>::new(75, 100, 125);
        assert_eq!(c.percent_f32(), 0.5);
        assert_eq!(c.percent_f64(), 0.5);

        // works with all negative numbers
        let c = ClampedValue::<i8>::new(-100, -40, -20);
        assert_eq!(c.percent_f32(), 0.75);
        assert_eq!(c.percent_f64(), 0.75);

        // works with mix of negative and positive numbers
        let c = ClampedValue::<i8>::new(-40, -10, 40);
        assert_eq!(c.percent_f32(), 0.375);
        assert_eq!(c.percent_f64(), 0.375);
    }

    #[test]
    fn operations() {
        let mut clamped_value = ClampedValue::new(20, 20, 40);

        clamped_value += 100;
        assert_eq!(*clamped_value.value(), *clamped_value.max());

        clamped_value -= 100;
        assert_eq!(*clamped_value.value(), *clamped_value.min(),);

        clamped_value *= 100;
        assert_eq!(*clamped_value.value(), *clamped_value.max(),);

        clamped_value /= 10;
        assert_eq!(*clamped_value.value(), *clamped_value.min());
    }
}
