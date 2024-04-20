use regex::Regex;

// Validate password. TODO: fix it
pub fn validate_password (password:String)->bool{
    let re = Regex::new(r"^(?<=(.*[a-z]){3,})(?<=(.*[A-Z]){1,})(?<=(.*[0-9]){2,})(?<=(.*[!@#$%^&*()\-__+.]){1,}).{12,134}$").unwrap();
    return !re.is_match(&password);
}  


// Check password length. Set to minimi 12
pub fn is_valin_length( password:String)->bool{
    return password.len()>11
}

// Check if password string contains numeric.
pub fn contains_digit(password:String)->bool{
    let mut is_numeric:bool = false;  
    for char_pass in password.chars(){
        if char_pass.is_numeric() {
            is_numeric = true;
            break;
        }
    }
    return is_numeric;
}

// Check if password contains special character.
fn contains_special_char (password:String)->bool{
    let special_chars = "!@#$%^&*()\\-__+.";
    let mut is_special:bool = false;  
    for char_spec in special_chars.chars(){
        if password.contains(special_chars) {
            is_special = true;
            break;
        }
    }
    return is_special;
}

 fn contais_upper_lower_chars (password:String)->bool{
    let mut is_numeric:bool = false;  
    let mut is_upper:bool = false;
    for char_pass in password.chars(){
        if char_pass.is_numeric() {
            is_numeric = true;
            break;
        }
    }
    return is_numeric
 }