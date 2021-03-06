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
a is ????
```

### String and String methods

Primitive string is immutable, fixed-length somewhere in memory.Another type of Growable string can also be used.

```
// Normal String
let hello = "Hello";
```

```
// Growable & Mutable String
let mut hello = String::from("Hello ");
```

#### String Methods

#### To get length of the string

```
// Get length of String
println!("Length - {}", hello.len());
```

#### Pushing data to string

We can push data with two methods. `push()` & `push_str()`. `push` method is char. It pushes only one character at a time. however if you want to push more than one character we can use `push_str()` method.

```
// Pushing a char to string (we can push only one char at a time)
hello.push('W');
println!("{}", hello);

// Pushing multiple string
hello.push_str("orld!");
println!("{}", hello);
```

#### Capacity string

We can add a capacity to the string variable we create. This can be done using `with_capacity()` method like below

```
// Capacity in bytes
let mut cap_string = String::with_capacity(15);
cap_string.push_str("Hello Rust!");
```

If you try to push more than the capacity assigned you will see an error.

To check the capacity you can use `capacity()` method

```
// check if empty
println!("Empty: {}", cap_string.is_empty());
```

#### To find the data that Contains a string

```
// Check if contains
println!("Contains 'Rust' - {}", cap_string.contains("Rust"));
```

#### To Replace the string with another string

```
// Replace 
println!("Replace 'Rust' with 'Rustlang', {}", cap_string.replace("Rust", "Rustlang"));
```

#### Looping through the string with whitespace

We can loop through the string with spliting whitespace. It litrelly means you can loop every word in a string variable.

```
// Loop through string with Whitespace
for word in cap_string.split_whitespace(){
    println!("{}", word)

}
```

#### Assertion Testing

Assertion testing is just testing the Left == Right. Its similar to comparision. Where it checks if LEFT == RIGHT values. If it passes The program execute without any error. If the condition fails the program will throw an error

```
// Assertion Testing
assert_eq!(11, cap_string.len());
```
The error looks something like the below,

```
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `14`,
 right: `11`', src/strings.rs:46:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Tuples

Tuples are group together values of different data types, Max 12 elements can be in a tuple.
We can declare tuple like the below example

```
let person: (&str, &str, i8) = ("John", "Cricket", 59);
```

Accessing tuple can be like below example

```
println!("{name} like {sport} and {name} scored {score}", name = person.0, sport = person.1, score = person.2);
```

### Arrays

Arrays are fixed length of elements with same datatype grouped together

#### To Create an array

```
let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
```

- let - keyword for assigning
- mut - Mutable so that we can change the value in future
- numbers - array name
- i32 - Integer 32 bit
- 5 - The number of values to be present in array

The length of the array should be equal to the value given while declaring. Also we cannot use mixed datatype in an array. 

for example the below cannot be done with rust arrays,

```
// Will throw error
let mut numbers: [i32; 5] = [1, 2, 3, 4, 5, 6];
(or)
let mut numbers: [i32; 5] = [1, 2, 3, 4, "five"];
```
#### Modifying a value in array

To modify the value in array you can use `array_name[index] = value`. 

```
numbers[0] = 100;
```
This changes the first element of the array to `100`. Note that the array should be declared as mutable data type.

#### Printing array 

```
// To print whole array
println!("{:?}", numbers);

// To print only the selected value
println!("{}", numbers[0]);
```

#### Get length of an array

```
println!("Length of the array - {}", numbers.len());
```

#### Slicing an array

Slicing is a method to split the array from a desired index to a desired index.

Example: if, a = [1, 2, 3, 4, 5] if we desire to slice first two values a[0..2]

```
// Slicing the array in RUST
let slice: &[i32] = &numbers[0..3];
println!("Sliced array - {:?}", slice);
```

In the above example we are creating another array `slice` with datatype `i32` and slicing the first three elements.

#### Memory utilization of array

We can verify the memory utilized for our array. for this we can make use of `mem` function from `std` library.

```
println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
```

We can also import the std library to avoid calling them with whole path.

```
use std::mem
```

If we included the the library like above, we can easily call the functions/ methods in the library.

```
println!("Array occupies {} bytes", mem::size_of_val(&numbers));
```

### Vectors

