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

    //string regular
    let name = "Luiz Felipe";
    //string specify the type
    let name2: &str = "Luiz Felipe";
    // static string, inject on the compiler code, you can't changed this, is not dynamic string
    let name_static: &'static str = "Estatico";

    //print string
    println!("Hello {name}");

    //Mutable string to concat, use :: to instance
    let mut name_mutable = String::from("Lebron");
    name_mutable += " James";
    println!("Hello {name_mutable}");
    
    //to be watch after set new variable if another, is not possible to use name2 anymore
    // this is a ownership data, rust doest not allow have the same data with sam
    // let new_name = name2;

    //bypass the ownership rule using format!
    let mut first_name = String::from("Gomez");
    let full_name = format!("{first_name} Limao");
    println!("Hello, {first_name}");

    //bypass using push_str
    let mut first_name = String::from("Gomez");
    let last = String::from("Limao");
    first_name.push_str(&last);

    //creating slices with string
    let mut slice_name_str = String::from("Caleb");
    println!("Hello, {}", &slice_name_str[3..5]);// should print eb index 3 and index 5
    println!("Hello, {}", &slice_name_str[..5]);// to get 0 index to 5 index
    println!("Hello, {}", &slice_name_str[6..]);// index 6 to end
    println!("Hello, {}", &slice_name_str[..]);// get all the elements


    //get each elements
    let mut each_ele = String::from("elements");
    for e in each_ele.chars(){
        println!("{e}")//type char
    }

}
