use std::mem::size_of_val;

pub fn basic_data_types() {
    integers();
    chars();
    floats();
    booleans();
    structs();
    enums();
    unions();
    options();

    println!("everything is declared.")
}

fn booleans() {
// boolean

    let mut boolean: bool;
    boolean = false;
    boolean = true;
}

fn floats() {
// floating point

    let _float_32bits: f32;
    let _float_64bits: f64;
}

fn chars() {
// char types
    let my_char: char = 'x';

    println!("how many bytes does a char have? {}bytes", size_of_val(&my_char));
}

fn integers() {
    let _unsigned_integer_8bits: u8;
    let _unsigned_integet_16bits: u16;
    let _unsigned_integer_32bits: u32;
    let _unsigned_integer_64bits: u64;
    let _unsigned_integer_128bits: u128;

    let _signed_integer_8bits: i8;
    let _signed_integer_16bits: i16;
    let _signed_integer_32bits: i32;
    let _signed_integer_64bits: i64;
    let _signed_integer_128bits: i128;

    let _default_cpu_unsigned: usize;
    let default_cpu_signed: isize = 12;

    println!("how many bits does this cpu have? {}bits", size_of_val(&default_cpu_signed) * 8);
}

fn structs() {
    struct MyCustomType {
        custom_number:i32,
        custom_other_number:i128
    }

    let my_custom_type_variable_allocated_on_stack = MyCustomType {
        custom_number: 123,
        custom_other_number: 345
    };

    let my_custom_type_variable_boxed_on_heap = Box::new(MyCustomType{
        custom_number: 0,
        custom_other_number: 0
    });

    let my_custom_type_variable_unboxed = *my_custom_type_variable_boxed_on_heap;
}

fn enums() {
    enum SeasonsEnumerationType {
        SUMMER,
        AUTUMN,
        WINTER,
        SPRING,
        SCREWED(char),
        WEIRD {code: i32, subcode: i64}
    }

    let season = SeasonsEnumerationType::WINTER;
    let screwed_season = SeasonsEnumerationType::SCREWED('a');
    let weird_season = SeasonsEnumerationType::WEIRD { code: 4, subcode: 3 } ;

    let matchResult = match weird_season {
        SeasonsEnumerationType::WEIRD {code: 4, ..} => "I don't care for the subcode",
        _ => "whatever"
    };
}

fn unions() {
    //////////////// declaring unions
    union IntOrFloat {
        i : i32,
        f : f32
    }

    fn test_int_or_float(iorf: IntOrFloat) {
        unsafe {
            match iorf {
                IntOrFloat { f: 2.5 } => { println!("Float: {}", iorf.f) }
                IntOrFloat { i:4 } => { println!("Integer mutcho loko {} ", iorf.i) }
                IntOrFloat{ f} => { println!("Float: {}", iorf.f)}
            }
        }
    }

    let mut int_or_float = IntOrFloat{ f: 2.5 };
    test_int_or_float(int_or_float);

    int_or_float = IntOrFloat{f: 2.4};
    test_int_or_float(int_or_float);

    int_or_float = IntOrFloat{i: 2} ;
    test_int_or_float(int_or_float);
}


fn options() {
    ///////////////////////// OPTIONS
    fn test_my_option(opt: Option<i32>) {
        match opt {
            Some(3) => println!("Peguei o tres: {} ", opt.unwrap()),
            Some(c) => println!("Peguei qualquer outro: {} ", c),
            None => println!("Pegou none"),
        }
    }

    let mut my_option: Option<i32> = Some(3);
    let my_none_integer: Option<i32> = None;
    // let my_none_generic: Option<T> = None; // this is an invalid declaration, you gotta specify which type is T
    test_my_option(my_option);
    test_my_option(my_none_integer);
    my_option = Some(4);
    test_my_option(my_option);

    if let Some(any_name) = my_option { println!("my option is {}", any_name); }
    if let None = my_none_integer{ println!("there is no value associated")}

    let mut i=0;
    while let Some(z) = my_option {
        i+=1;
        println!("Vai passar mais uma vez no while");
        if i==5 { my_option = None ; println!("Nunca mais vai entrar no while let") };
    }
}
