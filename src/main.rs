use rpassword;
use zeroize::{Zeroizing};

// entry point of execution in rust (main function)
fn main() {
    //let mut input = String::new();
    println!("Enter password:");
    let output = rpassword::read_password();
    let mut password_input = Zeroizing::new(match output {
        Ok(password) => { 
            println!("Voila.");
            password
        },
        Err(err) => {
            eprintln!("Failed to read the password. {}",err);
            std::process::exit(1)
        },
    });

    let cutoff = if password_input.len() > 3 { password_input.len() - 3 } else { password_input.len() };
    
// We are mutating the underlying byte buffer of the String.
// This is unsafe because String must always contain valid UTF-8.
// We guarantee safety here because:
// 1. We only overwrite existing bytes within bounds (0..cutoff).
// 2. We do not change length or capacity.
// 3. We only write ASCII '*' (valid UTF-8).
    unsafe {
        let bytes = password_input.as_mut_vec();
        
        // iterating over the password 
        for i in 0..cutoff{
            bytes[i] = b'*'
        }
    }

    //let raw_ptr = password_input.as_ptr();

}
