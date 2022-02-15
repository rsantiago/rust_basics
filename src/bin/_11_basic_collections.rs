use std::borrow::Borrow;
use std::collections::btree_map::Entry::Occupied;
use std::collections::{HashMap, HashSet};
use std::ptr::hash;

fn main() {
    collections();
}
pub fn collections() {
    simple_arrays();
    // multi_dimensional_arrays();
    // slices();
    // tuples();
    //vectors();
    //hashmaps();
    //hashsets();
}

fn hashsets() {
    // see this for hashing performance https://nnethercote.github.io/perf-book/hashing.html
    let mut hashset = HashSet::new();
    hashset.insert("test");
    hashset.insert("another_test");

    let hashset : HashSet<_>= (1..=500).collect();
    println!("Hashset size is {}", hashset.len());
}

fn hashmaps() {
    let mut a_hashmap = HashMap::new();
    a_hashmap.insert("test", 35);
    a_hashmap.insert("another_test", 22);

    for(key, value) in &a_hashmap {
        println!("key {} value {}", key, value)
    }

    println!("value for key 'test' is {}", &a_hashmap["test"]);
    // println!("value for key 'wrong_key' is {} ", &a_hashmap["wrong_key"]); // this will break the program!
    let mut value = a_hashmap.entry("wrong_entry").or_default();
    println!("The default value will be zero: {}", value);

    value  = a_hashmap.entry("another_wrong").or_insert(40);
    println!("The inserted value should be 40 -> {} ", value);

    let mut value_anodah = a_hashmap.entry("anodah").or_default();
    println!("The value should be 40 again -> {}", value_anodah);

    // println!("The value of value is: {}", value); // cant be done, as the last borrow could have messed up with where the 'value' pointer is referring to

    value = a_hashmap.entry("another_wrong").or_default();
    println!("The value should be 40 again -> {}", value);

    value  = a_hashmap.entry("another_wrong").or_insert(21);
    println!("The value should still be 40 -> {}", value);

    match a_hashmap.get("wrong again") {
        Some(v) => println!("There is value: {}", v),
        None => println!("No value")
    }

    let option = a_hashmap.get("wrong yet again");
    match option {
        Some(v) => println!("There is value: {}", v),
        None => println!("No value")
    }

    // println!("The value of value is {}", value); // cant do this here, because the match statements are immutable borrow
    /** this is a simple explanation for the problem
    https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable

    The only way to print value is assigning it to another reference, like this, before trying to print it

    let mut integer = 2;
    value = &mut integer;

    It happens so that value doesnt mess with memory that the first immutable borrow is trying to deal with.
    Even if you will not use it later, it can't point to some area that might be violated by a mutable borrow.
    Wow, that is quite interesting.

    **/

    let entry = a_hashmap.entry("another_wrong").or_default();
    *entry = 10;

    println!("Changed the value of entry to {}", entry);

    // entry = value // cant do this, because entry is immutable
    value = entry ; // could do this, because value is mutable
    println!("The value of value is {}", value);
    println!("The value of entry is {}", entry);


}

fn vectors() {
    let mut my_vector = Vec::new();
    my_vector.push(1);
    my_vector.push(2);
    my_vector.push(3);
    let mut my_other_vec = my_vector.clone();

    println!("two ways to access the values: {:?} - {:?}", my_vector.get(1), my_vector);
    println!("one more way {}", my_vector[2]);

    check_vector_index(&my_vector, 11);
    check_vector_index(&my_vector, 2);
    check_vector_index(&my_vector, 1);

    for x in &my_vector { println!("Also a way to go through the vector elements: {} ", x) }

    while let Some(y) = my_other_vec.pop() { println!("Found value: {}", y);  }

    loop {
        match my_vector.pop() {
            Some(v) => println!("Found a value -> {} ", v),
            None => break
        }
    }

    let vector_another_build = vec![1, 2, 3, 4];
    let vector_another_example = vec!["azul", "vermelho", "preto"];

    for x in vector_another_build {
        println!("value found: {}", x);
    }

    // println!("vector_another_build_contents: {:?} ", vector_another_build); // this cant be done here, because vector ownership is moved to the for loop

    for y in &vector_another_example { // immutable borrow here
        println!("value found: {}", y);
    }
    println!("Valores do vector_another_example {:?}", vector_another_example); // agora pode

    let mut consolidated_vectors = vec![];
    consolidated_vectors.extend(vector_another_example);
    // consolidated_vectors.extend(vector_another_build); // cant do this because the last line establishes another type for the vector
    println!("consolidated vector {:?}", consolidated_vectors)

    // println("vector_another_example : {:?}", vector_another_example); // this is not possible, since extend() method moved the vector completely (transferred the ownership to the extend method)

}

