// Define a function to calculate the factorial of a number using recursion
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Define a struct to represent a simple person
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Constructor method for Person
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: String::from(name),
            age,
        }
    }

    // Method to introduce the person
    fn introduce(&self) {
        println!("Hello, my name is {} and I'm {} years old.", self.name, self.age);
    }
}

fn main() {
    // Declare and initialize variables
    let x = 5;
    let y: u32 = 10;

    // Print the result of a calculation
    println!("The sum of {} and {} is {}", x, y, x + y);

    // Use a conditional statement
    if x < y {
        println!("{} is less than {}", x, y);
    } else {
        println!("{} is greater than or equal to {}", x, y);
    }

    // Use a loop to print factorial of numbers
    for i in 0..=5 {
        println!("Factorial of {} is {}", i, factorial(i));
    }

    // Create an instance of the Person struct
    let person = Person::new("Alice", 30);

    // Call a method on the Person instance
    person.introduce();
}
