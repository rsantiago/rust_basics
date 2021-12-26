use std::mem::size_of_val;

struct MyCustomType {
    custom_number:i32,
    custom_other_number:i128
}

enum SeasonsEnumerationType {
    SUMMER,
    AUTUMN,
    WINTER,
    SPRING
}

pub fn basic_data_types() {
    // all int types
    let _unsigned_integer_8bits : u8;
    let _unsigned_integet_16bits : u16;
    let _unsigned_integer_32bits : u32;
    let _unsigned_integer_64bits : u64;
    let _unsigned_integer_128bits: u128;

    let _signed_integer_8bits: i8;
    let _signed_integer_16bits: i16;
    let _signed_integer_32bits: i32;
    let _signed_integer_64bits: i64;
    let _signed_integer_128bits: i128;

    let _default_cpu_unsigned: usize ;
    let default_cpu_signed: isize = 12;

    println!("how many bits does this cpu have? {}bits", size_of_val(&default_cpu_signed)*8);

    // char types
    let my_char : char = 'x';

    println!("how many bytes does a char have? {}bytes", size_of_val(&my_char));

    // floating point

    let _float_32bits: f32;
    let _float_64bits: f64;

    // boolean

    let mut boolean : bool;
    boolean = false;
    boolean = true;

    let my_custom_type_variable_allocated_on_stack = MyCustomType {
        custom_number: 123,
        custom_other_number: 345
    };

    let my_custom_type_variable_boxed_on_heap = Box::new(MyCustomType{
            custom_number: 0,
            custom_other_number: 0
        });

    let my_custom_type_variable_unboxed = *my_custom_type_variable_boxed_on_heap;

    let season = SeasonsEnumerationType::WINTER;

    println!("everything is declared.")
}