#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8); // Q7 is a tuple struct.

impl From<f64> for Q7 {
    fn from(value: f64) -> Self {
        // Coerces any out-of-bounds input to fit.
        if value >= 1.0 {
            Q7(127)
        }
        else if value <= -1.0 {
            Q7(-128)
        }
        else {
            Q7((value * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(value: Q7) -> Self {
        // Equivalent to the iteration approach taken in listing 5.9.
        (value.0 as f64) * 2f64.powf(-7.0)
    }
}

// By design, it’s safe to convert from f32 to f64. A number that can
// be represented in 32 bits, it can also be represented in 64 bits.
impl From<f32> for Q7 {
    fn from(value: f32) -> Self {
        Q7::from(value as f64)
    }
}

// Generally, converting an f64 into a f32 risks a loss of precision.
// In this application, that risk doesn’t apply as we only have numbers
// between –1 and 1 to convert from.
impl From<Q7> for f32 {
    fn from(value: Q7) -> Self {
        f64::from(value) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7::from(1.));
        assert_eq!(Q7::from(-10.), Q7::from(-1.));
    }

    #[test]
    fn f32_to_q7() {
        let n1: f32 = 0.7;
        let q1 = Q7::from(n1);

        let n2: f32 = -0.4;
        let q2 = Q7::from(n2);

        let n3: f32 = 123.0;
        let q3 = Q7::from(n3);

        assert_eq!(q1, Q7(89));
        assert_eq!(q2, Q7(-51));
        assert_eq!(q3, Q7(127));
    }

    #[test]
    fn q7_to_f32() {
        let q1 = Q7::from(0.7);
        let n1 = f32::from(q1);
        assert_eq!(n1, 0.6953125);

        let q2 = Q7::from(n1);
        let n2 = f32::from(q2);
        assert_eq!(n1, n2);
    }
}
