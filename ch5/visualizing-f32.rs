// Similar constants are accessible via the std::f32 module.
const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 42.42;

    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", n, n_);
    println!("field    |  as bits | as a real number");
    println!("sign     |        {:01b} | {}", sign, sign_);
    println!("exponent | {:08b} | {}", exp, exp_);
    println!("mantissa | {:023b} | {}", frac, mant);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    // Strips 31 unwanted bits away by shifting these nowhere, leaving only the sign bit.
    let sign = (bits >> 31) & 1;
    // Filters out the top bit with a logical AND mask, then strips 23 unwanted bits away.
    let exponent = (bits >> 23) & 0xff;
    // Retains only the 23 least significant bits via an AND mask.
    let fraction = bits & 0x7fffff;

    // The mantissa part is called a fraction here as it becomes the mantissa once it’s decoded.
    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    // Converts the sign bit to 1.0 or –1.0).
    // Parentheses are required around –1.0_f32 to clarify operator precedence as method calls rank higher than a unary minus.
    let signed_1 = (-1.0_f32).powf(sign as f32);

    // exponent must become an i32 in case subtracting the BIAS results in a negative number; then 
    // it needs to be cast as a f32 so that it can be used for exponentiation.
    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf( i_ - 23.0 );
            mantissa += weight;
        }
    }

    (signed_1, exponent, mantissa)
}

// Cheats a bit by using f32 values in intermediate steps. Hopefully, it is a forgivable offense.
fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}
