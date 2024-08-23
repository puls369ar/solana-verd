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

    
    //  Arrays and Tuples
    let arr: [i32; 3] = [1,2,3];      //  [int32, _] is a syntax sugar, it is the same as [int32, int32, int32]
    let tup: (i32, char) = (126, 'a');

    let digit: i32 = arr[0];          //  1                 
    let character: char = tup.1;        //  a

    //  Functions: Calling sum(a:i32, b:i32) -> i32:    
    //  jump to line
    println!("The sum of 5 and 4 is {}", sum(5,4));           //  9
    

    // Conditions
    let number: i32 = 7;

    if number < 5 {
        println!("Number is smaller then 7");
    } else if number == 7 {
        println!("Number is 7");
    } else {
        println!("Number is greater then 5");
    }
    

    ////  Inline Conditional Expression
    
    let a: i32 = if true {
        5
    } else {
        6       // Never end lines with `;` here!!!
    };
    

    ////  Match Statement
    let number: i32 = 4;

    let option1: i32 = 1;
    let option2: i32 = 2;
    let option3: i32 = 3;
    let option4: i32 = 4;

    match number{
        option1 => println!("Number is {}", option1),
        option2 => println!("Number is {}", option2),
        option3 => println!("Number is {}", option3),
        option4 => println!("Number is {}", option4),
        _       => println!("There is no matches in options"),
    }


    //  Moving and Ownership
    let mut a: String = String::from("Hello");
    let b: String = a;                  // b takes the ownership from a, a is emtpy now
    a = String::from("World");  // pointer a is reevaluated, it is allowed as a was set as mutable
    let a: i32 = 4;                     // The name a is shadowed, but old pointer a will remain in a memory untill destruction as well as "World" in heap

    ////    Negative Example
    let a: String = String::from("Hello");
    print_string(a);            // "Hello" is moved
    //  println!("{}", a);      error[E0382]: borrow of moved value


    ////    The Fix

    ////    Solution#1
    let a: String = String::from("Hello");
    let b: String = a.clone();  // b and a now both have 
                                // two different "Hello" values
                                // present in heap
    println!("{}", a);          // No Error


    ////    Solution#2  (a better one)

    let a: String = String::from("Hello");
    let b: &str = &a;    // b is the reference to the a


    /*

    For reference as for any other variable it is a 
    mandatory to be mutable to be able to change the 
    value of it's referee. And hence it is logical
    that mutable references can point only to
    mutable varaibles. But there are several restrictions.
    -   We can't have more then one mutable references for any variable.
    -   Mutable variable can have immutable references, but it'll loose
        the availability to have mutable references after. The reason is pretty logical,
        If both mutable and immutable references existed immutable one would face
        change in a value made by mutable reference.
    -   We won't be able to change the value of the variable from the variable itself
        while it is borrowed by the reference.
    -   References are destructed after being used in a statement first time.
    -   References must always be valid and must always point to some variable,
        if we get a dangling reference the program will halt in compiletime.


    I see now why all of this restrictions exist, 
    these all are done to prevent issues when working
    with concurrency in RUST, to make sure 
    that the code won't be able to change the state
    of variable simultaneously from different pointers
    that are in different threads.

    */


    
    //  Structs
    
    let mut p1: Person = Person {
        name: String::from("Arta Johan"),
        age: 24,
            sex: true
    };

    p1.name = String::from("Emma Johan");       // Both types of modification
    p1 = Person {                               // are allowed when the variable is mutable 
        name: String::from("Jenna Johan"),      // Just keep the types (similiar to tuples and arrays)
        age: 21,
        sex: false
    };

    

    ////  Tuple Structs
    let p: Car = Car(String::from("BMW"), 25);

    /*  Also there are unit structs that we'll learn together with struct's complex
    types when discussing (traits)[#]   */


    // There are no classes in RUST, but only structs that
    // implement the same functionality. And YES!!! thank you
    // dear developers for this. I never really understood why
    // in C++ there are two paradigms having the same concept one of which
    // is just artifically avoided via syntax sugar.
    

    //  Enums
   let emily: Animals = Animals::Tiger; 
   let bob: Animals = Animals::Dog;

   println!("{:?}", bob);



    


}


//  Functions#
fn sum(a:i32, b:i32) -> i32 {   
    a+b         //  `return a+b` would work too, 
                //  it is just a conventional syntax sugar

                //  Argument types and return type should 
                //  be explicitly defined
}

//  Movement and Ownership#
fn print_string(text: String) {
    println!("{}", text);    // `text` is now the owner of "Hello"
                            // in print_string() function's scope,
                            // it will be destroyed when the function ends
}


//  Structs#
struct Person {
    name: String,
    age: i32,
    sex: bool
}

////  Tuple Structs#
struct Car(String, i32);

////    We can also define methods and associated functions (static functions)
////    RUST let's implementations to exists in separate blocks
////    it is advantageous for readability
impl Person {
    fn print_brief_intro(&self) {
        println!("{} is {} years old {}", self.name, self.age, if self.sex {"male"} else {"female"});
    } 
}



//  Enums#
#[derive(Debug)]
enum Animals {
    Tiger,
    Elephant,
    Dog,
    Wolf
}


    
