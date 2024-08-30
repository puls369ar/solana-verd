fn main() {
    //      Common Collections
    ////    Vectors

    // Vec is in `std::vec` module in `alloc` subcrate. imported
    // automatically in a prelude

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let res1 = v[0];    // Returning the copy of the element from heap
                        // If there is no trait `copy` for this type then
                        // it'll be moved out from the vector
    let res1 = &v[0];   // Returning the reference of the element from heap

    //  This line will give a RUNTIME error saying that the vector doesn't have 
    //  20th element
    //  println!("First element is {}", &v[20]);

    //  Better way to access Vector elements without interrupting the RUNTIME
    match v.get(20) {
        Some(twnth) => {
            println!("20th element: {}", twnth)
        },
        None => {
            println!("No 20th element")
        }
    }

    //  Strings
    //  HashMaps

}