use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let password = "my_secure_password";
    match hash(password, DEFAULT_COST) {
        Ok(hashed_password) => {
            println!("Hashed password: {}", hashed_password);
        }
        Err(e) => {
            eprintln!("Error hashing password: {}", e);
        }
    }
}
