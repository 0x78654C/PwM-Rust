// Validate password.
// Check upper/lower case x1, digits, special chars, length of 12.
pub fn validate_password (password:String)->bool{
    let pass = password;
    let mut is_valid:bool=false;
    if is_valid_length(pass.clone()) && contains_digit(pass.clone()) && contains_special_char(pass.clone()) && contais_lower_chars(pass.clone()) && contais_upper_chars(pass.clone()) {
        is_valid = true;
    }
    return is_valid;
}  


// Check password length. Set to minimi 12
fn is_valid_length( password:String)->bool{
    return password.len()>11;
}

// Check if password string contains numeric.
pub fn contains_digit(password: String)->bool{
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
        if password.contains(char_spec) {
            is_special = true;
            break;
        }
    }
    return is_special;
}

// Check if is lower case.
 fn contais_lower_chars (password:String)->bool{
    let mut is_lower:bool = false;
    for char_pass in password.chars(){
        if char_pass.is_ascii_lowercase() {
            is_lower =true;
            break;
        }
    }
    return is_lower;
 }

 // Check if is upper case.
 fn contais_upper_chars (password:String)->bool{
    let mut is_upper:bool = false;
    for char_pass in password.chars(){
        if char_pass.is_ascii_uppercase() {
            is_upper =true;
            break;
        }
    }
    return is_upper;
 }