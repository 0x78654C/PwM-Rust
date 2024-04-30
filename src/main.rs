// Password manager port from PwM cli
// Proof of concept. Not for real life usage yet.
// TODO: implement tries for passwords

use std::{env, io};
extern crate secstr;
use secstr::*;
use serde_json::to_writer_pretty;
use std::io::{stdin, Write, BufRead};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use core::str::from_utf8;
use base64::encode;
use argon2::{self}; 
use crate::color_write::write_yellow;
use crate::color_write::write_red;
use crate::color_write::write_cyan;
use crate::color_write::write_color;
use crate::validator::validate_password;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[path="./tests/aes_test.rs"]
mod aes_test;

#[path="./tests/argon_test.rs"]
mod argon_test;

#[path="./tests/json_test.rs"]
mod json_test;

#[path="./libs/argon_lib.rs"]
mod argon_lib;

#[path="./libs/aes_lib.rs"]
mod aes_lib;

#[path="./libs/json_lib.rs"]
mod json_lib;

#[path="./libs/password_validator.rs"]
mod validator;

#[path="./libs/color_write.rs"]
mod color_write;

const MAIN_SEPARTOR:&str = std::path::MAIN_SEPARATOR_STR;
const VAULT_DIR:&str = "Vaults";
const HELP_MESSAGE: &str = "\
PwM Copyright @ 2020-2024 0x078654c
PwM - A simple password manager to store localy the authentification data encrypted for a application using Rijndael AES-256 and Argon2 for password hash.
Contact: xcoding.dev@gmail.com



[x] Usage of Password Manager commands:
      -h       : Display this message.
      -createv : Create a new vault.
      -delv    : Deletes an existing vault.
      -listv   : Displays the current vaults.
      -addapp  : Adds a new application to vault.
      -dela    : Deletes an existing application in a vault.
      -updatea : Updates account's password for an application in a vault.
      -lista   : Displays the existing applications in a vault.
";


// Main function run.
fn main() {
    let args: Vec<String> = env::args().collect();

    // In case of no argument is passed to application.
    if args.len() == 1 {
        println!("{}","Use -h to list the parameters!\n");
        return;
    }

    // Read first parameter.
    let arg1 = &args[1];

    if arg1 == "-h" {
        println!("{}", HELP_MESSAGE);
    }

    if arg1 == "-createv" {
        create_vault();
    }

    if arg1 == "-delv" {
        delete_vaults();
    }

    if arg1 == "-listv" {
        list_vaults();
    }

    if arg1 == "-addapp" {
        add_applicaitons();
    }

    if arg1 == "-lista" {
        read_password();
    }

    if arg1 == "-dela"{
        delete_application();
    }

    if arg1 == "-updatea"{
        update_application();
    }
}

