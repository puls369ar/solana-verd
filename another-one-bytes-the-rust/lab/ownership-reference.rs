fn main() {

   

    //  Moving and Ownership
    let mut a: String = String::from("Hello");
    let b: String = a;                  // b takes the ownership from a, a is emtpy now
    a = String::from("World");          // pointer a is reevaluated, it is allowed as a was set as mutable
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
}

//  Movement and Ownership#
fn print_string(text: String) {
    println!("{}", text);    // `text` is now the owner of "Hello"
}                            // in print_string() function's scope,
     