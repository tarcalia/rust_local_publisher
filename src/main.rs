use std::io;
use services::book_service::handle_input;

mod services;
mod models;

fn main() {
    loop {
        println!("Give command or write 'exit' to quit.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error during read input");

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting application");
            break;
        }

        handle_input(&input);

        println!("Received word is: {}", input);
    }
}
