use regex::Regex;

// Validate password.
pub fn validate_password (password:String)->bool{
    let re = Regex::new(r"^(?<=(.*[a-z]){3,})(?<=(.*[A-Z]){1,})(?<=(.*[0-9]){2,})(?<=(.*[!@#$%^&*()\-__+.]){1,}).{12,134}$").unwrap();
     re.is_match(&password)
}