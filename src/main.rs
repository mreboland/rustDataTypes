fn main() {
    // Rust is statically typed, which means that the compiler must know exactly the data types for each variable in your code.

    // In most cases the compiler can infer the type of some value and we don't need to tell it explicitly in our code. Sometimes when many types are possible, we must inform the compiler what specific data type must be used. In these situations, we use type annotations.

    // I.e converting a string into a number using .parse()

    // Here we are telling the compiler we want the number variable to be a 32-but number by using "u32".
    // The underscore is to remove errors on compile as the variables are unused
    let _number: u32 = "42".parse().expect("Not a number!");

    // If we omit the u32 like below, we'll get a type error
    // let number = "42".parse().expect("Not a number!");

    // NUMBERS

    // An unsigned variable type of int can hold zero and positive numbers, and a signed int holds negative, zero and positive numbers.

    // In 32-bit integers, an unsigned integer has a range of 0 to 232-1 = 0 to 4,294,967,295 or about 4 billion. The signed version goes from -231-1 to 231, which is –2,147,483,648 to 2,147,483,647 or about -2 billion to +2 billion. The range is the same, but it is shifted on the number line.  

    // Length 	Signed 	Unsigned
    // 8-bit 	i8 	    u8
    // 16-bit 	i16 	u16
    // 32-bit 	i32 	u32
    // 64-bit 	i64 	u64
    // 128-bit i128 	u128
    // arch 	isize 	usize

    // The isize and usize types depend on the kind of computer your program is running on: 64 bits if you're on a 64-bit architecture and 32 bits if you're on a 32-bit architecture. They're the default type assigned to integers whenever you don't specify one.

    // Rust's floating-point types are f32 and f64. The default type is f64 because on modern CPUs the speed between the two is roughly the same but f64 is capable of more precision.

    let _x = 2.0; // f64, default type
    let _y: f32 = 3.0; // f32, via type annotation

    // All of Rust's primitive number types support mathematical operations
    
    // Addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // The below creates an overflow error as unsigned numbers can't be negative!
    // println!("1 - 2 = {}", 1u32 - 2);

    // Integer Division
    println!("9 / 2 = {}", 9u32 / 2); // gives us 4

    // Float division
    println!("9 / 2 = {}", 9.0 / 2.0); // gives us 4.5

    // Multiplication
    println!("3 * 6 = {}", 3 * 6);

    // We're using suffixes on the literal numbers to tell Rust which data type they will be (e.g., 1u32 is the number one as an unsigned 32-bit integer). If we don't provide these type annotations Rust tries to infer the type from context defaulting to i32 (a signed 32-bit integer) when it's ambiguous.

    // BOOLEANS

    // Booleans in Rust are represented by the type bool and have two possible values: true or false.

    let is_bigger = 1 > 4;
    println!("{}", is_bigger); // prints "false"

    // CHARACTER AND STRINGS

    // Rust has two string types and one character type. All of them are UTF-8 representations.
    // The 'char' type is the most primitive type among them and is specified with single quotation marks.

    let _c = 'z';
    let _z = 'Z';
    let _heart_eyed_cat = '😻';

    // the 'str' type, also know as a string slice, is a view into string data. We refer to those types in referenced form by using &str. We can think of &str as a pointer to an immutable string data. String literals are all of type &str.

    // String literals aren't suitable for every situation in which we might use text. That's because not every string can be known at compile time. For example when a user interacts with a program and sends text. For these situations, Rust has a second string type, 'String'. This type is allocated on the heap. It can store an amount of text that's unknown to us at compile time.

    // We can think of 'String' data as string data that can change as your program runs, while '&str' are immutable views into string data that do not changes as our program runs.

    // We can create a 'String' from a string literal by using the 'from' function

    let mut hello = String::from("Hello, "); // create a String from a string literal
    hello.push('w'); // push a character into our String
    hello.push_str("orld!"); // push a string literal into our String
    println!("{}", hello);

    // TUPLES

    // A tuple is a grouping of values of different types collected into one compound. They have fixed length, meaning that after they're declared, they can't grow or shrink in size. The type of a tuple is defined by the sequence of each member's type.

    // Ex: A tuple of length 3:
    // ("hello", 5i32, 'c');

    // The above tuple has the type signature (%'static str, i32, char), where:
    // &'static str is the type of the first element
    // i32 is the type of the second element
    // char the third

    // Tuples elements can be accessed by position, which is known as tuple indexing.
    let tuple = ("hello", 5, 'c');

    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');

    // The assert_eq! macro verifies that two expressions are equal to each other.
    // Tuples are useful when we want to combine different types into a single value. For instance, functions can use tuples to return multiple values because tuples can hold any number of values.
}
