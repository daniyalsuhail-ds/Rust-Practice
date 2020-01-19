use std::mem;

fn main() {

    let a:i8 = -123; //8bits
    println!("a = {}", a);

    // mut
    let mut b:i8 = 0;
    println!("b = {}", b);
    b = 32;
    println!("b = {}", b);
    b = 22;
    println!("b = {}", b);

    let mut c = 123456789;
    println!("c = {}, size = {} bytes", c , mem::size_of_val(&c) );
    c = -1;
    println!("c = {}, size = {} bytes", c , mem::size_of_val(&c) );

    // i8 u8 i16 u16 i32 u32 i64 u64
    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS",
             z , size_of_z, size_of_z * 8);

    let d:char = 'x' ;
    println!("d = {}, size = {} bytes", d , mem::size_of_val(&d) );

    let e:f64 = 2.5;
    println!("e = {}, size = {} bytes", e , mem::size_of_val(&e) );

    let g = false;
    println!("g = {}, size = {} bytes", g , mem::size_of_val(&g) );

    operators();

}

fn operators()
{
    //arithmetic
    let mut a = 2+3*4;
    println!("{}", a);
    a = a+1 ;
    a -= 2 ; // a = a - 2;

    println!("remain of {} / {} = {}" , a , 3 , (a%3) );

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.6;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi: f64 = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    //bitwise operator
    let _n1 = 3 | 9; // OR & AND ^ XOR ! NOR
                        // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", _n1);
    let two_to_10 = 4 << 3;
    println!("2^10 = {}", two_to_10);

    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0;

    let _xyz = 5;
    let xyz_is_5 = _xyz == 5;

}