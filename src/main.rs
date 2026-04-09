fn main() {
//Three ways to print things using macros
    println!("Hello, world!");
    print!("I am learning Rust. ");
    print!("It is awesome!\n"); 
    
 //Variables

    let name = "John";
    let last_name = "Doe";

/*
    Immutable Variables!
    By default, variables cannot be changed after they are created

    For example: 
    let x = 5;
    x = 10; //Error
    println!("{}", x); //Would not work at all
    
    We use the mut keyword to be able to change the variables
*/
    
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);

/*
    By the way...
    {} are used as a placeholder to show variable values. 
    The variables are printed in their respective order.
    So name would be printed first, followed by last_name 
*/
    println!("My full name is: {} {}", name, last_name);



    

    





}
