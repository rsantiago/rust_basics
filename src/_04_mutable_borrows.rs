use std::io::stdin;

pub fn mutable_borrows() {
    let mut a_string = String::new();

    // ---------------- if we do these immutable borrows, then all the mutable borrows bellow wont compile
    // let immutably_borrowing_a = &a_string; // immutably borrowing A
    // let immutably_borrowing_a_again = &a_string; // immutably borrowing A again
    //
    // println!("{} {} ", immutably_borrowing_a, immutably_borrowing_a_again); // printing the immutable borrows
    // println!("{} {} ", immutably_borrowing_a, immutably_borrowing_a_again); // printing the immutable borrows
    // ---------------- if we do these immutable borrows, then all the mutable borrows bellow wont work

    let mutably_borrowing_a = &mut a_string; // mutably borrowing

    // stdin().read_line(immutably_borrowing_a); // cant cast from immutably borrow to a mutable borrow
    // stdin().read_line(&mut immutably_borrowing_a_again); // same here
    // stdin().read_line(a_string); // cant transfer the ownership to readline, since it doesnt expect to take the ownership

    // can mutably borrow a_string, but it has already been mutably borrowed before,
    // therefore, the write powers were given to another variable (mutably_borrowing_a)
    // stdin().read_line(&mut a_string);

    let _result = stdin().read_line(mutably_borrowing_a); // can get a mutable borrow through this variable

    println!("{}", mutably_borrowing_a); // it is possible to print this variable

    //let mutably_borrowing_a_again = &mut a_string; // cant mutably borrow twice or more

    let mut another_mutable_string = String::new();
    let mut yet_another_mutable_string = String::new();

    // mutably_borrowing_a = another_mutable_string; // cant transfer ownership to a variable that points to a borrow reference instead of a string
    // mutably_borrowing_a = &another_mutable_string; // cant immutably borrow to an immutable variable
    // mutably_borrowing_a = &mut another_mutable_string // cant mutably borrow to an immutable variable that was set before

    let mut mutable_variable_mutably_borrowing : &mut String; // declaring a mutable variable of mutably borrowed string type
    // mutable_variable_mutably_borrowing = &mut a_string; // cant assign a_string as mutable more than once

    // assigning successively new values to a mutable variable that holds a mutable borrow to a string

    mutable_variable_mutably_borrowing = &mut another_mutable_string;
    println!("{}", mutable_variable_mutably_borrowing);

    mutable_variable_mutably_borrowing = &mut yet_another_mutable_string;
    println!("{}", mutable_variable_mutably_borrowing);

    mutable_variable_mutably_borrowing = mutably_borrowing_a;

    //println!("{}", a_string); // cant immutably borrow after mutably borrowing a
    println!("{}", mutable_variable_mutably_borrowing);


}