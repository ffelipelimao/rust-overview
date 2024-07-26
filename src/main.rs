fn main() {
    println!("Hello, world!");

    // First cargo is the package manager to rust
    // cargo init to start a project and cargo run to execute
    // so cargo will build at folder called targe

    // rust needs a fn main(){} to init your code

    // number types
    let a: i32 = 10;
    let b: u32 = 10; // unsigned int (only positives numbers)
    let c: f32 = 2.3; // float 32 bits range
    let d = 2.3_f32; //you can declare numbers like that
    
    // cast number    
    let e = 19 as f32;

    //chars
    let g: char = 'h';

    // using numbers and casting to chars with numbers to ASCII values
    let h = 65 as char;
    println!("{h}"); //should print A

    let j = false;
    let k = 10>5;
    println!("{k}");

    // we can use interpolation with calculations
    let slices = 20;
    let people = 6;

    println!("Each person gets {}, and {} left overs", slices / people, slices % people);

    // to use as float number, you need to cast them so slices as f64 / people as f64

    // format decimal numbers
    println!("Each person gets {:.2}", slices as f64 / people as f64);

    // increase and decrease with mut variable 
    let mut new_slices = 23;
    new_slices += 1;

    // using math functions
    new_slices = new_slices.pow(2); //power of 2 

    //using underscore for big numbers 
    let big = 1_000; // one thousand

    //tuples with mix types
    let grade = ("Limao", 69, 420, 24);
    //using index to access
    println!("{}", grade.1); // 69? nice!

    // first [types of the array, qtd of elements in array]
    let data: [i32; 3] = [1,2,3];
    let data_infer = [1,2,3];

    //using index to access array
    let first = data_infer[0];

    //len to get last position
    let last_element: i32 = data_infer[data_infer.len() -1];

    //simple loop
    for n in data{
        println!("{n}")
    }

}
