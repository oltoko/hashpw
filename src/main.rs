use scrypt::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Scrypt
};
use std::io::Write;
use rpassword::read_password;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // read the password from stdin
    print!("Enter password: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    // re-enter the password
    print!("Re-Enter password: ");
    std::io::stdout().flush().unwrap();
    let re_entered_password = read_password().unwrap();

    // check if both passwords match
    if password != re_entered_password {
        return Err("Passwords do not match!".into());
    }

    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($scrypt$...)
    let password_hash = Scrypt.hash_password(password.as_bytes(), &salt)?.to_string();
    println!("Password hash: {}", password_hash);

    Ok(())
}
