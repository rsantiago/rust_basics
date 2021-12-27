pub fn collections() {
    simple_arrays();
    multi_dimensional_arrays();
    slices();
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
