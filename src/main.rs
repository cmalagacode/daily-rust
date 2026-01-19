fn math(x: i32, y: i32) -> i32 {
   x + y
}

fn test_array() {
    let array_1: [i32; 5] = [1, 2, 3, 4, 5];
    let num = array_1[1]; // get element in index 1
    println!("{}", num); // prints 2
    println!("{:?}", array_1); // print the array
}

fn say_what() {
    println!("Hello, World");
}

fn main() {
    let _a_math_result = math(1, 2); // return 3 ---> i32
    test_array(); // return nothing
    say_what();
    compound_data_types();
}

fn compound_data_types() {
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
}