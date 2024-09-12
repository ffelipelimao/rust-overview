use std::char;
use rust_decimal_macros::dec; //importando apenas o dec 
use std::fs;

mod game_level;

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

    /*let result = match character{
        Some(character) => character,
        None => (),
    };
    */


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

    // iterators

    let iter_names = vec!["Test1","Test2","Test3",];
    let mut iter_data = iter_names.iter();

    let mut iter_item = iter_data.next();

    let value = match iter_item{
        Some(item) => "item",
        None => &"",
    };

    // closures e iterations adapters (basicamente manipular os dados de um interator map, filter e reduce ou fold)

    let vec_data = vec![1,2,3,4];

    // crie o iterator, chame o map para cada elemento multiplicaar e depois o collector para devolver um vetor
    let squared: Vec<_> = vec_data.iter().map(|x| x*x).collect();

    // cria um vector com tuplas (tipo o map)
    let products = vec![
        ("laptop charger", 25.0),
        ("keyboard", 22.0),
        ("mouse", 18.0),
        ("monitor", 30.0),
        ("cable", 5.0),
    ];


    //aplica o desconto de 10% em cada produto com o iter de map com tuplas usando o || para pega o valor de cada elemento,
    // que nesse caso é uma tupla, entao se usa ()
    let discounted: Vec<_> = products
        .iter()
        .map(|(name, price)| (name, price * 0.9))
        .collect();

    println!("discounted: {:?}", discounted);

    // Criando escopo diferentes da função main
    {
        let name = String::from("Scope");
    }

    // movendo valores, após isso pointer1 nao e mais acessivel
    let pointer1 = String::from("Ownership");
    let pointer2 = pointer1;


    let xxxx = String::from("Reference");
    let yyy = &xxxx;

    println!("{xxxx} , {yyy}");

    // let _x_mut = &mut xxxx;
    let p_y = String::from("Ref");

    //slices
    let slices_awn = String::from("Hello World");
    let first = &slices_awn[0..4];
    let last = &slices_awn[6..];

    //funções
    let result = greet("Felipe");

    //as vezes nem precisa return (isso é doidera)
    let times_two = anonymous_return(2);

    //closures
    let scores = vec![1,2,3];
    let my_print = || println!("{scores:?}");

    //crates são suas dependencias, voce importa elas como use
    // let dddd = dec!(0.1);

    game::print_points();
    
    //voce pode ter modulos dentro de modulos, e para acessar eles com super (hierarquia mesmo)

    //usando mod dentro desse arquivo e ter um game_level com mesmo nome do seu mod, voce pode pegar ele de outro arquivo
    game_level::print_game_level();

    // Macros em Rust são ferramentas para gerar código automaticamente, permitindo criar trechos de código reutilizáveis e reduzindo a repetição. 
    // Eles são usados para tarefas como criar funções, estruturas ou manipular dados de forma dinâmica.
    
    // Definindo uma macro chamada 'ola'
    macro_rules! ola {
        () => {
        println!("Olá, mundo!");
        };
    }
    
    // Usando a macro
      ola!();

    //Se um tipo implementa uma trait, ele precisa seguir as regras e métodos definidos por ela, como uma interface em outras linguagens.
    trait Desenhavel {
        fn desenhar(&self);
    }
    
    // Implementando a trait para uma struct
    struct Circulo;
    struct Quadrado;
    
    impl Desenhavel for Circulo {
        fn desenhar(&self) {
            println!("Desenhando um círculo");
        }
    }
    
    impl Desenhavel for Quadrado {
        fn desenhar(&self) {
            println!("Desenhando um quadrado");
        }
    }

    let c = Circulo;
    let q = Quadrado;

    c.desenhar();
    q.desenhar();

    //structs
    struct Player {
        x: i32,
        y: i32
    }

    // &mut self se refera a propria instancia do player
    impl Player {
        pub fn move_plyer_x(&mut self) -> i32{
            self.x*2
        }
    }

    // mutavel pois eu a func move_plyer_x altera seu valor
    let mut player = Player{
        x: 10, 
        y: 23
    };

    println!("player x level is {}", player.x);
    println!("player moved, now x level is {}", player.move_plyer_x());


    //erros
    //podemos tratar quando uma fn retorna erro com expect, porem podemos para o erro para funcao de cima com "?"
    // let content = fs::read("ex.txt").expect("enable to read");
    let numero = "10".parse::<i32>().expect("Erro ao converter para número");
    println!("Número: {}", numero);
    
    //passando o erro pra fn de cima
   
    // let content = fs::read("ex.txt")?;
    match dividir(10, 0) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(e) => println!("Erro: {}", e),
    }

    /*
    Box é um ponteiro inteligente que aloca um valor na heap em vez da stack. 
    Ele é útil quando você quer mover um valor grande ou de tamanho desconhecido em tempo de compilação para a heap, ou 
    quando precisa de um tipo que tenha um tamanho fixo, como em casos de recursão.

    Principais usos do Box:

	1.	Alocar grandes quantidades de dados na heap.
	2.	Trabalhar com tipos que têm tamanho desconhecido.
	3.	Habilitar recursão em structs.
    */

     let valor = Box::new(10); // Aloca o número 10 na heap
     println!("Valor: {}", valor);

 }

fn greet(name: &str) -> String {
    return format!("Greetings {name}")
}

fn anonymous_return(n: i32) -> i32{
    n*2
}

// criando um modulo
mod game {

    //publica para consegui importar depoi
    pub fn print_points(){
        println!("32")
    }
}

pub fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divisão por zero"))
    } else {
        Ok(a / b)
    }
}