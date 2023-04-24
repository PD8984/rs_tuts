fn main() {
    // print_phase("Hello");
    println!("{}", gcd(20, 5));
}

// fn print_phase(phrase: &str) {
//     println!("{}", phrase);
// }

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = b;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}
