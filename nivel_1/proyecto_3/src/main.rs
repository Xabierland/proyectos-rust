use rand::thread_rng;
use rand::Rng;
use std::io;

fn main() {
    // Crea la variable RNG la cual es un objeto del tipo ThreadRng
    let mut rng = thread_rng();
    //Usando la funcion gen_range de los objetos ThreadRng generamos un numero entre 0 y 99
    let rng_num: i32 = rng.gen_range(0,100);
    println!("Introduce el numero a comparar:");
    let mut user_num = read_int();
    while user_num != rng_num
    {
        if user_num<rng_num
        {
            println!("El numero es mayor.");
            println!("Introduce el siguiente numero:");
            user_num = read_int();
        }
        else 
        {
            println!("El numero es menor.");
            println!("Introduce el siguiente numero:");
            user_num = read_int();
            
        }
    }
    println!("¡Has acertado!");

}

fn read_int() -> i32 
{
    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("Error al leer línea");

    match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Ese no es un número entero válido, lo tomare como 0");
            return 0;
        },
    }
}
