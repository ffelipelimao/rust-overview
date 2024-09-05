use std::char;

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
    let mut new_slices: i32 = 23;
    new_slices += 1;

    // using math functions
    // new_slices = new_slices.pow(2); //power of 2 

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
    //println!("Hello, {}", &slice_name_str[6..]);// index 6 to end
    println!("Hello, {}", &slice_name_str[..]);// get all the elements


    //get each elements
    let mut each_ele = String::from("elements");
    for e in each_ele.chars(){
        println!("{e}")//type char
    }


    //Vectors
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(5);
    numbers.push(10);
    numbers.push(20);

    println!("{}", numbers.get(2).expect("index doest not exits"));

    //Vectors using macros
    let mut numbers_x2: Vec<i32> = vec![5,10,20];

    //Vectors with capacity
    let mut numbers_x3: Vec<i32> = Vec::with_capacity(3);

    //Iterator with vectors
    for num in numbers_x3.iter(){
        println!("{num}");
    }

    //Iterator with vectors mutable
    for num_2 in numbers_x3.iter_mut(){
        *num_2 += 2;
        println!("{num_2}");
    }

    //easy way to see date
    println!("{numbers_x2:?}");

    //cast array to Vec
    let mut numbers_array = [1,2,4];
    let mut numbers_vec = numbers_array.to_vec();

    //vec can be easier to use and they are dynamic in size, you can use to another types to

    //Enum - can be use outside of main
    enum Membership {
        Bronze, 
        Silver,
        Gold,
    }

    let status = Membership::Gold;

    //match expression, looks like a switch case
   let message = match status {
        Membership::Bronze | Membership::Silver => "Basic benefits!!",
        Membership::Gold => "All benefits",
    };

    println!("{message}");

    //Option,Result

    //Result quer dizer que pode ser considerado um possivel erro

    let example = String::from("Limao");
    let character = example.chars().nth(2); //pega um element especfico ao transformar em chars

    //pub fn nth(&mut self, n: usize) -> Option<Self::Item> - Ao ver o nth retorna um Option que é basicamente voce ter o dado ou não
    //o tipo do character é Option<char> então pra isso precisamos usar um match expression

    let result = match character{
        Some(character) => character,
        None => ""
    };


    //em rust o correto a se falar é "if expression"
    let age = 18;
    if age >= 18{
        println!("your are an adult");
    } else {
        println!("your are a child");
    }

    // podemos atribuir valor com o resultado de if 
    let is_adult = if age >= 19 {
        true
    } else {
        false
    };

    let points = 48;
    let bonus = if points > 30{ 10 } else { -10 };

    // loops
    let mut number_loops = 0;

    let result = loop {
        number_loops += 1;
        if number_loops == 10{
            break; number_loops * 20;
        }
    };
    
    // while
    while number_loops > 0{
        number_loops -= 1;
    }

    // for in
    for i in 1..6{
        println!("{i}")
    }

    // for in reverse
    for i in (1..6).rev(){
        println!("{i}")
    }

}
