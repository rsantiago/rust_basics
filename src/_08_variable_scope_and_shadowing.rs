pub fn scope_and_shadowing() {
    let _a = 1;
    let b = 2;

    let _a = 3; // shadowed the first a
    println!("a = 3 -> {}", _a);
    {
        let b = 6 ; // shadowing the first b
        println!("b = 6 -> {}", b);
    }

    println!("b = 2 -> {} ", b);
}