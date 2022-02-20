use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;
fn main() {

    // play_game();
    // something();
    // iter_mut_test();
    vector_test();

}

fn play_game() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        println!("Guess the number");
        io::stdin().read_line(&mut guess).expect("failed to read line");
        let guess = match guess.as_str().trim().parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                println!("guess a proper number");
                continue;
            }
        };

        println!("guess: {}, secret_number: {}", guess, secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}","equal , good guess".green());
                break;
            }
            Ordering::Less => println!("{}","lesser , bad guess".red()),
            Ordering::Greater => println!("{}","greater , bad guess".blue()),
        }
    }
}


fn something() {
    struct A<'a> {
        some_int: u32,
        some_str: &'a str,
    }

    impl<'a> A<'a> {
        fn new(x: u32, s: &'a str) -> Self {
            Self { some_int: x , some_str: s }
        }

        fn modify(&mut self, y: u32) {
            self.some_int = y;
        }

        fn get(&self) -> u32{
            self.some_int
        }

        fn get_str(&self) -> &str {
            self.some_str
        }
    }
    let mut a = A::new(32, "");

    a.modify(55);
    let some_int = a.get();
    let some_str = a.get_str();
    println!("{} {}", some_int, some_str);
}

fn iter_mut_test() {
    let mut a = [1,2,3,4];
    for x in a.iter_mut() {
        *x += 1;
    }
    assert_eq!(a, [2,3,4,5]);
}

fn vector_test() {
    let mut v = Vec::<u32>::new();
    v.push(23);
    
    assert_eq!(v, vec![23]);
    
    for a in v.iter_mut() {
        *a += 23;
    }
    assert_eq!(v, vec![46]);
}