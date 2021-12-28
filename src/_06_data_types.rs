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
    generics();
    string_slices();
    strings();
    println!("everything is declared.")
}

fn strings() {
    let mut strings_live_in_the_heap = String::from("my string live in the heap");
    strings_live_in_the_heap.push('a');
    strings_live_in_the_heap.push_str(" - and i can push a static string slice to this string");

    let mut another_string = String::from("create a string from a static slice");

    let concatenated = another_string + " i want to add this string slice to the string. The first string ownership is moved to this statement and cant be used anymore";

    let two_strings_concatenated = strings_live_in_the_heap + &concatenated
        + " and both strings ownership are moved. Cant be used anymore";

    println!("{:?}",two_strings_concatenated);

    let string_from_to_string_in_a_slice = "i can call to_string and convert a string slice into a string".to_string();

    // let lets_format_some_strings = "lets {} some {}"; // this can't be used as a template for formatting
    let format = "format";
    let strings = "strings";
    let formatted_string = format!("lets {} some {}", format, strings);
    println!("{}",formatted_string);

    let (repeat, dont) = ("repeat", "dont");
    let formatting_with_indexes = format!("{0} {1} {0}", repeat, dont);
    println!("{}", formatting_with_indexes);

    let (with, parameters) = ("whatever", "u lalala");
    let formatting_with_names = format!("lets format {with} named {parameters}",
        with=with, parameters=parameters
    );
    println!("{}", formatting_with_names);

    let mixed = format! {"{}, {}, {whatsoever}, {2}, {}",
                          format, strings, formatted_string, whatsoever = repeat};

    println!("{}", mixed);
}

fn string_slices() {

    // these are string slices:
    
    let static_string: &'static str = "this string will exist throughout the program lifetime";
    let this_is_also_a_static_string = "this is also a static string";
    {
        let this_reference_to_a_static_string_is_not_static = "The string will live in the binary, but the reference will die";
    }
    println!("living references: \n{}\n{}", static_string, this_is_also_a_static_string);
    // println!("This static string is not accessible anymore: {}", this_reference_to_a_static_string_is_not_static);

    // let single_char = static_string[0]; // this is not allowed
    let single_char = static_string.chars().nth(0);
    if let Some(c) = single_char { println!("but this is allowed: {}", c); }

    let string_slice_from_a_string: &str = &String::from("It is hard to build a string without a string slice");

}

fn generics() {
    struct Coordinates<T> {
        x: T,
        y: T
    }

    let int_coordinate: Coordinates<u16> = Coordinates{ x:3, y:4};
    let string_slice_coord = Coordinates{x:"mycoord", y: "anothercoord"};
    let i16_coord = Coordinates{x: 3i16, y: 6i16};
    let fisize_coord = Coordinates{x: 1f32, y: 2f32};
}

fn booleans() {
    let mut boolean: bool;
    boolean = false;
    boolean = true;
}

fn floats() {
    let _float_32bits: f32;
    let _float_64bits: f64;
}

fn chars() {
    let my_char: char = 'x';
    println!("how many bytes does a char have? {}bytes", size_of_val(&my_char));
}

fn integers() {
    static STATIC_NUMBER: i32 = 3; // this will live forever while the program is being executed

    let _unsigned_integer_8bits: u8; // 8 bits unsigned int
    let _unsigned_integer_16bits: u16;
    let _unsigned_integer_32bits: u32;
    let _unsigned_integer_64bits: u64;
    let _unsigned_integer_128bits: u128;

    let _signed_integer_8bits: i8;
    let _signed_integer_16bits: i16;
    let _signed_integer_32bits: i32; // 32bits signed int
    let _signed_integer_64bits: i64;
    let _signed_integer_128bits: i128;

    let _default_cpu_unsigned: usize; // unsigned int of the processor bits size
    let default_cpu_signed: isize = 12; // signed int of the processor bits size
    let another_way_to_declare_variables = 19 as i128; // simple casting

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

    enum MyCustomEnum {
        ThisWillBeA400 =400,
        ThisWillBeA500 =500
    }

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
    // unions are types that can hold any of the types declared
    // they occupy the same space that the biggest type holds in memory
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
