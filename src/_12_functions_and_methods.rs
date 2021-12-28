pub fn functions_and_methods() {
    functions();
    methods();
    function_references();
    closures();
}

fn closures() {
    let i_am_inventing_a_function_inside_a_variable = |x:isize| -> isize { x*4 };
    let return_was = i_am_inventing_a_function_inside_a_variable(32);
    println!("Return was: {}", return_was);

    let another_interesting_closure_declaration = |x, y, w| { x + y + w};
    println!("{}", another_interesting_closure_declaration(1,2,3));

    // this wont happen, because the type is declared in compile time
    // and there is already a reference to the closure
    // println!("{}", another_interesting_closure_declaration("a string ".to_string(), "string slice", "another string slice"));

    let another_interesting_closure_that_will_be_of_type_string = |x, y, w| { x + y + w};
    println!("this will work, though {}", another_interesting_closure_that_will_be_of_type_string("a string ".to_string(), "string slice ", "another string slice"));

}

fn function_references() {
    let my_function_reference = methods;
    my_function_reference();
}

enum POSITION {
    Floor,
    Sky
}
struct Airplane {
    position: POSITION,
}

impl Airplane {
    fn perform(&self) {
        match self.position {
            POSITION::Floor => println!("Im parked."),
            POSITION::Sky => println!("Im flying!")
        }
    }
}
fn methods() {
    let airplane_floor = Airplane { position: POSITION::Floor };
    let airplane_flying = Airplane { position: POSITION::Sky };

    airplane_floor.perform();
    airplane_flying.perform();
}

fn functions() {
    a_certain_amount_of_args_that_do_nothing(1, 2, 3);
    // a_certain_amount_of_args_that_do_nothing(z= 1, x=3, y=4); // named args not supported yet

    let my_cast_int = example_of_return(3);
    let mut variable_to_change=2;
    altering_a_value_in_the_argument(&mut variable_to_change);
    println!("the variable was changed to: {}", variable_to_change);
}
fn altering_a_value_in_the_argument(argument_to_change: &mut isize) {
    *argument_to_change=3;
}
fn a_certain_amount_of_args_that_do_nothing(x: i32, y: i32, z : i32 ){
    println!("{} {} {} ", x, y, z);
}

fn example_of_return(x: i32) -> i128 {
    x as i128
}