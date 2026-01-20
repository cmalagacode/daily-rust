// String (String::from(&str)) vs &str (string slice)


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
    let _x = vec![1, 2, 3,4, 5];
}


fn hello_world() {
    println!("Hello, world!");
}

fn shadowing_part2() {
    let x = 10;
    println!("x: {}", x);
    let x = "Thi is a str";
    println!("x: {}", x);
    const _MAX_NUMBER: u32 = 100; // once created constants cannot be mutated .. must be known at compile time ..
    static _MIN_NUMBER: u32 = 90; // const gets inlined .. but static is a constant that does not
}

fn usize_practice() {
    let x: usize = 10;
    println!("x: {}", x);
    let b = true;
    print!("{}\n", b);
    let mut array_1: [i32; 5] = [4, 5, 6, 6, 10];
    array_1[4] = 100;
    println!("{:?}", array_1);
    let array_2 = [0; 10];
    println!("{:?}", array_2);
}

fn all_about_vector() {
    let vec_1: Vec<i32> = vec![4, 5, 6, 8, 9];
    println!("vec_1 values = {:?}", vec_1);
}

fn all_about_tuples() {
    let database_row = ("122323", "John", "Doe", 40);
    println!("database_row = {:?}", database_row);
}

fn fun_string() {
    // string slice
    let a = "this is a string slice";
    let mut b = String::from(a); // this is a string that is mutable
    let mut b_vec: Vec<char> = b.chars().collect();
    b_vec[0] = '0';
    b = b_vec.into_iter().collect();
    println!("Printing mutaded string: {}", b);
}

fn main() {
    rust_array();
    hello_world();
    two_pointer_a();
    // abcdcba
    let result = is_pali("abcdcba");
    println!("Result is {} it is a pali", result);
    shadowing_rusty();
    shadowing_part2();
    usize_practice();
    println!("==============");
    all_about_vector();
    println!("==============");
    println!("==============");
    all_about_tuples();
    println!("==============");
    fun_string();
    println!("Hello");
    compound_data_types();
}
fn shadowing_rusty() {
    let x = 5;
    println!("{}", x);
    {
        let x = 10;
        println!("{}", x);
    }
}

fn rust_array() {
    let grades: [f32; 5] = [50.23, 100.00, 92.33, 40.33, 33.33];
    println!("Grades: {:?}", grades);
    let names: [&str; 5] = ["John", "Jane", "Jack", "Jill", "Jim"];
    println!("Names: {:?}", names);
    let other = [100, 200, 300];
    println!("Other: {:?}", other);
}

fn compound_data_types() {
    println!("**Coumpound Data Types Function:**");
    // arrays
    let mut array_1: [i32; 5] = [1, 2, 3, 4, 5];
    let num = array_1[4]; // element value: 5
    array_1[1] = 10; //[1, 10, 3, 4, 5];
    println!("Array: {:?}", array_1);

    // vector
    let mut vec_1 = vec![1, 2, 3, 4, 5];
    println!("Vec: {:?}", vec_1);

    // tuples
    let tup: (i32, &str) = (400, "hello");
    println!("Tuple: {:?}", tup);
    println!("Tuple index 0: {}", tup.0);
    let tup2 = (100, 200);
    let (x, y) = tup2;
    println!("Tuple x: {} .. y {}", x, y);
    // unit type .. tuple
    let _unit = (); // does not consume memory
    let x = 22; // shadow
    println!("{}", x);
    println!("{}", x);
    println!("**Coumpound Data Types Function:**");
}