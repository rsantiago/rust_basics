use std::io::stdin;

pub fn reading_lines() {
    println!("----------------------------------------------");
    println!("-----------how to read lines properly---------");
    println!("---- AND dealing with mutable references -----");
    println!("----------------------------------------------");

    let _my_string = String::new(); // we are not using this variable, so it has to be prefixed with _

    // ------------------------------------------------------------------
    // the following is not possible:
    // the programmer is trying to pass the whole string smart pointer ownership to the function.
    // the read_line function expects to just receive a mutable reference to borrow the string temporarily.
    // *borrowed*, because it doesnt want to be the owner of the string.
    // *mutable* reference, because it wants to change the string content.
    // so, a mutable borrow is a read-write reference to an object, without stealing the ownership
    // of the object

    // let result = stdin().read_line(my_string); // impossible, as explained

    // the following is also not possible.
    // the reference is passed correctly (no ownership transferred),
    // but it is immutable (read-only).
    // The function will not be able to change the content.
    // let result = stdin().read_line(&my_string);

    // the following is also not possible,
    // since the function expects a mutable smart pointer, and
    // the variable was declared as immutable
    // let result = stdin().read_line(&mut _my_string);

    // lets create a mutable variable, that owns an empty string here.
    // this declaration makes the string a read-write object:
    let mut another_string = String::new();

    // the following is not possible.
    // 'another_string' is correctly declared as mutable (read-write),
    // but it is being passed as an immutable reference (read-only)
    // to a function that expects a mutable reference (read-write)
    // let result = stdin().read_line(&another_string);

    println!("Now, lets read a string and assign to a string variable");

    // in the following line,
    // the another_string is passed correctly as a mutably-borrowed reference.
    // It means that it is a read-write reference to the string object,
    // and the read_line function is NOT the owner of the string.
    // rust is full of nuances, it is weird at first, but then you will get used to it
    let _result = stdin().read_line(&mut another_string);

    println!("string is: {}", another_string);
}