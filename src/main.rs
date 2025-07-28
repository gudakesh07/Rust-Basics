// mod hola;
// fn main(){
//     let x = 56;
//     let y: u32 = 340000;
//     let z = 1000.001;
//     println!("x: {}, y:{}, z:{}",x,y,z)
// }

fn main(){
    let is_male = true;
    let is_above_18 = false;
    if (is_male){
        print!("You are a male.")
    } else{
        print!("You are not a male.")
    }
    if is_male && is_above_18{
        print!("You are a legal male.")
    }
}
// fn main(){
//     let greeting = String::from("Hello World!!!");
//     println!("{}", greeting);

//     let char1 = greeting.chars().nth(1);
//     print!("char1: {}", char1.unwrap())
// }