use std::io::stdin;

pub fn reading_lines() {
    println!("----------------------------------------------");
    println!("-----------how to read lines properly---------");
    println!("---- AND dealing with mutable references -----");
    println!("----------------------------------------------");

    let _my_string = String::new(); // we are not using this variable, so it has to be prefixed with _

    // ------------------------------------------------------------------
    // the following is not possible,
    // the programmer is trying to pass the whole string smart pointer ownership to the function
    // the read_line function expects to receive a mutable reference to borrowed the string temporarily
    // *borrowed*, because it doesnt want to be the owner of the string
    // *mutable* reference, because it wants to change the string content

    // let result = stdin().read_line(my_string);

    // also not possible, since the reference is borrowed correctly (no ownership transferred),
    // but the reference is immutable, so the function will not be able to change the content
    // let result = stdin().read_line(&my_string);

    // also not possible, since the function expects a mutable smart pointer, and
    // the variable was declared as immutable
    // let result = stdin().read_line(&mut my_string);

    // lets create a mutable reference here
    let mut another_string = String::new();

    another_string.as_str();
    // not possible, since another string is mutable,
    // but it is being passed as an immutable reference
    // to a function that expects a mutable reference
    //let result = stdin().read_line(&another_string);

    println!("Now, lets read a string and assign to a string variable");
    // the reference is a mutable borrowed reference, no ownership is transferred
    let _result = stdin().read_line(&mut another_string);

    println!("string is: {}", another_string);
}