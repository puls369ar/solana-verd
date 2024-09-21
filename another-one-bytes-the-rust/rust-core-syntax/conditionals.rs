fn main() {

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
}