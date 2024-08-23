---
title: Another one bytes the RUST
date: '2024-06-24'
tags: ['crypto','programming','rust']
draft: false
summary: This article will cover everything I've learnt about RUST programming language 
---

# Some Details
Rust is a compiled language which means the human readable source code
is being converted into the type of executable file that can then be ran
by any appropriate machine, unlike interpreted languages like python
where you always need interpreter of that certain language to run
the code straight from the source code. 
     

#   Running RUST
The compiler for rust is `rustc`, but it is more convenient to work
with the projects using cargo package manager. Follow instructions
in [this](https://www.rust-lang.org/tools/install) webpage to install
them both.

```bash
rustc main.rs
```

will create executable binary in a current folder

To work with cargo package manager we 
first create the project and give it the name.

```bash
cargo new rust-project
``` 

This will generate
folder as we named the project with the following structure inside

```bash
rust-project
├── Cargo.toml
└── src
    └── main.rs
```

where `main.rs` is just a simplest hello world program and `Cargo.toml` is
the file to give some information about the project and also 
write the dependencies out from

```
[package]
name = "rust-project"
version = "0.1.0"
edition = "2024"


[dependencies]
```

cargo build 

will compile the project and we get many new files in project 
after that

```
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── build
        ├── deps
        ├── examples
        ├── incremental
        ├── rust-project
        └── rust-project.d
```

Here target folder is generated which contains binaries and other neccessary
files for further development, but the most important for us is executable
in nested debug folder. By performing
```bash
cargo run
```
it will automatically start executing the binary file. If there isn't one 
it will perform the whole proccess starting from compilation.


# Variables
It is statically and strongly typed programming language with no
need to set types explicitly in variable definition. But creating 
the variable that has no value at all will result in a compiletime error.

```rust
fn main() {
    let var;
}

//error[E0282]: type annotations needed
```

Variables here are ***immutable*** by default. So if we try to change the value
of a variable we will get an error

```rust
fn main() {
    let var = 5;
    var = 4;
}

//error[E0384]: cannot assign twice to immutable variable `var`
```

There are two workarounds for this problem. Setting
the variable to be mutable or redefining the variable anew.

```rust
fn main() {
    let mut var1 = 5;
    var1 = 4;

    let var2 = 5;
    let var2 = 4;
}
```

Yes it sounds very strange, but here we can redefine the variable
with the same name even with the different type. In any case the value defined
previously will remain in a memory cell until the scope of the definition is ended
and destructor is called for this ***shadowed*** variable.  Also it can be confusing
that immutable variables are very much like ***constant***s, but there are few differences.
Constants can't be converted into mutable by simply adding
`mut` keyword in definition, also constants can't be redefined and you have to
implicitly set the type of the constant.
It is a common convention to write constants in an uppercase with underscores.

```rust
const MY_AGE: u32 = 17;
```

# Types
There two groups of types in RUST, Scalar and Compound.
Scalar types are the simplest ones like the integer
that can be signed and unsigned have the capacity
based on a different amount of bytes the type has. 
Also floating point numbers with single and double precisions,
boolean variables and characters. Remember you don't
have to set variables explicitly, the following demo
is just a showcase.

```rust
let a: u8 = 0;              //  [0;2^8-1]
let b: u16 = 0;             //  [0;2^16-1]
let c: u32 = 0;             //  [0;2^32-1]          default
let d: u64 = 0;             //  [0;2^64-1]
let e: u128 = 0;            //  [0;2^128-1]

let f: u8 = 0;              //  [-2^7;2^7-1]
let g: u16 = 0;             //  [-2^15;2^15-1]
let h: u32 = 0;             //  [-2^31;2^31-1]      default
let i: u64 = 0;             //  [-2^63;2^63-1]
let j: u128 = 0;            //  [-2^127;2^128-1]

let a: f32 = 0;             //  IEEE754 single precision    default
let b: f64 = 0;             //  IEEE754 double precision

let true_or_false: bool = true;     //true,false

let letter: char = 'A';             //UTF32, 4 bytes

let p: isize = 0;
let q: usize = 0;                   //exotic types i'll learn about later
```

Read more about the ways numbers are stored in 
my (article)[#] where I deep dive more in CS staff too.
Just two compound types. Tuples and arrays

```rust
let arr: [int32, _] = [1,2,3];              //[int32, _] is a syntax sugar, it is the same as [int32, int32, int32]
let tup: (i32, char) = (126, 'a');

let digit: int32 = arr[0];
let character: char = tup.1;

println!("{} {}", digit, character);           //1 a    
```

Arrays are fixed size in Rust, otherwise we will need
to allocate array's memory dynamically which is a different topic.
Think of tuple as arrays that can have
elements of different values though. Even if we make tuples or
arrays mutable we won't be able to change their sizes or types
of particular element in tuple, but only change element's values or
change the whole tuple itself.

# Functions
```rust
fn main() {
    x: i32 = 1;
    y: i32 = 2;
    res: int32 = sum(x,y);
    println!("{}", res);    // 3
}

    fn sum(a:i32, b:i32) -> i32 {
        return a+b;
        //just a+b would work too
    }
```

It is a mandatory to explicilty define argument types
when creating a function, aslo it the function returns
some value then the type of it should be too defined
when creating the function. We don't have to explicitly write
return keyword as the last statement without ending ; sing 
will automatically be considered as the returnable one. But I will
the classical way more natural and convinient for me.

> I don't feel I'll use the possibility to pass explicit
> variable definition, because I think that having types written for
> variables will give more natural control over the code and showmark
> more esthetic code. Besides in some places in RUST it is a mandatory to 
> keep such rules, so keeping them everywhere in a code will prevent 
> confusion and will create a good habit of writing a nice code.

# Conditions
We have classic if statement
```rust
let number: i32 = 7;

if number < 5 {
    println!("Number is smaller then 7");
} else if number == 7 {
    println!("Number is 7");
} else {
    println!("Number is greater then 5");
}
```

inline conditional expression
```rust
let a: i32 = if true {
    5
} else {
    6       // Never end lines with ; here!!!
};
```

match statement
```rust
let number: i32 = 4;

let option1: i32 = 1;
let option2: i32 = 2;
let option3: i32 = 3;
let option4: i32 = 4;

match {
    option1 => println!("Number is {}", option1),
    option2 => println!("Number is {}", option2),
    option3 => println!("Number is {}", option3),
    option4 => println!("Number is {}", option4),
    _       => println!("There is no matches in options"),
}
```

Also match mechanism is uniquely used with (enums)[#]

# Ownership and Moving
RUST has a very interesting memory managment mechanism
which is nor Garbage Collector neither Manual allocation. Here by default
all static values are calculated during the compiletime and initialized 
in order depended on scopes.
Any dynamically initialized value in heap can be referenced only from
one pointer though. This pointer is called the owner of the value.


```rust
let a: String = String::from("Hello");
```

Now if we assign a to another newly initialized variable
with the same type it will not just create the second pointer 
that addresses to the value "Hello" in heap, but also it will empty
the value of a variable.

```rust
let mut a: String = String::from("Hello");
let b: String = a;                  // b takes the ownership from a, a is emtpy now
a: String = String::from("World");  // pointer a is reevaluated, it is allowed as a was set as mutable
let a: i32 = 4;                     // The name a is shadowed, but old pointer a will remain in a memory untill destruction as well as "World" in heap
```

Now a is an empty pointer of a type String. If it is
mutable we could give it another String value, otherwise
we can just redefine it. This is called moving. The scope of
the moving value depends on it's owner, so the scope can be changed 
in the code.

```
fn main() {
    let a: String = String::from("Hello");
    print_string(a);            // "Hello" is moved
    println!("{}", a);          // error[E0382]: borrow of moved value
}

fn print_string(text: String) {
    println!("{}",text);    // "Hello" owner now is text
                            // in print_string function's scope,
                            // it will be destroyed when the function ends
}
```

# References
But what if we want to give the value that is in heap
to other functions or other elements? One solution is 
to simply copy and transfer it
```rust
let a: String = String::from("Hello");
let b: String = a.clone();  // b and a now both have 
                            // two different "Hello" values
                            // present in heap
```

but better solution is to 
give not the ownership, but 
the reference of that value.

```rust
let a: String = String::from("Hello");
let b: &str = &a;    // b is the reference to the a
```

For reference as for any other variable it is a 
mandatory to be mutable to be able to change the 
value of it's referee. And hence it is logical
that mutable references can point only to
mutable varaibles. But there are several restrictions.
- We can't have more then one mutable references for any variable.
- Mutable variable can have immutable references, but it'll loose
the availability to have mutable references after. The reason is pretty logical,
If both mutable and immutable references existed immutable one would face
change in a value made by mutable reference.
- We won't be able to change the value of the variable from the variable itself
while it is borrowed by the reference.
- References are destructed after being used in a statement first time.
- References must always be valid and must always point to some variable,
if we get a dangling reference the program will halt in compiletime.


> I see now why all of this restrictions exist, 
> these all are done to prevent issues when working
> with concurrency in RUST, to make sure 
> that the code won't be able to change the state
> of variable simultaneously from different pointers
> that are in different threads.


# Slices
Now that we have

# Structs
```rust
struct Person {
    name: String,
    age: i32,
    sex: bool
}

fn main() {
    let mut p1: Person = Person {
        name: String::from("Arta Johan"),
        age: 24,
        sex: true
    }

    p1.name = String::from("Emma Johan");       // Both types of modification
    p1 = Person {                               // are allowed when the variable is mutable 
        name: String::from("Jenna Johan"),      // keeping struct's types, similiar to tuples and arrays
        age: 21,
        sex: false
    }
}
```

Also we can create structs without giving them field names, these are called tuple structs
```rust
struct Person(String, i32);

fn main() {
    let p: Person = Person(String::from("Arta"), 25);
}
```

Also there are unit structs that we'll learn together with struct's complex
types when discussing (traits)[#]
We can also define methods and associated functions (static functions)
for structs

```rust
struct Person {
    name: String,
    age: i32,
    sex: true
}

impl Person {
    fn print_brief_intro(&self) {
        println!("{} is {} years old {}", self.name, self.age, if self.sex {"male"} else {"female"});
    } 
}

// RUST let's implementations to exists in separate blocks
// it is advantageous for readability
impl Person {
    fn print_person_motto(name: &String) {
        println!("All Persons are free");
    }
}
```

> There are no classes in RUST, but only structs that
> implement the same functionality. And YES!!! thank you
> dear developers for this. I never really understood why
> in C++ there are two paradigms having the same concept one of which
> is just artifically avoided via syntax sugar.


# Enums
```rust

#[derive(Debug)]
enum Animals {
    Tiger,
    Elephant,
    Dog,
    Wolf
}

fn main() {
   let emily: Animals = Animals::Tiger; 
   let bob: Animals = Animals::Dog;

   println!("{:?}", bob);
}
```

# Packages, Crates and Modules

- There are two types of crates, binary and library
- Binary crates are the ones that are gonna be the entry points for the compiler execution
- Library crates are just collections of features and functionalities that can be referenced and used in other RUST programs

- Each package should have at least one crate, wether library or binary
- Each package can have only one library crate, but many binary crates
- In a single package there can be additional binary crates that are in src/bin folder other then the primary binary crate
- If the package has only a library crate then it is a library package
- If the package has only additional binary crates it is nor executable neither library package


- Primary binary crate is defined by it's root file that is named as `src/main.rs`
- Library crate is defined by it's root file that is named as `src/lib.rs`
- Each binary crate shoud have main() function


- multiple modules can be defined in a single .rs file, but it is more convenient
    to organize them in a way that each module is separate in it's own .rs file

```rust:src/main.rs
mod alfaModule {
    pub fn hello() {
        println!("Hello from Alpha")
    }
}

fn main() {
    println!("Hello World")
}
```


```rust:src/main.rs
mod alphaModule;

fn main() {
    println!("Hello World")
}
```

```rust:src/alphaModule.rs
fn hello() {
    println!("Hello from A")
}
```

To call the function hello() we should set it's path through modules.
There are _**relative**_ and _**absolute**_ paths

Let's add a nested module inside alpaModule
```rust:src/alphaModule.rs
mod betaModule; 

fn hello() {
    println!("Hello from A")
}
```
```rust:src/alphaModule/betaModule.rs

fn bye() {
    println!("Bye from Beta")
}
```

```rust:src/main.rs
mod alphaModule;

fn main() {
    // alphaModule::betaModule::bye()           Relative path starts from the 
    //                                          closest sibiling module.
    //                                          In this case main function is a 
    //                                          sibiling to the alphaModule, so we start there

    crate::alphaModule::betaModule::bye()   //  Absolute path should be specified
                                            //  from the root of the module tree 
                                            //  which is named _**crate**_ by default
                                            //  in a custom project
}
```

The second method is more prefered for us to see where the item comes from.
There is a keyword `use` which allows to set the path once for the scope and use 
the end address as a reference for the rest of the scope

```rust:src/main.rs
mod alphaModule;

use crate::alphamodule::betaModule

fn main() {
    betaModule::bye()
}
```

also `pub use` will export that path, giving the possibility to use the path
not only in the scope where it was set, but in all external and internal
scopes as well. 

When bringing the path into the current scope it is a convention to specify it
not fully to the item, but to the module containing the item. This gives
more readability and distinction to prevent name conflicts from different modules

All items and modules are private to the external ones and public to
sibilings and nested items by default
So in our example code won't still compile because both `hello()` and `bye()` functions
are private. To make it work we should modify their access. `pub` keyword will
make any item and module accessible for all external modules 

```rust:src/alphaModule.rs
pub mod betaModule; 

pub fn hello() {
    println!("Hello from A")
}
```
```rust:src/alphaModule/betaModule.rs

pub fn bye() {
    println!("Bye from Beta")
}
```
