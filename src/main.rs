use std::io;

fn main() {
    println!("Temp Converter");

    loop {
        println!("Please choose input unit (C/F)");
        let mut input = String::new();
        let mut input_temp = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input = input.trim();

        match input {
            "help" => {
                println!(
                    "Enter temp (in Celsius) to convert to fahrenheit\nType 'exit' to exit.\n"
                );
                continue;
            }
            "exit" => break,
            "C" => {
                println!("Please enter temperature in C");

                io::stdin()
                    .read_line(&mut input_temp)
                    .expect("Failed to read line.");

                match input_temp.trim().parse::<f32>() {
                    Ok(num) => println!("{} degrees fahrenheit.\n", convert(num, 'C')),
                    Err(_) => {
                        println!("That's not a number!\n");
                        continue;
                    }
                };
            }
            "F" => {
                println!("Please enter temperature in F");

                io::stdin()
                    .read_line(&mut input_temp)
                    .expect("Failed to read line.");

                match input_temp.trim().parse::<f32>() {
                    Ok(num) => println!("{} degrees fahrenheit.\n", convert(num, 'F')),
                    Err(_) => {
                        println!("That's not a number!\n");
                        continue;
                    }
                };
            }
            &_ => println!(
                "Sorry, I don't recognise that command, enter 'help' for a list of commands."
            ),
        }
    }
}

fn convert(temp: f32, unit: char) -> f32 {
    if unit == 'C' {
        let n: f32 = 9f32 / 5f32;
        temp * n + 32f32
    } else {
        let n: f32 = 5f32 / 9f32;
        (temp - 32f32) * n
    }
}
