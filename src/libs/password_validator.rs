use regex::Regex;

// Validate password.
pub fn validate_password (password:String)->bool{
    let re = Regex::new(r"^(\d)([a-z])([A-Z]).{12,500}$").unwrap();
     re.is_match(&password)
}