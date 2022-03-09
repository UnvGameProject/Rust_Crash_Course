/*
Primitive types --
Integers:u8,i8,u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in
memory) unsigned(u) as not negative or no sign, (i) as integer
Floats f32, f64
Booleans
Characters (char)
Tuples
Arrays -use vectors as growable arrays 
 */

 // rust needs to know the type of all variables at compile time, however, the compiler 
 // can usually infer the type based on the value and how its used


pub fn run(){
    //Default is i32

    let x = 1;

    //default is "f64"
    let y = 2.5;

    //add explicit type 
    let z: i64 = 45454454545454;

    //Find Max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Set Boolean
    let is_active = true;
    println!("{:?}", (x, y, z, is_active));

    //Get boolean from expression
    let is_greater: bool = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

}