// Create vaults
fn create_vault(){
    let mut vault_name = String::new();
    let mut master_password1 = String::new();
    let mut master_password2 = String::new();

    let mut tries:i32 =0;
    println!("{}", "Enter vault name:");
    let _=stdin().read_line(&mut vault_name);
    let len = vault_name.len();
    vault_name.truncate(len-1);
    if vault_name.len() < 3 {
        write_yellow("Vault name must be at least 3 characters long!".to_string());
        return;
    }
    let current_exe = env::current_exe().unwrap();
    let current_path = get_current_exe_path(current_exe.as_path().display().to_string());
    let vault=format!("{}{}{}{}{}",current_path,VAULT_DIR,MAIN_SEPARTOR,vault_name.trim(),".x");
    let vault_exist_first: bool = Path::new(vault.as_str()).is_file();
    if vault_exist_first{
        let info ="Vault ".to_owned() + vault_name.trim() + " already exist!";
        write_yellow(info); 
        return;
    }

    loop {
        println!("{}", "Master Password: ");
        let _= io::stdin().read_line(&mut master_password1); // test only
        //master_password1 = rpassword::read_password().unwrap();

        println!("{}", "Confirm Master Password: ");
        let _= io::stdin().read_line(&mut master_password2); // test only
        //master_password2 =  rpassword::read_password().unwrap();
        let len2: usize = master_password2.len();
        let len1: usize = master_password1.len();
        master_password2.truncate(len2 - 1);
        master_password1.truncate(len1- 1); 
        
        if master_password1.trim() != master_password2.trim(){
            write_red("Passwords are not the same!".to_string());
        }

        if !validate_password(master_password2.clone()){
            write_yellow("Password must be at least 12 characters, and must include at least one upper case letter, one lower case letter, one numeric digit, one special character and no space!".to_string());
            tries+=1;
        }
        if tries > 2 {
            write_red("You have exceeded the number of tries!".to_string());
            return;
        }
        if  (master_password1.trim() != master_password2.trim()) || !validate_password(master_password2.clone()){
            master_password1=String::new();
            master_password2=String::new();
        }
        else{
            break;  
        }
    }  
    if master_password2.clone().len() < 1 {
            return;
    }
    let hash = argon_lib::argon_password_hash(&master_password2);
    let split:Vec<_> = hash.split('$').collect();
    let hash_split = split[5];
    let enc_hash = encode(hash_split);
    let enc_data = "1";
    let data = aes_lib::encrypt(enc_data.as_bytes(), &enc_hash);
    let dir_exist:bool = Path::new(&VAULT_DIR).is_dir(); 
    if !dir_exist{
        let _ =fs::create_dir(VAULT_DIR); 
    }
    let vault_exist: bool = Path::new(vault.as_str()).is_file();
    if !vault_exist{
        let mut file =  File::create(vault.to_string()).expect("File exist?");
        let _ = file.write_all(data.as_bytes());
        let info = "[+] Vault ".to_owned() + vault_name.trim() +" was created!";
        write_cyan(info,false);
    }else{
        let info = "[+] Vault ".to_owned() + vault_name.trim()+" already exists!";
        write_yellow(info);  
    }
}


// Delete vaults.
fn delete_vaults(){
    let mut vault_name = String::new();
    let mut master_password = String::new();
    let mut tries:i32 =0;
    println!("{}", "Enter vault name:");
    let _ = stdin().read_line(&mut vault_name);
    let current_exe = env::current_exe().unwrap();
    let current_path = get_current_exe_path(current_exe.as_path().display().to_string());
    let vault=format!("{}{}{}{}{}",current_path,VAULT_DIR,MAIN_SEPARTOR,vault_name.trim(),".x");
    let vault_exist_first: bool = Path::new(vault.as_str()).is_file();
    if !vault_exist_first{
        let inf:String = "Vault ".to_owned() + vault_name.trim()+" does not exist!";
        write_yellow(inf); 
        return;
    }
    let file = vault;
    loop{
        println!("{}", "Master Password: ");
        //master_password =rpassword::read_password().unwrap();
        let _= io::stdin().read_line(&mut master_password); // test only
        let len: usize = master_password.len();
        master_password.truncate(len-1); 
        if !validate_password(master_password.clone()){
        write_yellow("Password must be at least 12 characters, and must include at least one upper case letter, one lower case letter, one numeric digit, one special character and no space!".to_string());
            tries += 1;
            master_password=String::new();
        } else {
            break;
        }

        if tries>2 {
            write_yellow("You have exceeded the number of tries!".to_string());
            return;
        }
    }
	let decrypt_string = decrypt_vault(file.clone(),master_password); 
    if decrypt_string != "1" && !decrypt_string.contains("{") {
        write_red("Something went wrong. Check master password or vault name!".to_string());
    }else{
        fs::remove_file(file.clone()).expect("Vault already deleted?");
        let inf  = "[-] Vault ".to_owned()+vault_name.trim()+" was deleted!";
        write_yellow(inf);
    }
}

// Get executable path from current_exe
 fn get_current_exe_path(current_exe:String)->String{
    let cur_split:Vec<_> = current_exe.split(MAIN_SEPARTOR).collect();
    let cur_count = current_exe.split(MAIN_SEPARTOR).count();
    let mut cur_path = String::new();
    let mut count = cur_count;
    for splits in cur_split{
        count -=1 ;
        if count != 0{
            cur_path.push_str(format!("{}{}",splits,MAIN_SEPARTOR).as_str());
        }
    } 
    return cur_path;
 }

