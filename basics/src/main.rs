#[derive(Clone, Copy, Debug, PartialEq)]
struct Number {
    odd: bool,
    value: i32,
}

fn print_number(n: &Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

use std::fmt::Debug;
fn compare<T>(left: T, right: T)
where
    T: Debug + PartialEq,
{
    println!("{:?} {} {:?}", left, if left == right { "==" } else { "!=" }, right);
}

fn make_tester(answer: String) -> impl Fn(&str) -> bool {
    move |challenge| {
        challenge == answer
    }
}

fn main() {

    let x = Number { odd: true, value: 51 };
    let y = Number { value: 34, ..x }; 
    let z = x;

    print_number(&x);
    print_number(&y);
    print_number(&z);

    compare(&x, &y);

    let o1: Option<i32> = Some(128);
    o1.unwrap(); // this is fine

    //let o2: Option<i32> = None;
    //o2.unwrap(); // this panics!
    
    let test = make_tester("hunter2".into());
    println!("{}", test("******"));
    println!("{}", test("hunter2"));    
    

}
