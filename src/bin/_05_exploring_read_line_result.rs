use std::io::stdin;

fn main() {
    exploring_read_line_result();
}
pub fn exploring_read_line_result() {
    println!("type a floating number");
    let mut my_var = String::new();
    let _result = stdin().read_line(&mut my_var);

    // result.is_ok() // if the line was read
    let float : f32 = my_var.trim().parse().unwrap();
    println!("float: {}" , float);

    let number= float.round();
    println!("integer: {}", number);

}