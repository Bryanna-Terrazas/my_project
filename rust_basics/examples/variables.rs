fn main() {
    // 1. Immutable variable (cannot be changed)
    let school = "UTRGV";
    
    // 2. Mutable variable (can be changed)
    let mut credits = 12;
    credits = 15; // Changing the value
    
    // 3. Variable with a specific data type (Integer)
    let year: i32 = 2026;
    // 4. {} are place holders for the vairable. if not then a error will occur
    println!("Welcome to {}, year {}. You have {} credits.", school, year, credits);
}