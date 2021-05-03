fn main() {
    // Rust is statically typed, which means that the compiler must know exactly the data types for each variable in your code.

    // In most cases the compiler can infer the type of some value and we don't need to tell it explicitly in our code. Sometimes when many types are possible, we must inform the compiler what specific data type must be used. In these situations, we use type annotations.

    // I.e converting a string into a number using .parse()

    // Here we are telling the compiler we want the number variable to be a 32-but number by using "u32".
    let number: u32 = "42".parse().expect("Not a number!");

    // If we omit the u32 like below, we'll get a type error
    // let number = "42".parse().expect("Not a number!");

}
