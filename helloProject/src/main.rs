fn main() {
    // signed and unsigned integers
    // signed can be + or - (i8, i16, i32 i64, i128)
    // unsi   gned can only be + (u8, u16, u32, u64, u128)
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // diff btw i32(32 bits) and i64(64 bits)
    // range:
    // i32 range can vary from (-2^31 to +2^31)
    // i64 range can vary from (-2^63 to +2^63)

    // max values for i32 and i64
    let e: i32 = 2147483647 ;
    let i: i64 = 9223372036854775807;
    println!("Max value of i32: {}", e);
    println!("Max value of i64: {}", i);

    // Floats [Floating point types/ fractional integers]
    // f32, f64
    let pi: f64 = 3.14;  
    println!("Value of pi is: {}", pi);

    // Boolean Values true, false
    let isSnowing: bool = true;
    println!("Is it snowing: {}", isSnowing);

    // Character type - single character e.g a, z
    // Note: string is double quote, char is single quote
    let letter: char = 'a';
    println!("First letter of the alphabet is: {}", letter)
}
