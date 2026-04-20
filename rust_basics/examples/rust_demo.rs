fn main() {
    // --- 1. IMMUTABLE (Default in Rust) ---
    // In C++, this is changeable. In Rust, it's locked without 'mut'.
    let score = 95; 

    // --- 2. RANGES (The 'Professor' request) ---
    // Instead of (score >= 90 && score <= 100), we use inclusive ranges.
    if (90..=100).contains(&score) {
        println!("Grade: A");
    } else if (80..90).contains(&score) {
        println!("Grade: B");
    } else if (70..80).contains(&score) {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }

    let school = "UTRGV";
    
    // --- 3. MUTABLE (Explicitly unlocked) ---
    let mut credits = 12; 
    
    // We print this so the professor sees the change happen live
    println!("Welcome to {}. Starting credits: {}", school, credits);

    // Direct reassignment to show 'mut' in action
    credits = 18; 
    
    let year: i32 = 2026;

    println!("Update for year {}: You now have {} credits.", year, credits);
}



    /* let score = 90;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B"); 
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }

        // 1. Immutable variable (cannot be changed)
    let school = "UTRGV";
    
    // 2. Mutable variable (can be changed)
    let mut credits = 12;
    credits = 18; // Changing the value
    
    // 3. Variable with a specific data type (Integer)
    let year: i32 = 2026;
    // 4. {} are place holders for the vairable. if not then a error will occur
    println!("Welcome to {}, year {}. You have {} credits.", school, year, credits);
}
    */