fn main() {
  
  
    //  Generic Function
    let number_list = vec![34,50,35,200];
    let char_list = vec!['y','m','a','q'];

    let result = largest(&number_list);
    println!("Largest number {result}");
    let result2 = largest(&number_list);
    println!("Largest char {result}");

    
}













//  Generic Function#
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//  Generic Struct
struct Human<T> {
    x: T,
    y: T
}


//  Generic implementation module for `Human<T>` structure
impl<T> Human<T> {
    fn getX(&self) -> &T {
        &self.x
    }
}

//  Non Generic implementation module for particular type of `Human<f32>` structure
impl Human<f32> {
    fn get_coords(&self) -> [&f32; 2] {
        [&self.x,&self.y]
    }
}


//  Non Generic implementation module for particular type of `Human<f32>` structure
impl<T: std::fmt::Display> Human<T> {
    fn print_coords<U: std::fmt::Display>(&self, description: &U) {
        println!("{}: {}", description, self.x)
    }
}




    