fn calculate_grade() {
    println!("Enter the score (0-100):");
    let mut score = String::new();
    std::io::stdin()
        .read_line(&mut score)
        .expect("Failed to read line");
    let score: u32 = score.trim().parse().expect("Please enter a valid number");

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else if score >= 60 {
        println!("Grade: D");
    } else {
        println!("Grade: F");
    }
}

fn convert_temperature() {
    println!("Enter temperature in Celsius:");
    let mut celsius = String::new();
    std::io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");
    let celsius: f64 = celsius.trim().parse().expect("Please enter a valid number");

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("Temperature in Fahrenheit: {:.2}", fahrenheit);
}

fn categorize_number() {
    println!("Enter a number:");
    let mut number = String::new();
    std::io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number: i32 = number.trim().parse().expect("Please enter a valid number");

    if number % 2 == 0 {
        println!("The number is Even.");
    } else {
        println!("The number is Odd.");
    }
}

fn main() {
    loop {
        println!("Choose one of the following options:");
        println!("1. Calculate Grade");
        println!("2. Convert Temperature (Celsius to Fahrenheit)");
        println!("3. Classify Number (Even or Odd)");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

        if choice == 1 {
            calculate_grade();
        } else if choice == 2 {
            convert_temperature();
        } else if choice == 3 {
            categorize_number();
        } else if choice == 4 {
            break;
        } else {
            println!("Invalid choice. Please run the program again.");
        }
    }
}
