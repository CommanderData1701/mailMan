use rpassword::read_password;
use std::io;
use std::io::Write;

// TODO: implement
pub fn authenticate(pw: Option<String>) -> Result<String, (String, i32)> {
    print!("Enter password: ");
    let _ = io::stdout().flush();

    let password = match pw {
        Some(content) => content,
        None => {
            match read_password() {
                Err(e) =>  {
                    let message = format!("Failed to read password: {}", e);
                    return Err((message.to_string(), 1));
                },
                
                Ok(password) => password,
            }
        }
    };

    Err(("Not implemented!".to_string(), 1))
}
