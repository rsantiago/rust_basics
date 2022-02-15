fn main() {
    const_static_unsafe();
}
const REPLACED_VARIABLE_IN_COMPILE_TIME:isize=100;
static THIS_VARIABLE_HAS_AN_ADDRESS:isize=200;
static mut THIS_VARIABLE_LIVES_AND_IT_IS_UNSAFE_CAUSE_IT_IS_MUTABLE:isize=300;

pub fn const_static_unsafe() {
    println!("this variable will be replaced: {}", REPLACED_VARIABLE_IN_COMPILE_TIME);
    println!("this variable is not replaced, the variable lives throughout the program: {}", THIS_VARIABLE_HAS_AN_ADDRESS);


    println!("This is an unsafe operation, cause the variable may be written in other places. Gotta tell the compiler you know what you are doing.");
    unsafe {
        println!("this variable is not replaced, it lives throughout the program too {} ", THIS_VARIABLE_LIVES_AND_IT_IS_UNSAFE_CAUSE_IT_IS_MUTABLE);
        THIS_VARIABLE_LIVES_AND_IT_IS_UNSAFE_CAUSE_IT_IS_MUTABLE = 500;
    }

}