// List vaults.
fn list_vaults() {
    let current_exe = env::current_exe().expect("");
    let current_path = get_current_exe_path(current_exe.as_path().display().to_string());
    let vault = format!("{}{}",current_path,VAULT_DIR);
    let vault_exist_first: bool = Path::new(vault.as_str()).is_dir();
    if !vault_exist_first{
        write_yellow("There are no vaults created!".to_string()); 
        return;
    }
    println!("List of current vaults:");
    println!("----------------");
    let files_read= fs::read_dir(vault).unwrap();
    for file_vault in  files_read{
        let vault= file_vault.unwrap().path().as_path().display().to_string();
        let split_path:Vec<_> = vault.split(MAIN_SEPARTOR).collect();
        let  file_count =  vault.split(MAIN_SEPARTOR).count();
        let  file: &str = split_path[file_count - 1].as_ref();
        println!("{}", file.replace(".x", ""));
    }
    println!("----------------");
}

// Add new apllication to vault.
 fn add_applicaitons(){
    let mut tries:i32 =0;
    let mut vault_name = String::new();
    let mut master_password = String::new();
    let mut application = String::new();
    let mut account = String::new();
    let mut acc_password = String::new();
    let _count:i32 = 3;
    println!("{}", "Enter vault name:");
    let _ = stdin().read_line(&mut vault_name);
    let current_exe = env::current_exe().unwrap();
    let current_path = get_current_exe_path(current_exe.as_path().display().to_string());
    let vault=format!("{}{}{}{}{}",current_path,VAULT_DIR,MAIN_SEPARTOR,vault_name.trim(),".x");
    let vault_exist_first: bool = Path::new(vault.as_str()).is_file();
    if !vault_exist_first{
        let inf = "Vault ".to_owned()+vault_name.trim()+" does not exist!";
        write_yellow(inf);
        return;
    }

    loop{
        print!("{}","Enter master password for vault: ");
        write_cyan(vault_name.to_string(),true);
        //master_password =rpassword::read_password().unwrap();
        let _= io::stdin().read_line(&mut master_password); // test only
        let len: usize = master_password.len();
        master_password.truncate(len - 1); 
        if !validate_password(master_password.clone()){
            write_yellow("Password must be at least 12 characters, and must include at least one upper case letter, one lower case letter, one numeric digit, one special character and no space!".to_owned());
            tries += 1;
            master_password=String::new();
        } else {
            break;
        }

        if tries>2 {
            write_yellow("You have exceeded the number of tries!".to_string());
            return;
        }
    }

    let mut decrypt_string = decrypt_vault(vault.to_string(), master_password.clone());
    if decrypt_string != "1" && !decrypt_string.contains("{"){
        write_red("Something went wrong. Check master password or vault name!".to_string());
        return;
    }
    println!("{}", "Enter application name:");
    let _ = stdin().read_line(&mut application);
    let app  = String::from(application.trim());
    //TODO: make check exceded tries
    if app.trim().len() < 3{
        write_yellow("The length of application name should be at least 3 characters!".to_string());
        return;
    }

    print!("{}","Enter account name for ");
    write_color(app.to_string(),true, Color::Magenta);
    let _ = stdin().read_line(&mut account);
    let acc  = String::from(account.trim());
    //TODO: make check exceded tries
    if acc.trim().len() < 3{    
        write_yellow("The length of account name should be at least 3 characters!".to_string());
        return;
    }

    print!("{}", "Enter password for ");
    write_color(acc.to_string(),true, Color::Green);
    let _ = stdin().read_line(&mut acc_password);
    let pass  = String::from(acc_password.trim());

    //TODO: make check exceded tries
    if pass.trim().len() < 1{
        write_yellow( "Password should not be empty!".to_string());
        return;
    }
    let serialize_data= json_lib::json_serialize(app.to_string(), acc, pass);
    let data_added  =format!("{}{}","\r\n", serialize_data);
    decrypt_string.push_str(data_added.as_str());
    let password = master_password.clone();
    let hash = argon_lib::argon_password_hash(password.as_str());
    let split:Vec<_> = hash.split('$').collect();
    let hash_split = split[5];
    let enc_hash = encode(hash_split);
    let data =aes_lib::encrypt(decrypt_string.trim().as_bytes(), &enc_hash);
    if vault_exist_first {
        //TODO: store ecnrypted app in vault file.
        let mut file_open =  File::options().write(true).open(vault).unwrap();
        write!(file_open,"{}", data).unwrap();
        let inf = "Data for ".to_owned()+&app+" is encrypted and added to vault!";
        write_yellow(inf);
    }else{
        let inf  ="Vault ".to_owned()+&vault_name+" does not exist!"; 
        write_yellow(inf);  
    }

 } 

 // Read password from vault.
