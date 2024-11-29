mod cli;

fn main() {
    match cli::run() {
        Ok(msg)             => println!("{}", msg),

        Err((msg, code))    => {
            println!("{}", msg);
            std::process::exit(code);
        }
    };
}
