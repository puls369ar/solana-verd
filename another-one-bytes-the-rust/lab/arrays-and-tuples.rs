fn main() {
   
    //  Arrays and Tuples
    let arr: [i32; 3] = [1,2,3];            //  [int32, _] is a syntax sugar, it is the same as [int32, int32, int32]
    let tup: (i32, char) = (126, 'a');

    let digit: i32 = arr[0];                //  1                 
    let character: char = tup.1;            //  a

    println!("The digit is {} \nThe character is {}", digit, character);
}