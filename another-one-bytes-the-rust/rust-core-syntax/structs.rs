fn main() {
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
