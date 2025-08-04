use List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    match list {
        Cons(value, cons) => {
            println!("First value: {}", value);
            match *cons {
                Cons(value, cons2) => println!("Second value: {}", value),
                Nil => println!("Empty list"),
            }
        },
        Nil => println!("Empty list"),
    }
}
