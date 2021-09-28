# RUST PROGRAMMING LANGUAGE

### Why Rust?

* Statically Typed Language
* Better memory safety due to the compiler
* Easier concurrency
* Powerfull, crossplatform commandline tools
* Embedded Devices
* System programming (linux kernal programming)
* Have webdevelopment support as well

### Installing Rust in linux

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1
```

### Checking Rust version

```
rustup --version
rustup 1.24.3
```

### Creating sample Hello World Program

```
fn main(){
    println!("Hello World");
}
```

To Run this code you can use `rustc` Rust Compiler or `cargo` Rust Package manager

```
$ rustc Hello.rs
```

This would create a binary executable file which you can run

```
$ ./Hello 
Hello World
```

### Create a Project with Cargo

The previous example is just a functional one. However in realworld we will use `Cargo` to initialize and Build the projects.

#### To create a new project folder with Cargo

```
$ cargo new ProjectName
```

The above command will create new Folder `ProjectName` and would initialize the dependency files. If you want to create the project files in current working directory you can directly initialize with the below command

```
$ cargo init
```

**Note:** It is recommended to create the project folder in snake case like `hello_world`

#### The directory structure

We will have a `src` folder in the directory with a sample `main.rs` file. This is the file to which we write our code. Then we have a file named `Cargo.toml` which have the package name, version, dependencies if any. 
We also have `.gitignore` file. This file is created just to specify the ignore path. We don't want to commit our binary executable files to git. 

#### To compile and run using cargo

To build and run the `main.rs` file, we need to run the below command

```
$ cargo run
   Compiling Hello v0.1.0 (/media/sundar/Data/Programming/Practices/RUST/Hello)
warning: crate `Hello` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `hello`

warning: `Hello` (bin "Hello") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 8.26s
     Running `target/debug/Hello`
Hello, world!
```

As you could see this command compiled and ran the `main.rs` file and shown the output in last line. After you compiled there will be new directory `target`. This is where the executable files stored.
Mostly the executable files are inside `/target/debug/` directory.

#### To Build without running

If you just want to build without running you can use

```
$ cargo build
```

#### To release in production 

```
$ cargo build --release
   Compiling hello_world v0.1.0 (/media/sundar/Data/Programming/Practices/RUST/hello_world)
    Finished release [optimized] target(s) in 0.55s
```

This creates a release folder with binary executables and dependencies

### Modules

We must have the `main.rs` file. whatever additional code that we add could be added as new `.rs` file and can be included in the `main.rs` file

For example, if you want to add more code, just add another file in `/src/` directory in project folder. 

*print.rs*

```
pub fn run(){
    // To Print something from this file
    println!("This is printing from print.rs file");
}
```

Now to include this module in the main.rs file just use `mod filename;`

```
mod print;
```

we could call the function like the below,

```
mod print;

fn main() {
    print::run();
}
```

### Formatting the console output 

#### Placeholder

To print a number you must use '{}' in our `println` statement. `{}` called as placeholder

```
println!("Number: {}", 1);
```

#### Positional arguments

You can use positional arguments if you have multiple arguments

```
// Roger is sad and Roger wants to Sleep
println!("{0} is {1} and {0} wants to {2}", "Roger", "sad", "sleep");
```

In the above example 'Roger' comes twice. we don't want to call them twice instead, we could add the position of the arguments passed (Index of the argument placed)

**Note:** The Index always starts with 0

#### Named Arguments

In several cases you need to name the arguments to clear confusion. During that time we can use named arguments like below,

```
// Named Arguments
println!("{name} is {activity}", name = "John", activity = "playing");
```

### Variables

* Variables are the references to the data 
* In Rust, Varaibles are immutable (cannot be reassigned)
* Rust is a block-scope language

#### Creating a variable

```
println!("******* VARIABLES *******");
    // Assigning a variable and value
    let name = "Justin";
    let age = 52;

    println!("{} is {}", name, age);
```

#### Creating a mutable variable

We can also create mutable variable with keyword `mut`

```
// mutable variable
let mut humidity = 65;
println!("Humidity {}", humidity);
```

#### Creating multiple variables

```
let (my_name, my_age, my_city) = ("Karthik", 29, "Chennai");
println!(NAME: {}, AGE: {}, CITY: {}, myname, my_age, my_city);
```

### DataTypes

In Rust we have several primitive datatypes which we use in generally all the programming languages like integer, float, boolean, char, Tuples, arrays. Datatypes in RUST will usually be inferred by the value we pass in the variable automatically. However we have options to explicitly assign a varaible to specific datatype.
Below are the Datatypes we use in RUST

* Integers - u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (Number of bits they take in memory, u - unsigned)
* Floats - f32, f64
* Boolean - (bool)
* Character - (char)
* Tuples
* Arrays


#### INTEGERS

```
let int_number = 9;
println!("Integer Number: {}", int_number);
```

Integer by default take i32 datatype. However we can also explicitly assign the datatype

```
// Adding the datatype explicitly
let int64_number: i64 = 5454645533;
println!("Int 64 Number: {}", int64_number);
```

#### To find the maximum number in the Datatype

We have a `std` library in which we have `MAX` function to identify the maximum number in datatype

```
// Maximum numbers in datatypes
println!("Max of i32: {}", std::i32::MAX);
println!("Max of i64: {}", std::i64::MAX);
println!("Max of i128: {}", std::i128::MAX);
```

#### Boolean

We can define boolean in two ways
1. By explicitly assigning the boolean datatype to the variable
2. Addigning boolean datatype based on condition

```
let is_active = true;
// or
// let is_active: bool = true;

println!("Active Status: {}", is_active);

// Assigning boolean based on condition
let is_greater = 10 > 5;
println!("Is Greater: {}", is_greater);   
```

#### Character

Char is the character where you can use only one character value in single quotes ('a'). we can also assign the unicode smileys using char like below

```
// Character
let a = 'a';
let smiley = '\u{1F60D}';

println!("{} is {}", a, smiley);
```

When you run you could see the below

```
a is 😍
```



