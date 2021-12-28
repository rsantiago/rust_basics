

pub fn immutable_borrows() {
    println!("----------------------------------------------");
    println!("-----------mutable borrows--------------------");
    println!("----------------------------------------------");

    // general picture:
    // it is possible to borrow an object with a read-write reference.
    // there can be only one mutable-borrow (read-write reference) at a time.
    // mutable borrows doesn't steal the ownership of the object.
    // When the variable that mutably borrowed an object goes out of scope,
    // the object can be immutably-borrowed as much as you want
    // (passing a read-only reference to other variables)
    // or you can do another mutable borrow (read-write reference)

    let a_string = String::from("a_string"); // immutable reference declared

    // the following is a string slice, immutably borrowed from a string
    // (that is, a read-only pointer to a string)
    let _another_string : &str = &a_string;

    // it is impossible to create a mutable borrow from an immutable variable
    // and assign it to a mutable variable,
    // even if the left variable is immutable or mutable, it doesnt matter:
    // let immutable_variable_to_mutable_reference = &mut a_string; // doesnt work
    // let mut mutable_variable_to_mutable_reference = &mut a_string; // doesnt work either

    // it is possible to immutably borrow an immutable object
    // and assign the pointer to a mutable variable, as follows.
    // You are just creating a read-only pointer.
    // This is pretty standard, you are declaring a mutable variable
    // that stores an immutable pointer to an object in the heap.
    let mut _mutable_variable_to_immutable_reference = &a_string;
    // crazy shit...

    // cant read line from an immutable string variable
    // stdin().read_line(&mut a_string);

    // the following lines are also not allowed.
    // stdin().read_line(_mutable_variable_to_immutable_reference); // because the reference is immutable and the function expects a mutable reference
    // stdin().read_line(&mut _mutable_variable_to_immutable_reference); // because you can't make an immutable reference, mutable

    // we will create another string bellow (immutable)
    let another_immutable_string = String::from("another immutable string");

    // the following is impossible, because it is trying to transfer the whole object ownership
    // to a mutable variable that is a storage to an immutable reference (borrowed) variable
    // _mutable_variable_to_immutable_reference = another_immutable_string; // see the lack of '&'

    // this is possible, though, since we are taking an immutable reference
    // and assigning to a mutable variable
    _mutable_variable_to_immutable_reference = &another_immutable_string;
    // very intricate and detailed stuff. Gotta think about it to get the principles.

    let _immutable_variable = &a_string; // immutably borrowing a reference to an immutable variable
    // _immutable_variable = another_immutable_string; // not possible, since the variable stores a borrow to an immutable variable
    // _immutable_variable = &another_immutable_string; // not possible, since it is an immutable variable in the left. Cant change its content.
}