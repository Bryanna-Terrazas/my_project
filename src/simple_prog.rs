struct Person {
    name: String,
    age: i32,
}

enum Status {
    Child,
    Adult,
}

fn check_age(age: i32) -> Status {
    if age < 18 {
        Status::Child
    } else {
        Status::Adult
    }
}

// Ownership moves into this function
fn show_name(name: String) {
    println!("Name: {}", name);
}


//demo function to show ownership and pattern matching
pub fn demo() {
    let people = [
        Person { name: String::from("Ana"), age: 12 },
        Person { name: String::from("Bob"), age: 20 },
    ];

    for person in people {
        show_name(person.name);   // ownership of name moves here

        match check_age(person.age) {
            Status::Child => println!("Age: Child"),
            Status::Adult => println!("Age: Adult"),
        }
    }
}