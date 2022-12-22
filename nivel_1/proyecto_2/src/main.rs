use std::io;

fn main() 
{
    //Imprime las diferentes operaciones disponibles.
    println!("Calculadora\n");
    println!("¿Que quieres hacer?");
    println!("\tSumar\t\t[1]");
    println!("\tRestar\t\t[2]");
    println!("\tMultiplicar\t[3]");
    println!("\tDividir\t\t[4]");
    println!("\tSalir\t\t[*]");
    //Crea la variable donde se guardare la opcion seleccionada
    let select = read_int();
    //Dependiendo
    match select
    {
        1 => suma(),
        2 => resta(),
        3 => multi(),
        4 => divid(),
        _ => return,
    }
}

fn suma()
{
    println!("Introduce el primer numero a sumar:");
    let x = read_int();
    println!("Introduce el segundo numero a sumar:");
    let y = read_int();
    print!("\nResultado: ");
    print!("{}", x+y);
    println!("");
}

fn resta()
{
    println!("Introduce el primer numero a restar:");
    let x = read_int();
    println!("Introduce el segundo numero a restar:");
    let y = read_int();
    print!("\nResultado: ");
    print!("{}", x-y);
    println!("");
}

fn multi()
{
    println!("Introduce el primer numero a multiplicar:");
    let x = read_int();
    println!("Introduce el segundo numero a multiplicar:");
    let y = read_int();
    print!("\nResultado: ");
    print!("{}", x*y);
    println!("");
}

fn divid()
{
    println!("Introduce el primer numero a dividir:");
    let x = read_int();
    println!("Introduce el segundo numero a dividir:");
    let y = read_int();
    if y != 0
    {
        print!("\nResultado: ");
        print!("{}", x/y);
        println!("");
    }
    else 
    {
        println!("No se puede dividir por 0");
    }
}

fn read_int() -> i32 {
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
