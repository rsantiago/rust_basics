fn main() {
    moving_ownership();
}

pub fn moving_ownership() {
    println!("----------------------------------------------");
    println!("-----------single ownership principles--------");
    println!("---- & moving ownership between variables-----");
    println!("----------------------------------------------");

    // general idea: each object in the heap has only one owner. Period.
    // the ownership is passed between variables.
    // new owner makes last owner inaccessible, so you can't reference it anymore,
    // or the compiler will byte you in the face :-D
    // the owner holds what is called 'smart pointer', which is the structure
    // that allows to monitor and destroy objects in the heap,
    // as the variable goes out of scope

    // smart pointer, first owner of the string
    let old_owner = "'Whatever string'".to_string();
    // the literal is called a 'string slice', you have to call 'to_string()' to turn into a string.
    // see the data_types file to learn more about it.

    println!("{} {}", old_owner, "old owner"); // just printing the actual owner.

    // smart pointer goes to new owner now, first owner looses ownership and becomes inaccessible
    let new_owner = old_owner;
    println!("{} {}", new_owner, "object ownership moved here to new_owner");
    println!("{} {} ", new_owner, "new owner again");

    //println!("{} {} ", old_owner, "this is not allowed, old owner is not the owner anymore");

    // the following is not possible at this point, as 'old_owner' is declared as an immutable variable.
    // cant assign value twice to an immutable variable
    // old_owner = new_owner;

    // immutable borrows can happen at will, no problem:
    // immutable borrows mean I am using a variable to reference
    // the same object that the owner points to
    // 'immutable' means that I can't changed the value of the object i am pointing to.
    // 'immutable borrows' are read-only references to objects.
    let immutable_borrow_a = &new_owner;
    // & was used to create a read-only reference the same object
    // whose ownership is held by the 'new_owner' variable
    // references are used as the original variable,
    // but they cant have the ownership of the variable

    // general principle: you can immutably borrow as much as you like.
    // It means that you are creating read-only references.

    let immutable_borrow_b = &new_owner; // can borrow twice or n times
    let immutable_borrow_b_new_owner = immutable_borrow_b; // it works
    let _immutable_borrow_b_another_new_owner = immutable_borrow_b_new_owner; // it works
    let _immutable_borrow_b_trying_to_get_b_again = immutable_borrow_b; // it works normally

    let immutable_borrow_c = &immutable_borrow_b; // it is a reference to a reference: &&string
    let immutable_borrow_d = &immutable_borrow_c; // a reference to a reference to a reference: &&&string

    // see bellow that, although immutable_borrow_d is a reference to a reference, to a reference,
    // or &&&string, the compiler knows that you want to reach the object that was borrowed
    // and can present the correct value in the output
    println!("{} {} {} {}",
             immutable_borrow_a, immutable_borrow_b, immutable_borrow_c, immutable_borrow_d);

    println!("{} {} {}",
            immutable_borrow_b_new_owner,
            _immutable_borrow_b_another_new_owner,
            _immutable_borrow_b_trying_to_get_b_again);

    // the following function called declares a variable that points directly to the object,
    // therefore, it will steal the ownership of the 'new owner' variable.
    // Once an owner variable goes out of scope,
    // rust deallocate the object that the variable pointed to.
    steal_ownership(new_owner);
    // Note that de-allocation only happens with objects that live
    // in the heap. Simple variables like ints, booleans, chars etc
    // usually live in the stack and their value are fully copied from one variable
    // to the other. There are no references, only copies and no borrow or ownership checkers for those.

    let mut a = 3;
    let mut b = a;
    b = 4; // a is still '3'
    println!("{} is the value of a", a);

    // print!("{} {} ", new_owner,
       // "not possible, the ownership was taken by the function variable");

    println!("At this time, \
    The string was deallocated because the function variable died");

    // the following code bellow is not possible anymore,
    // since new_owner is not the owner anymore,
    // and the variables that borrows can't find the value of the object in new_owner
    // println!("{} {} {} {}",
    // mutable_borrow_a, mutable_borrow_b, mutable_borrow_c, mutable_borrow_d);

    // the following is also not possible, since the variable had its ownership
    // transferred to the first call to the same function (and then it was deallocated)
    // steal_ownership(new_owner);

    println!("No variable ownerships are taken by the println function, \
             since it is a macro, not a function");

}

fn steal_ownership(brand_new_owner: String) {
    println!("{} {}", brand_new_owner, "a function now has the ownership of the string");
}

//impl TryFrom<>