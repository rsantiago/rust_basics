use std::mem::size_of_val;

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

    println!("how many bits does this cpu have? {}bits", size_of_val(&default_cpu_signed)*8)
}