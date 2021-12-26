pub fn basic_operators() {
    let mut a = 1+2*8-3/2 ; // all basic operations supported

    // means a = a (operator) number
    a += 2;
    a -= 3;
    a *= 3;
    a /= 3;
    a %= 2;

    // power operations
    let mut b_cubed: i8 = i8::pow(2,3);
    i8::pow(b_cubed, 3);

    let c_whatever: f64 = f64::powi(2.4, 3); // to elevate to an integer
    f64::powi(c_whatever, 3); // elevating to an integer

    let d_whatever: f64 = f64::powf(2.5, 2.3); // elevating to a float number
    f64::powf(d_whatever, std::f64::consts::PI); // elevate to float numbers

    // bitwise operations

    let _or = 1 | 2 ;
    let _and = 1 & 1;
    let _xor = 1^0 ;
    let _nor = !1 ;

    let _shift_left = 1 << 10;
    let _shift_right = 10 >> 4;

    // logical

    let _equals = 1 == 1;
    let _different = 1!=2;
    let _less_than = 1 < 3;
    let _more_than = 2>1;
    let _less_or_equal = 4<=4;
    let _more_or_equal = 5>=5;

    // non supported
    // a++;
    // ++a;
    // a--;
    // --a;
    // a^3;




}