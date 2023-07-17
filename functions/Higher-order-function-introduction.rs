// Higher-order function that takes a function as an argument
fn apply_function(value: i32, func: Box<dyn Fn(i32) -> i32>) -> i32 {
    func(value)
}

// Function that returns another function
fn create_multiplier(factor: i32) -> Box<dyn Fn(i32) -> i32> {
    let multiplier = move |value: i32| -> i32 {
        value * factor
    };
    Box::new(multiplier)
}

fn main() {
    let result = apply_function(5, Box::new(|x| x * x)); // Call apply_function with a closure that squares the input
    println!("Result: {}", result);

    let multiply_by_3 = create_multiplier(3); // Call create_multiplier to obtain a function that multiplies by 3
    let result = multiply_by_3(7); // Call the obtained function with an argument of 7
    println!("Result: {}", result);
}