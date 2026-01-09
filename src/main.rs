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


fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    hello_world();
    two_pointer_a();
}