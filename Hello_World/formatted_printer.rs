fn main(){
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.

    let number_of_days = 31;
    println!("{} days", number_of_days);

    // Positional arguments can be used. Specifying an integer in {} determines which argument will be used
    // Arguments start at 0 immediately after the format string.
    let name_1 = "Alice";
    let name_2 = "Bob";
    println!("{0}, this is {1}. {1}, meet {0}", name_1, name_2);

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    println!("Base 10:                  {}",  69420); //69420
    println!("Base 2 (binary):         {:b}", 69420); //10000111100101100
    println!("Base 8 (octal):          {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal):   {:x}", 69420); // 10f2c

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);//    1

    //pad numbers with extra zeros
    println!("{number:0>5}", number= 8); //00008
    println!("{number:0<5}", number= 8); //80000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:>width$}", number=4, width = 5); //    4
    println!("{number:0>width$}", number=4, width=5); //00004
    println!("{number:0<width$}", number=4, width=5); //40000


    // Rust even checks to make sure the correct number of arguments are used.
    //This is wrong, let me correct it://println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}.","Bond", "James" );

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    //println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line
    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}