use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;


fn main() {

    //  File::open will return Result<File,std::io:Error>
    //  so that we can handle it later enum match mechanism
    //  Other functions may return Result<T,E> object with different
    //  T and E types 
    let greeting_file_result = File::open("enums.rs");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    //  We can expand the handling by trying to find the error further
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // Also Result<T,E> has builtin functions to perform some
    // handling operations by himslef without using enum-matching
    let _greeting_file = File::open("hello.txt").unwrap();          //  Just prints the error message when it occurs
    let _greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");    //  Same except we provide the message  

    
}

//  Separate function to showcase the error propogation
fn _user_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),    // Instead of calling panic! we return the error for the
                                    // caller to handle it      
    };
    
    let mut username = String::new();
    
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),  
        Err(e) => Err(e),
    }

    //  returns Ok wrapped username string or Error 
    //  wrapped error message for the calling function
}


//  And finally we got into syntax sugar of the latest example
//  which is very commonly used
fn _user_from_file_short() -> Result<String, io::Error> {
    
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
