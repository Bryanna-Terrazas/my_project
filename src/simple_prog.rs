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

//Don't include : [Person; 2] in the onlineGDB version, it will cause an error.
//Also don't include : Person in the for loop, just use person instead. It will cause an error otherwise.
pub fn demo() {
    let people = [
        Person { name: String::from("Ana"), age: 12 },
        Person { name: String::from("Bob"), age: 20 },
    ];

    for person in people {
        match check_age(person.age) {
            Status::Child => println!("{} is a child", person.name),
            Status::Adult => println!("{} is an adult", person.name),
        }
    }
}