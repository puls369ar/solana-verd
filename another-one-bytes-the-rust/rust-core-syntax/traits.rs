
fn main() {
    let arta: Person = Person {
        name: String::from("Arta Johan"),
        age: 25
    };

    println!("{}", arta)
}

struct Person {
    name: String,
    age: i32
}


//  Here we implemented the trait that is
//  imported by default in a prelude std lib and it let's
//  us describe how the structure which we implement the trait for
//  will return the string when we print it's instance via `println` function
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "The name of the person is {}\nThe age of the person is {}", self.name, self.age)
    }
}