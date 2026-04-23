fn main() {
    // Three ways to print things using macros
    println!("Hello, world!");
    print!("I am learning Rust. ");
    print!("It is awesome!\n"); 
    
    // Variables

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

    /* 
        Data Types!

        Variables in Rust do not need to be declared. 
        Implicit Typing!
        However, we can still explicitly tell Rust what
        the type should be. 
    */
    // Letting Rust figure it out (although VS Code tries to put it in there):
    let _my_num = 5;         // integer
    let _my_double = 5.99;   // float
    let _my_letter = 'D';   // character
    let _my_bool = true;    // boolean
    let _my_text = "Hello"; // string

    // Explicitly stating what the type is:

    let _a_num: i32 = 5;         // integer
    let _a_double: f64 = 5.99;   // float
    let _a_letter: char = 'D';   // character
    let _a_bool: bool = true;    // boolean
    let _a_text: &str = "Hello"; // string

    // Constants NEED an explicitly stated type.
    const _BIRTHYEAR: i32 = 2005;
    const _MINUTES_PER_HOUR: i32 = 60;

    /* 
        Rust supports all the common operators like:
        Arithmetic Operators 
            +, -, *, /, %
        Assignment Operators
            =, +=, -=, *=, /=, %=
        Comparison Operators
            ==, !=, >, <, >=, <=
        Logical Operators
            &&, ||, !
    */
    // Arithmetic
    let addition = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", addition);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);

    // Assignment Ops
    let mut x = 10;
    println!("Start: {}", x);
    x += 5;
    println!("After += 5: {}", x);
    x -= 2;
    println!("After -= 2: {}", x);
    x *= 2;
    println!("After *= 2: {}", x);
    x /= 3;
    println!("After /= 3: {}", x);
    x %= 4;
    println!("After %= 4: {}", x);
    // Comparison Ops
    let a = 5;
    let b = 10;
    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);

    //Logical Ops
    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);

    /* 
        Conditional Statements!
        Use if to specify a block of code to be executed

        Use else to specify a block of code to be executed,
        if the same condition is false

        Use else if to specify a new condition to test if 
        the first condition is false

        Use match to specify many alternative blocks of code
        to be executed

        If..else can be used as a statement or an expression
        to assign a value to a variable in Rust. 

    */

    if 7 > 5{
        println!("7 is greater than 5.");
    }

    let age = 16;

    if age >= 18{
        println!("You can vote.");
    } else {
        println!("You are too young to vote.")
    }

    // If as an expression
    // Note: This is similar to the ternary operator (condition ? value1 : value2)
    // but Rust doesn't have a ternary operator.
    // Also, you can't mix types when doing this
    let time = 20;
    let greeting = if time < 18 {"Good day."} else {"Good evening."};
    println!("{}", greeting);

    //Match
    //Can be used to select one of many code blocks to be executed.

    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }
    // There can also be several matches using OR
    // The results of match can also be saved into a variable
    let result: &str = match day {
        1 | 2 | 3 | 4 | 5 => "Weekday",
        6 | 7 => "Weekend",
        _ => "Invalid day",
    };

    println!("{}", result);

    /*
        Loops!
        There are 3 types: loop, while, and for.

        break will stop the loop
        continue will skip a value in the loop
        
        loop will repeat forever until you stop it
    */
    let mut counter = 0;
    loop{
        println!("Crazy? I was crazy once...");
        println!("They put me in a room, a rubber room...");
        println!("A rubber room with rats, the rats made me crazy.");
        if counter == 20{
            break;
        }

        counter += 1;
        
    }
    // We can also use loop to return a value

    counter = 0;

    let end_result = loop{
        println!("I can still print stuff out!");

        if counter == 3{
            break counter; //Stops the loop and returns 3.
        }
        
        counter += 1;
    };

    println!("The loop stopped at: {}", end_result);

    // Like most languages, while loops will repeat while a condition is true
    counter = 0;
    while counter <= 5 {
        println!("{}", counter);
        counter += 1;
    }

    // for loops!
    // unlike while loops, if you know how many times you want it to run
    // use the for loop with the in keyword
    // this will print from 1 to 5, 1..6 means not including 6.
    for i in 1..6{
        println!("i is: {}", i);
    }
    // This will print 1 - 6. 
    for i in 1..=6{
        println!("i is: {}", i);
    }

    // Calling All Functions!

    say_hello(); 
    greet("Brie");

    let sum = add(3, 4);
    println!("Sum is: {}", sum);


}

/*
    Creating Functions!
    To create a function with a return value,
    we use the -> symbol in the header to show what type of 
    value will be returned.

    inside the function, use return to send the value back
*/

fn say_hello(){
    println!("Hello from a function!");
}

fn greet(name: &str){
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

