fn main() {
    println!("The sum of 5 and 4 is {}", sum(5,4));           //  9
}

fn sum(a:i32, b:i32) -> i32 {   
    a+b         //  `return a+b` would work too, 
                //  it is just a conventional syntax sugar

                //  Argument types and return type should 
                //  be explicitly defined
}