fn read_password(){
    let mut tries:i32 =0;
    let mut vault_name = String::new();
    let mut master_password = String::new();
    let mut application = String::new();
    println!("{}", "Enter vault name:");
    let _ = stdin().read_line(&mut vault_name);
    let current_exe = env::current_exe().unwrap();
    let current_path = get_current_exe_path(current_exe.as_path().display().to_string());
    let vault=format!("{}{}{}{}{}",current_path,VAULT_DIR,MAIN_SEPARTOR,vault_name.trim(),".x");
    let vault_exist_first: bool = Path::new(vault.as_str()).is_file();
    if !vault_exist_first{
        let inf = "Vault ".to_owned()+ vault_name.trim()+" does not exist!";
        write_yellow(inf);
        return;
    }
    loop{
        println!("{}", "Master Password: ");
        //master_password =rpassword::read_password().unwrap();
        let _= io::stdin().read_line(&mut master_password); // test only
        let len: usize = master_password.len();
        master_password.truncate(len-1); 
        if !validate_password(master_password.clone()){
            write_yellow("Password must be at least 12 characters, and must include at least one upper case letter, one lower case letter, one numeric digit, one special character and no space!".to_string());
            tries += 1;
            master_password=String::new();
        } else {
            break;
        }

        if tries>2 {
            write_yellow("You have exceeded the number of tries!".to_string());
            return;
        }
    }

    let decrypt_string = decrypt_vault(vault.to_string(), master_password);
    if decrypt_string != "1" && !decrypt_string.contains("{"){
        write_yellow( "Something went wrong. Check master password or vault name!".to_string());
        return;
    }
    println!("{}", "Enter application name (leave blank for all applications):");
    let _ = stdin().read_line(&mut application);
    let app  = String::from(application.trim());
    //TODO: make check exceeded tries
    if app.trim().len() > 0{
        println!("This is your decrypted data for {}:", app);
    }else{
        println!("This is your decrypted data for the entire vault:");
    }
    let decrypted_lines = decrypt_string.lines();
    for line in decrypted_lines{
        if line.len() >2 && line.contains(&app) {
            let deserialize = json_lib::json_deserialize(line);
            let split_deserialize:Vec<_> = deserialize.split("|").collect();
            println!("-------------------------");
            println!("Application Name:{}",split_deserialize[0]);
            println!("Account Name    :{}",split_deserialize[1]);
            println!("Password        :{}",split_deserialize[2]);
        }
    }
    println!("-------------------------");
}

