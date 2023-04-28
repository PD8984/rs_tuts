fn main() {
    // let _var = 1; //created on stack
    // let mut s = "hello".to_string(); // created on heap
    // s.push_str("world")

    // let x = vec!["preet".to_string()];
    // let y = x;
    // let z = y;
    // println!("{:?}", z);

    // let x = vec!["preet".to_string()];
    // let y = x.clone();
    // let z = y.clone();
    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);

    // let x = 1;
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    let s = String::from("takes"); // create a variable with a string takes
    takes_ownership(s); //give ownership to the function

    let val = 1;
    make_copy(val);

    let str1: String = give_ownership();
    println!("{}", str1);
}

fn takes_ownership(s: String) {
    let strin = s;
    println!("{}", strin);
}
fn make_copy(one: i32) {
    let val1 = one;
    println!("{}", val1);
}

fn give_ownership() -> String {
    "given".to_string()
}

//var is dropped, s is also dropped
