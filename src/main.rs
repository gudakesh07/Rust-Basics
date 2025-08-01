fn main() {
    let x: i32 = -56;
    let y = 1000;
    let z = 100.000;
    println!("{}", x);
    println!("{}", y);

    print!("x:{}, y:{}, z:{}",x,y,z);

    let mut a: i8 = 10;

    // for i in 0..1000{
    //     a = a + 100;
    // }
    println!("{}", a); // since compiler statically checks the code it never checks the code on runtime that's why 
    // even after knowing that a is i8 it never corrects the code.

    let is_male = false;
    let mut is_above_18 = false;

    // Since all the variables are mutable by default in rust we need to use mut if we want to change the value!!!

    is_above_18 = true;
    
    if is_male {
        println!("Yes that person is a male!!!");
    } else {
        println!("You are not a male!!!");
    }

    if is_male && is_above_18{
        println!("You are a legal male!!!");
    }


    // Strings

    let greeting = String::from("Hello World!!");

    println!("{}", greeting);

    // println!("{}", greeting.chars().nth(1000));

    // Conditionals

    let is_even = 99;

    if is_even % 2 == 0 {
        println!("The number is even.")
    } else if is_even % 2 == 1{
        println!("The number is odd!!!")
    }

    // Conditionals

    for i in 0..10{
        println!("{}", i)
    }
    
}
