fn two_pointer_a() {
    let data = vec![1, 2, 3, 4, 5];
    let mut left = 0;
    let mut right = data.len() - 1;
    while left < right {
        println!("left: {} .. right: {}", left, right);
        left += 1;
        right -= 1;
    }
    println!("Data Is The Same: left: {} .. right: {}", left, right);
}
fn is_pali(x: &str) -> bool {
    let mut left = 0;
    let mut right = x.len() - 1;
    while left < right {
        if x.chars().nth(left) != x.chars().nth(right) {
            return false;
        }
        left += 1;
        right -= 1;
    }
    return true;
}

fn vect_test() {
    let x = vec![1, 2, 3,4, 5];
}


fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    hello_world();
    two_pointer_a();
    // abcdcba
    let result = is_pali("abcdcba");
    println!("Result is {} it is a pali", result);
    shadowing_rusty();
}
fn shadowing_rusty() {
    let x = 5;
    println!("{}", x);
    {
        let x = 10;
        println!("{}", x);
    }

    let x = 22; // shadow
    println!("{}", x);
}