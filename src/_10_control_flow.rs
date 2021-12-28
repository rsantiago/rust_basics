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

    for x in 0.. {
        if x == 10 { break }
    }

    for(mypos, x) in (10..21).enumerate() {
        println!("{} : {}", mypos, x)
    }

    let last_int: Vec<i32> =
        (0..).take_while(|x| *x<10)
            .collect();
    println!("this is a neat way of doing for too! See the closures samples in this code base. Result is {:?}", last_int );

    let my_integer = 32;
    let my_match = match my_integer {
        30 => "hey",
        named_match @ 20 => "that was a named match",
        _ if my_integer %2 ==0 => "that was a conditioned match",
        40 => "ho",
        41..=50 => "lets",
        _ => "go"
    };

    let a_tuple = (2,3);
    let match_result = match a_tuple {
        (0,0) => "in case the tuple is 0,0",
        (0, y) => "in case the first element is 0, and the other is ",
        (x, y) => "exausting the options by creating variables"
    };

}