fn check_vector_index(my_vector: &Vec<i32>, index: usize) {
    match my_vector.get(index) {
        Some(3) => println!("Found 3!"),
        Some(v) => println!("Found a value: {}", v),
        None => println!("no value found")
    }
}

fn tuples() {
    let my_simple_tuple = get_a_simple_tuple(3, 5, 7);
    println!("my simple tuple {0}, {1}, {2}, {0}, {1}, {2}", my_simple_tuple.0, my_simple_tuple.1, my_simple_tuple.2);

    let (a_destructured, b_destructured, c_destructured) = my_simple_tuple;

    let another_simple_tuple = get_a_simple_tuple(1,2,5);
    let tuple_of_tuples = (my_simple_tuple, another_simple_tuple);
    println!("Tuple of tuples: {:?}", tuple_of_tuples);
    println!("Accessing elements from combined tuples: {0}, {1}", (tuple_of_tuples.0).0, (tuple_of_tuples.1).0);

    let ((a, b, c),(d, e, f)) = tuple_of_tuples; // destructuring a combined tuple
    let tuples_can_have_many_different_types = ('a', "test", 32, true);
    let single_element_tuple = (32);
    let another_single_element_tuple = (43, );
}

fn get_a_simple_tuple(a: isize, b:isize, c:isize) -> (isize, isize, isize) {
    (a+2, b+4, c+ 9)
}

fn slices() {
    let my_array = ['a', 'b', '%', '+', '1', '8', 'w'];
    let two_elements_immutable_slice = &my_array[0..4];
    let _another_immutable_slice_from_first = two_elements_immutable_slice;
    let all_elements_slice_from_array = &my_array;
    let _four_elements_slice_from_last_slice_no_size_specified: &[char] = &all_elements_slice_from_array[0..5];

    get_a_slice(&my_array);
    get_a_slice(two_elements_immutable_slice);
    get_a_slice(_another_immutable_slice_from_first);
    get_a_slice(all_elements_slice_from_array);
    get_a_slice(&_four_elements_slice_from_last_slice_no_size_specified);

    let mut my_mutable_array = ['1', '2', '3', '4'];
    let mutable_slice : &mut [char] = &mut my_mutable_array[0..4];
    change_a_slice( mutable_slice);
}

fn get_a_slice( slice: &[char]) {
    println!("This is the first element: {} and the length: {}", slice[0], slice.len());
}

fn change_a_slice( slice: &mut [char]) {
    slice[0] = '3';
    println!("The new slice value is: {:?}", slice);
}

fn simple_arrays() {
    let int_array_on_stack: [i32; 3] = [1, 2, 3];
    println!("array size: {} ", int_array_on_stack.len());
    println!("This will print the entire array, in debug mode: {:?}", int_array_on_stack);

    if int_array_on_stack != [1, 2, 4] {
        println!("compared the whole array")
    }

    if int_array_on_stack == [1, 2, 3] {
        println!("But this will be true")
    } else {
        println!("will never reach this part")
    }

    let int_array_same_value = [3; 30];
    println!("values of the same value array: {:?}", int_array_same_value);

    for index in 0..int_array_same_value.len() {
        print!("{} ", int_array_same_value[index]);
    }

    let int8bits_array = [4i8; 10];
    println!("\nThis array of 8 bits has {} bytes in size", std::mem::size_of_val(&int8bits_array));
    let int128bits_array = [4i128; 10];
    println!("\nThis array of 128 bits has {} bytes in size", std::mem::size_of_val(&int128bits_array));

    get_array_as_argument(int_array_same_value);
}

fn get_array_as_argument(my_array: [i32; 30]) {
    println!("{:?}", my_array)
}
fn multi_dimensional_arrays() {
// type; number os elements
    let my_first_matrix: [[i32; 3]; 5];
    my_first_matrix =
        [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
            [10, 11, 12],
            [13, 14, 15]
        ];

    println!("{:?}", my_first_matrix);

    let three_dimensional_array: [[[u8; 5]; 3]; 2];
    three_dimensional_array =
        [
            [
                [1, 2, 3, 4, 5],
                [6, 7, 8, 9, 10],
                [11, 12, 13, 14, 15]
            ],
            [
                [16, 172, 183, 194, 205],
                [216, 227, 238, 249, 250],
                [111, 212, 113, 114, 215]
            ],
        ];

    println!("{:?}", three_dimensional_array);
}