fn delete_application(){
    let mut tries:i32 =0;
    let mut vault_name = String::new();
    let mut master_password = String::new();
    let mut application = String::new();
    let mut account = String::new();
    println!("{}", "Enter vault name:");
    let _ = stdin().read_line(&mut vault_name);
    let current_exe = env::current_exe().unwrap();
    let current_path = get_current_exe_path(current_exe.as_path().display().to_string());
    let vault=format!("{}{}{}{}{}",current_path,VAULT_DIR,MAIN_SEPARTOR,vault_name.trim(),".x");
    let vault_exist_first: bool = Path::new(vault.as_str()).is_file();
    if !vault_exist_first{
        println!("Vault {} does not exist!", vault_name.trim()); 
        return;
    }
    loop{
        println!("{}", "Master Password: ");
        //master_password =rpassword::read_password().unwrap();
        let _= io::stdin().read_line(&mut master_password); // test only
        let len: usize = master_password.len();
        master_password.truncate(len-1); 
        if !validate_password(master_password.clone()){
            write_yellow("Password must be at least 12 characters, and must include at least one upper case letter, one lower case letter, one numeric digit, one special character and no space!".to_string());
            tries += 1;
            master_password=String::new();
        } else {
            break;
        }

        if tries>2 {
            write_yellow("You have exceeded the number of tries!".to_string());
            return;
        }
    }

    let decrypt_string = decrypt_vault(vault.to_string(), master_password.clone());
    if decrypt_string != "1" && !decrypt_string.contains("{"){
         write_yellow("Something went wrong. Check master password or vault name!".to_string());
        return;
    }
    println!("{}", "Enter application name:");
    let _ = stdin().read_line(&mut application);
    let app  = String::from(application.trim());
    //TODO: make check exceeded tries
    if app.trim().len() == 0{
        println!("Application name should not be empty!");
        return;
    }
    if !decrypt_string.contains(&app){
        println!("Application {} does not exist!",app);
        return;
    }

    println!("Enter account name for {}:", app);
    let _ = stdin().read_line(&mut account);
    let acc  = String::from(account.trim());
    //TODO: make check exceded tries
    if acc.trim().len() < 3{
        write_yellow("Account name should not be empty!".to_string());
        return;
    }
    let decrypted_lines = decrypt_string.lines();
    let mut list_values:Vec<String>= Vec::new();
    let mut account_check:bool=false;
    for line in decrypted_lines{
        if line.len() > 2 {
        let deserialize = json_lib::json_deserialize(line);
        let split_deserialize:Vec<_> = deserialize.split("|").collect();
        if split_deserialize[0] != app || split_deserialize[1] != acc{
            list_values.push(line.to_string()+"\r\n");
        }else{
            account_check = true;
        }
      }
    }

    if !account_check{
        let inf = "Account ".to_owned()+&acc+" does not exist!";
        write_yellow(inf);
        return;
    }

    let add_vault = list_values.into_iter().collect::<String>();
    let final_vault = add_vault.trim(); 
    let hash = argon_lib::argon_password_hash(master_password.clone().as_str());
    let split:Vec<_> = hash.split('$').collect();
    let hash_split = split[5];
    let enc_hash = encode(hash_split);
    let data =aes_lib::encrypt(final_vault.as_bytes(), &enc_hash);
    if vault_exist_first {
        //TODO: store ecnrypted app in vault file.
        let  v = vault.clone();
        fs::remove_file(vault).expect("Vault already deleted?");
        let mut file =  File::create(v.to_string()).expect("File exist?");
        let _ = file.write_all(data.as_bytes());
        println!("[-] Account {} for {} was deleted!",acc,app);
    }else{
        let inf ="Vault ".to_owned()+&vault_name+" already exist!"; 
        write_yellow(inf);
    }
}


