use regex::Regex;
// Validate password.
pub fn validate_password (password:String)->bool{
    let re = Regex::new(r"^(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{12,500}$").unwrap();
    return !(password.len()>0 || !re.is_match(&password) || check_space_char(password) || !check_special_char(password));
}

// Check if contains empty space.
fn check_space_char(input:String)->bool{
    return input.contains(" ")
}


//Check if char has spacial character.
fn check_special_char(input:String)->bool{
    let specia_chars=("\\|!#$%&/()=?»«@£§€{}.-;'<>_ ");
    return input.chars().any(|c| matches!(specia_chars));
}