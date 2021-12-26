pub fn control_flow() {

    let ifs_are_expressions_too = if 1 < 5 {
        println!("right");
        5
    } else if 5 < 1{
        println!("wrong");
        if 1==1 { 3 } else { 4 } // embeded ifs on return statements
    } else {
        println!("whatever");
        21
    };

    let mut my_number = 2;
    while my_number<5 {
        my_number+=1;

        if my_number == 2 { continue }
    }

    loop {
        if my_number != 0 { break }
    }

    for x in 0..10 {
        continue
    }

    for(mypos, x) in (10..21).enumerate() {
        println!("{} : {}", mypos, x)
    }

    let my_integer = 32;
    let my_match = match my_integer {
        30 => "hey",
        40 => "ho",
        41..=50 => "lets",
        _ => "go"
    };


}