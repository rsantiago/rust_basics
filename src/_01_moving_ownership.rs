
pub fn moving_ownership() {
    println!("----------------------------------------------");
    println!("-----------single ownership principles--------");
    println!("---- & moving ownership between variables-----");
    println!("----------------------------------------------");

    // smart pointer, first owner of the string
    let old_owner = "'Whatever string'".to_string();

    println!("{} {}", old_owner, "old owner");

    // smart pointer, new owner, first owner dies
    let new_owner = old_owner;
    println!("{} {}", new_owner, "object ownership moved here to new_owner");
    println!("{} {} ", new_owner, "new owner again");

    //println!("{} {} ", old_owner, "this is not allowed, old owner is not the owner anymore");

    // this is not possible at this point, old_owner is declared as immutable
    // cant assign value twice to an immutable variable
    // old_owner = new_owner;

    // immutable borrows can happen at will, no problem:
    let immutable_borrow_a = &new_owner;  // & is just a reference to new_owner immutable variable
    // references are used as the original variable, but they cant have the ownership of the variable

    let immutable_borrow_b = &new_owner; // can borrow twice or n times
    let immutable_borrow_b_new_owner = immutable_borrow_b; // it works
    let _immutable_borrow_b_another_new_owner = immutable_borrow_b_new_owner; // it works
    let _immutable_borrow_b_trying_to_get_b_again = immutable_borrow_b; // it works normally

    let immutable_borrow_c = &immutable_borrow_b; // it is a reference to a reference
    let immutable_borrow_d = &immutable_borrow_c; // a reference to a reference to a reference


    println!("{} {} {} {}",
             immutable_borrow_a, immutable_borrow_b, immutable_borrow_c, immutable_borrow_d);

    println!("{} {} {}",
            immutable_borrow_b_new_owner,
            _immutable_borrow_b_another_new_owner,
            _immutable_borrow_b_trying_to_get_b_again);

    steal_ownership(new_owner);

    // print!("{} {} ", new_owner,
       // "not possible, the ownership was taken by the function variable");

    println!("The string was deallocated because the function variable died");

    // this is not possible anymore, since new_owner is not the owner anymore,
    // and the variables that borrows can't find the value of the object in new_owner
    // println!("{} {} {} {}",
    // mutable_borrow_a, mutable_borrow_b, mutable_borrow_c, mutable_borrow_d);

    // this is also not possible, since the variable had its ownership
    // transferred to the first call to the same function (and then it was deallocated)
    // steal_ownership(new_owner);

    println!("No variable ownerships are taken by the println function, \
             since it is a macro, not a function");

}

fn steal_ownership(brand_new_owner: String) {
    println!("{} {}", brand_new_owner, "a function now has the ownership of the string");
}

//impl TryFrom<>