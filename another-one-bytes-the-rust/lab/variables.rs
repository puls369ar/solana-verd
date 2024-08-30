fn main() {
    //  Case1
    //  let var;            error[E0282]: type annotations needed

    //  Case2
    //  let var = 5;
    //  var = 4;            error[E0384]: cannot assign twice to immutable variable `var`

    //  Case4               
    let mut var1 = 5;       //  But we can make modify mutable variables
    var1 = 4;               //  Those are defined by adding `mut` keyword to
                            //  the variable definition

    //  Case3
    let var2 = 5;   
    let var2 = 4;            //  No Error. Redefining allowed in RUST

    //  Constants
    const MY_AGE: u32 = 17;  //   Defining constants in RUST  


    //  Types
    let a: u8 = 0;              //  [0;2^8-1]
    let b: u16 = 0;             //  [0;2^16-1]
    let c: u32 = 0;             //  [0;2^32-1]          default
    let d: u64 = 0;             //  [0;2^64-1]
    let e: u128 = 0;            //  [0;2^128-1]

    let f: u8 = 0;              //  [-2^7;2^7-1]
    let g: u16 = 0;             //  [-2^15;2^15-1]
    let h: u32 = 0;             //  [-2^31;2^31-1]      default
    let i: u64 = 0;             //  [-2^63;2^63-1]
    let j: u128 = 0;            //  [-2^127;2^128-1]

    let k: f32 = 0.0;             //  IEEE754 single precision    default
    let l: f64 = 0.0;             //  IEEE754 double precision

    let true_or_false: bool = true;     //true,false

    let letter: char = 'A';             //UTF32, 4 bytes

    let p: isize = 0;
    let q: usize = 0;                   //exotic types i'll learn about later
}