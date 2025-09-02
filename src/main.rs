// fn main() {
//     println!("Hello, world!");
// }


//Basic Varibles in Rust

// numbers
//i32--> decide the number size , i means signed integer..since rust is an memeory based language
//i -->signed number 
//u-->unsigned number
//f-->floating number

// if x :i8 ..number assigned is 10000 beyond the memoery space of i8 , then we compiler will throw an error, but x=x+100,,, will not throw error as complier does not work on run time logic implemented


// fn main(){
//     let mut x: i32 =18;
//     // let y:u32 =67;
//     // let z:f32= 100.23;

//     for _ in 0..1000{
//         x= x+100;
//     }
//     // println!("x:{},y:{},z:{}",x,y,z);
//     // println!("{} {} {}",x,y,z)

//     print!("{}",x)
// }



//BOOLEANS

fn main(){
    let is_male : bool=true;
    let is_above_18: bool=true;

    if is_male {
        println!("You are a male");
    }
    else{
        println!("Your are not a male");

    }

    if is_male && is_above_18{
        println!("Legal hai bhai legal hai")
    }
}