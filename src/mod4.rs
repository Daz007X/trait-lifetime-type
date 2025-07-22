use std::fmt::Debug;

// A trait for logging
pub trait Logger {
    fn log(&self, message: &str);
}

// An advanced trait for processing data
pub trait DataProcessor<T>: Logger
where
    T: Debug,
    Self::Output: Debug, 
{
    // Associated type for the result
    type Output;

    // Required method
    fn process(&self, input: T) -> Self::Output;

    // Default method using the Logger trait
    fn process_with_logging(&self, input: T) -> Self::Output {
        self.log(&format!("Processing input: {:?}", input));
        let result = self.process(input);
        self.log(&format!("Result: {:?}", result));
        result
    }
}

// A concrete implementation
struct MyProcessor;

impl Logger for MyProcessor {
    fn log(&self, message: &str) {
        println!("[LOG]: {}", message);
    }
}

impl DataProcessor<i32> for MyProcessor {
    type Output = i32;

    fn process(&self, input: i32) -> Self::Output {
        input * 2
    }
}

fn main() {
    let processor = MyProcessor;
    let result = processor.process_with_logging(21);
    println!("Final result: {}", result);
}
