

pub fn immutable_borrows() {
    println!("----------------------------------------------");
    println!("-----------mutable borrows--------------------");
    println!("----------------------------------------------");

    let a_string = String::new(); // immutable reference declared

    let _another_string : &str = &a_string; // this is a string slice from an immutable borrowed from a_string
    // it is impossible to create a mutable reference from an immutable variable
    // and assign to a mutable variable,
    // even if the left variable is immutable or mutable, it doesnt matter

    // let immutable_variable_to_mutable_reference = &mut a_string;
    // let mut mutable_variable_to_mutable_reference = &mut a_string;

    // it is possible to store (borrow) an immutable reference to an immutable variable
    // inside a mutable variable
    // since the left variable is declared as mutable storage to an immutable reference (&String)
    let mut _mutable_variable_to_immutable_reference = &a_string;
    // crazy shit...

    // cant read line from immutable string variable
    // stdin().read_line(&mut a_string);

    // mutable_variable_to_immutable_reference
    // is mutable, but it borrows an immutable reference
    // that points to an immutable variable
    // reading a line is impossible in any of the ways bellow:

    // stdin().read_line(_mutable_variable_to_immutable_reference); // because the reference is immutable and the function expects a mutable reference
    // stdin().read_line(&mut _mutable_variable_to_immutable_reference); // because you can't make an immutable reference, mutable

    // the mutable variable, though, can reference another immutable string, for example
    let another_immutable_string = String::new();

    // the following is impossible, because it is trying to transfer the whole object ownership
    // to a mutable variable that is a storage to an immutable reference (borrowed) variable
    // _mutable_variable_to_immutable_reference = another_immutable_string;

    // this is possible, though, since we are taking an immutable reference and assigning to a mutable variable
    _mutable_variable_to_immutable_reference = &another_immutable_string;
    // very intricate stuff.

    let _immutable_variable = &a_string; // borrowing a reference to an immutable variable
    // _immutable_variable = another_immutable_string; // not possible, since the variable stores a borrow to an immutable variable
    // _immutable_variable = &another_immutable_string; // not possible, since it is an immutable variable
}