
fn main() {
    println!("Programa principal!");
    println!("=====================");
    println!("--- Variables --------");
    let mut x = 5;
    // let x: i8x1 = 10; // Declarar e inicializar una variable de tipo entero.
    println!("El valor de x es: {}",x);
    println!("Ahorita el valor de x es inmutable: {}",x);
    println!("Pero si queremos que sea mutable es decir poder cambiar el valor, durante la ejecucion del programa.
    Anadimos lo siguiente let mut x = <value>");

    // println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