// Update applicaiton password.
fn update_application(){
    let mut tries:i32 =0;
    let mut vault_name = String::new();
    let mut master_password = String::new();
    let mut application = String::new();
    let mut account = String::new();
    let mut acc_password = String::new();

    println!("{}", "Enter vault name:");
    let _ = stdin().read_line(&mut vault_name);
    let current_exe = env::current_exe().unwrap();
    let current_path = get_current_exe_path(current_exe.as_path().display().to_string());
    let vault=format!("{}{}{}{}{}",current_path,VAULT_DIR,MAIN_SEPARTOR,vault_name.trim(),".x");
    let vault_exist_first: bool = Path::new(vault.as_str()).is_file();
    if !vault_exist_first{
        let inf =  "Vault ".to_owned()+vault_name.trim()+" does not exist!";
        write_yellow(inf);
        return;
    }

    loop{
        println!("{}", "Master Password: ");
        //master_password =rpassword::read_password().unwrap();
        let _= io::stdin().read_line(&mut master_password); // test only
        let len: usize = master_password.len();
        master_password.truncate(len-1); 
        if !validate_password(master_password.clone()){
            write_yellow("Password must be at least 12 characters, and must include at least one upper case letter, one lower case letter, one numeric digit, one special character and no space!".to_string());
            tries += 1;
            master_password=String::new();
        } else {
            break;
        }

        if tries>2 {
            write_yellow("You have exceeded the number of tries!".to_string());
            return;
        }
    }

    let decrypt_string = decrypt_vault(vault.to_string(), master_password.clone());
    if decrypt_string != "1" && !decrypt_string.contains("{"){
        write_red("Something went wrong. Check master password or vault name!".to_string());
        return;
    }
    println!("{}", "Enter application name:");
    let _ = stdin().read_line(&mut application);
    let app  = String::from(application.trim());

    if app.trim().len() == 0{
        write_yellow("Application name should not be empty!".to_string());
        return;
    }
    if !decrypt_string.contains(&app){
        let inf = "Application ".to_owned()+&app+" does not exist!";
        write_yellow(inf);
    }

    println!("Enter account name for {}:", app);
    let _ = stdin().read_line(&mut account);
    let acc  = String::from(account.trim());

    if acc.trim().len() < 3{
        write_yellow("Account name should not be empty!".to_string());
        return;
    }

    println!("Enter new password for {}:", acc);
    acc_password=rpassword::read_password().unwrap();
    let pass  = String::from(acc_password.trim());

    if pass.trim().len() < 1{
        write_yellow("Password should not be empty!".to_string());
        return;
    }

    let decrypted_lines = decrypt_string.lines();
    let mut list_values:Vec<String>= Vec::new();
    let mut account_check:bool=false;
    
    for line in decrypted_lines{
        if line.len() > 2 {
        let deserialize = json_lib::json_deserialize(line);
        let split_deserialize:Vec<_> = deserialize.split("|").collect();
        if split_deserialize[0] == app && split_deserialize[1] == acc{
            let serialize_data= json_lib::json_serialize(app.to_string(), acc.clone(), pass.clone());
            let data_added  =format!("{}", serialize_data);
            account_check=true;
            list_values.push(data_added+"\r\n");
        }else{
            list_values.push(line.to_string()+"\r\n");
        }
      }
    }

    if !account_check{
        let inf = "Account ".to_owned()+&acc+" does not exist!"; 
        write_yellow(inf);
        return;
    }

    let add_vault = list_values.into_iter().collect::<String>();
    let final_vault = add_vault.trim(); 
    let hash = argon_lib::argon_password_hash(master_password.clone().as_str());
    let split:Vec<_> = hash.split('$').collect();
    let hash_split = split[5];
    let enc_hash = encode(hash_split);
    let data =aes_lib::encrypt(final_vault.as_bytes(), &enc_hash);
    if vault_exist_first {
        let  v = vault.clone();
        fs::remove_file(vault).expect("Vault already deleted?");
        let mut file =  File::create(v.to_string()).expect("File exist?");
        let _ = file.write_all(data.as_bytes());
        println!("[*] Password for {} was updated!", acc);
    }else{
        let inf = "Vault ".to_owned()+&vault_name+" already exist!";
        write_yellow(inf);
    }
}

// Decrypt vaults.
// TODO: use secure string
fn decrypt_vault(vault_path:String, master_password:String)->String{
    let password = master_password.trim();
    let hash = argon_lib::argon_password_hash(password);
    let split:Vec<_> = hash.split('$').collect();
    let hash_split = split[5];
    let enc_hash = encode(hash_split);
    let mut file = File::open(vault_path).unwrap();
    let mut content = String::new();
    let _ =file.read_to_string(&mut content);
    let decrypted_bytes = aes_lib::decrypt(&content.as_str(), &enc_hash).unwrap();
    let decrypt_string = from_utf8(&decrypted_bytes).unwrap(); 
    return String::from(decrypt_string);
}