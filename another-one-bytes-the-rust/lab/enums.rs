fn main() {
   
    //  Enums
    let emily: Animals = Animals::Tiger(56); 
    let bob: Animals = Animals::Dog(String::from("Bob"));         
   
    println!("{:?}", bob);


    ////   Enums together with Match statement
    match bob {
        Animals::Tiger(age) => {
            println!("The age of the tiger is {}", age)
        },
        Animals::Elephant => {
            println!("Elephant is a good animal")
        },
        Animals::Dog(name) => {
            println!("The name of the dog is {}", name)
        },
        Animals::Wolf => {
            println!("Wolf is a dangerous animal")
        },
    };


    ////    Option<T> enum with it's Some(T) and None options
    //      this comes from `std::option` module  

    //  With this can we have empty variables that was forbidden by core RUST design
    let mut number: Option<i32> = Some(5);

    match number {
        Some(value) => println!("The number is {}", value),
        None => println!("No number"),
    }


    number = None;  // modifying the value to be None of type i32
    match number {
        Some(value) => println!("The number is {}", value),
        None => println!("No number"),
    }
}

//  Enums#

//  Enum options in RUST can contain additional data inside
//  that are initialized when initializing them
#[derive(Debug)]
enum Animals {
    Tiger(i32),
    Elephant,
    Dog(String),
    Wolf
}