Vectors are resizable arrays. Most of the array methods would be applicable to vectors as well (printing vector, modifying single value, getting vector length, finding memory utilization, slicing). However unlike arrays, we could add new element and pop the last added element in vectors

#### To create a vector

```
let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
```

#### Adding or Pushing new value to the vector

```
numbers.push(120);
```

#### Pop the last element in vector

```
numbers.pop();
```

#### Looping through Vectors

```
for number in numbers.iter(){
    println!("Number: {}", number);
}
```
#### Mutate while looping vector

We can change the values of vector while looping. This would be very useful to change the values in a single shot. 

For example if we want to multiply the whole vector by 2. we can use the for loop and `iter_mut()` method like below

```
println!("Original vector {:?}", numbers);

for n in numbers.iter_mut(){
    *n *= 2;
}
println!("Vector multiplied by 2 {:?}", numbers);

```

**Output**

```
Original vector [100, 2, 3, 4, 5, 11]
Vector multiplied by 2 [200, 4, 6, 8, 10, 22]
```

### Conditionals and Comparision

In program, we might need to check specific conditions and act as per the condition status. This can be through `if elseif else` statements in RUST. Comparision is nothing but compare couple of values and return the boolean values (`true` or `false`). Below are some comparision operators.

```
&& - AND
|| - OR
> - Greater Than
< - Less than
<= - Less than or equals to
>= - Greater than or equal to
== - Equals to
!= - Not Equal to
```

We can frame `if elseif else` condition like the below using these comparision operators.

```
pub fn run(){
    
    // creating variables
    let age: u8 = 21;
    let has_id: bool = false;

    // Checking if age is greater than or equal to 21 and has ID proof
    if age >= 21 && has_id{
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && has_id {
        println!("Bartender: Sorry, You have no access");
    } else {
        println!("Bartender: I'll have to see your ID.");
    }

}
```

We can also simplify the usage by creating another variable and capture the boolean value. This could be a good shorthand use for if/elseif/else

```
//Shorthand If/Else
let is_adult = if age >= 21 { true } else { false };
println!("Is He an Adult? {}", is_adult);
```

Here, we are creating a new variable `is_adult`, compare the age using `if{}else{}` and assigning value based on the condition `true` or `false`.

### Loops

Loops are used to iterate until a specific condition is met. We have three types of loop in Rust.

#### 1. Infinite loop

```
pub fn run() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Count - {}", count);

        if count == 20 {
            break;
        }
    }
}
```

In this run funtion, we creating a mutable variable `count`. Then we use Rust's `loop` and increment the `count += 1`. Then we are printing the count number. If we don't have a breakpoint this loop would run forever. So, we can create a `if` condtion to `break` the loop.

#### 2. While loop

While loop is similar to infinite loop. we need to pass the specific contition the loop runs until the condition false.

```
let mut count = 0;

while count <= 100 {
    println!("Count - {}", count);
    count += 1;
    }
```

As per the above code the while loop runs util the count is less than and equal to 100. Once it reached 101, It breaks and come out of the loop by itself.

Let's have a practical `fizzbuzz` example. FizzBuzz is a popular interview question for programming. Basically, we need to iterate over a range of number and print `fizz` if they are divisible by 2, print `buzz` if they are divisible by 5, print `fizzbuzz` if the number is divisible by 7. Lets see how we can use `fizzbuzz` in while loop

```
let mut count = 1;

while count <= 100 {
    
    if count % 7 == 0{
        println!("FizzBuzz");
    } else if count % 5 == 0 {
        println!("Buzz");
    } else if count % 2 == 0 {
        println!("Fizz");
    } else {
        println!("{}", count);
    }

    count += 1;
}
```
**Note:** If you forget to increment the loop will not run.

#### 3. For Loop

For loop runs to iterate a range of numbers, string, Vectors, array, tuples. Advantage of for loop is, we don't need to create a value and increment. Let's see a minimal example of for loop range below,

```
for number in 0..100{
    println!("Number - {}", number);
}
```
This snippet of for loop iterates from 0 to 100.

Let's see the same `fizzbuzz` example with for loop this time.

```
for number in 1..100{
    if number % 7 == 0{
        println!("FizzBuzz");
    } else if number % 5 == 0 {
        println!("Buzz");
    } else if number % 2 == 0 {
        println!("Fizz");
    } else {
        println!("{}", number);
    }
}
```

### Functions

Functions are the block of code with instructions, Functions can be reused, as per DRY(Don't Repeat Yourself) terminology.

#### Simple function


```
pub fn run(){
    sample();
}

fn sample(){
    println!("This is a simple function, without any arguments");
}
```

The above function `sample` can be called in `main` function and it can be reused 'n' number of times.


#### Function with arguments
We can pass arguments to the function and mention the values for the arguments while calling. 

**Example**

```
pub fn run(){
    println!("********* FUNCTIONS **********");
    greet("Hello", "Sundar");
}

fn greet(greeting: &str, name: &str){
    println!("{} {}, Nice to see you!", greeting, name);
}
```

#### Function with return value
In general, a function will have return value. Most probably this return value will be assigned to a variable and use it. We could return the value by using `->` pointer symbol.

```
fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
```
Note that there isn't any `;` (semicolon) at the end of expression. This is to inform rust language to consider the expression to be returned. We can call this function by storing the return value in a variable

```
pub fn run(){
    let sum = add(5,9);
    println!("SUM : {}", sum);
}
```

#### Closure function

We have a new type of function called closure function. This is an anonymous function which captures values from the scope it is defined. This can be used as simplified way to write a function.

**Syntax**
```
let <variable_name> = | argument1: <type>, argument2: <type> | <expression>
```

**Usage**
```
let prod_numbers = | number1: i32, number2: i32 | number1 * number2;
println!("Product is {}", prod_numbers(3, 4));
```

### Reference pointers

Reference pointers are the reference to the point a resource in memory. This pointer varies from primitive array and non primitive arrays (vectors)

#### 1. Primitive array
Referencing a variable to another variable is simple in primitive array.

```
let array1 = [1, 2, 3, 4, 5];
let array2 = array1;

println!("Arrays - {:?}", (array1, array2));
```

In this example `array2` is assigned with value of `array1`. and both the arrays are printed as tuples.

**Output**

```
Arrays - ([1, 2, 3, 4, 5], [1, 2, 3, 4, 5])
```
The output looks expected. However if you try the same with non-primitive array. this will fail.

#### 2. Non-Primitive array (vectors)

Let's try with non primitive arrays (vectors)

```
let vector1 = vec![1,2,3,4,5];
let vector2 = vector1;
println!("Vectors - {:?}", (vector1, vector2));
```

If we try the same with non primitive arrays (vectors). we will get the below error. It means the value which is stored in `vector1` will be moved to `vector2`. Now `vector1` will not have any value so it throw error.

**error**
```
   Compiling rust_practice v0.1.0 (/media/sundar/Data/Programming/Practices/RUST/rust_practice)
error[E0382]: use of moved value: `vector1`
  --> src/pointer_ref.rs:18:33
   |
15 |     let vector1 = vec![1,2,3,4,5];
   |         ------- move occurs because `vector1` has type `Vec<i32>`, which does not implement the `Copy` trait
16 |     let vector2 = vector1;
   |                   ------- value moved here
17 | 
18 |     println!("Vectors - {:?}", (vector1, vector2));
   |                                 ^^^^^^^ value used here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `rust_practice` due to previous error
```

**Solution**
To solve this issue we must reference pointer with (&) to use both the value with both the vectors

```
let vector1 = vec![1,2,3,4,5];
let vector2 = &vector1;

println!("Vectors - {:?}", (&vector1, vector2));
```

**output**
```
Vectors - ([1, 2, 3, 4, 5], [1, 2, 3, 4, 5])
```

### Struct

Structs are used to create a custom data type. we can use them in tradional way, Tuple struct or implement them as "class" with methods(functions). Class here referred is just for understanding.

#### 1. Traditional way

Let's see the traditional way first. We can try creating an RGB struct.

```
struct Rgb{
    red: u8,
    green: u8,
    blue: u8,
}
```

We use this struct (datatype) to our variable, assign values to it and we can call this variable in our function like below

```
let mut c = Rgb {
    red: 200,
    green: 120,
    blue: 15 
};

println!("RED - {} BLUE - {} GREEN - {}", c.red, c.green, c.blue);
```
we can modify the value like this -> `c.green = 150`

#### 2. Creating Tuple struct

Let's try the same with tuple now.

```
struct NewColor(u8, u8, u8);

let mut c = NewColor(255,0,120);
println!("Tuple RGB - {:?}", (c.0, c.1, c.2));
```

To modify the tuple value -> `c.0 = 220;`

**Output**

```
RED - 200 BLUE - 120 GREEN - 26
Tuple RGB - (255, 0, 120)
Tuple RGB - (220, 0, 120)
```

#### 3. Creating struct and Implement functions

In this example we would create a struct `Person` and we will have `first_name` and `last_name`. We could create a construction function `new` to construt the `Person`, Also we can create other methods like `get_fullname` & `change_lastname`, 

```
// creating struct
struct Person{
    first_name: String,
    last_name: String
}
```

To implement this struct to a function we need to use `impl` keyword like below and we must create a constructor. 

```
impl Person{
    // constructor
    fn new(first:&str, last:&str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

}
```

In the above consturctor we just created implemented `Person` Struct and we created a method/function `new` with `first` and `last` parameter and return it to `Person` struct. Then we are creating `first_name` and `lastname` and assign the parameter. we use `to_string()` method to get them and save them as string.

Now to instantiate this `Person` struct we need to create a variable and call the `new()` method/function with parameters like the below,

```
let mut p = Person::new("Sundara", "Rajan");
println!("FIRSTNAME - {}\nLASTNAME - {}", p.first_name, p.last_name);
```

This looks good, lets create other methods to show fullname and change the last name.

```
fn get_fullname(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
}
```

`get_fullname` function returns the `String`. we pass `(&self)` as parameter. This is just like `this` or `self` keyword in other programming languages. we also use `format!()` function, which is similar to `println!()` function.

```
println!("{}", p.get_fullname());
```

This is how we can call the method.

Lets create another function to change the lastname.

```
//function to change last name
fn change_lastname(&mut self, last: &str){
    self.last_name = last.to_string();
}
```

We create `change_lastname` function and pass `&mut self` since we will change the `last_name`. then pass `last: &str` as second parameter(last is the parameter used to instantiate lastname in `new()` function). Using this method is fairly simple.

```
p.change_lastname("Raman");
```

We can try convert this firstname and lastname as a tuple.

```
// Convert as tuple 
fn as_tuple(self) -> (String, String){
    (self.first_name, self.last_name)
}
```

Calling the tuple is simple as well.

```
println!("As Tuple: {:?}", p.as_tuple());
```

**Note:** These methods should be within the same `impl` block. 

*complete Code*

```
// creating struct
struct Person{
    first_name: String,
    last_name: String
}

// Implementing in function
impl Person {
    // Constructor function
    fn new(first:&str, last:&str) -> Person{
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    //function to get fullname
    fn get_fullname(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
        
    }
    
    //function to change last name
    fn change_lastname(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    // Convert as tuple 
    fn as_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}
```

**Output**

```
FIRSTNAME - Sundara
LASTNAME - Rajan
Sundara Rajan
Sundara Raman
As Tuple: ("Sundara", "Raman")
```

### Enums

Enums are the types which have definitive values. We can see an example of ball movement in a game. Lets create a `enum` for movement and assign the moving state. 

```
enum Movement{
    // variants - The required possible values
    Up,
    Down,
    Left,
    Right, 
}
```
Now we need a function to move the ball. So we are creating `move_ball` function with argument `m` of type `Movement` (Movement considered to be a type since we created with enum). Then we are using a `match` keyword. `match` is like switch case in other languages. It performs action depends on the info

```
fn move_ball(m: Movement){
    // Perform action depending on the Info (movement)
    match m{
        Movement::Up => println!("Ball Moved Up"),
        Movement::Down => println!("Ball Moved Down"),
        Movement::Left => println!("Ball Moved Left"),
        Movement::Right => println!("Ball Moved Right"),

    }
}
```

Lets create movement actions in our main run function and call `move_ball()` function.

```
pub fn run(){

    let action1 = Movement::Up;
    let action2 = Movement::Down;
    let action3 = Movement::Left;
    let action4 = Movement::Right;
    
    move_ball(action1);
    move_ball(action2);
    move_ball(action3);
    move_ball(action4);
}
```

**Output**

```
Ball Moved Up
Ball Moved Down
Ball Moved Left
Ball Moved Right
```