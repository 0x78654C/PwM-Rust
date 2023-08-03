use std::env;
use std::fmt::Display;
use std::io::{stdin, stdout, Write, BufRead, BufReader};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use core::str::from_utf8;
use base64::encode;
use argon2::{self, Config, ThreadMode, Variant, Version};
mod aes_test;
mod argon_test;
mod json_test;

#[path="./libs/argon_lib.rs"]
mod argon_lib;

#[path="./libs/aes_lib.rs"]
mod aes_lib;

#[path="./libs/json_lib.rs"]
mod json_lib;

const VAULT_DIR:&str = "Vaults";
const HELP_MESSAGE: &str = "\
PwM Copyright @ 2020-2022 0x078654c
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


struct Globals {
    number: i32
}

impl Globals{
    fn up(&mut self, num: i32){
        self.number += num;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
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
}

// Create vaults
fn create_vault(){
    let mut vault_name = String::new();
    let mut master_password1 = String::new();
    let mut master_password2 = String::new();
    println!("{}", "Enter vault name:");
    let _=stdin().read_line(&mut vault_name);
    if vault_name.len() < 3{
        println!("{}", "Vault name must be at least 3 characters long!");
        return;
    }

    let vault=format!("{}{}{}{}{}",".\\",VAULT_DIR,"\\",vault_name.trim(),".x");
    let vault_exist_first: bool = Path::new(vault.as_str()).is_file();
    if vault_exist_first{
        println!("Vault {} already exist!", vault_name.trim()); 
        return;
    }

    println!("{}", "Master Password: ");
    let _=stdin().read_line(&mut master_password1);
    if master_password1.trim().len() < 12{
        println!("{}", "Password must be at least 12 characters, and must include at least one upper case letter, one lower case letter, one numeric digit, one special character and no space!!");
        return;
    }
    println!("{}", "Confirm Master Password: ");
    let _ = stdin().read_line(&mut master_password2);

     if master_password1.trim() != master_password2.trim(){
        println!("{}", "Passwords are not the same!");
     }else{
        let password = master_password1.trim();
        let hash = argon_lib::argon_password_hash(password);
        let split:Vec<_> = hash.split('$').collect();
        let hash_split = split[5];
        let enc_hash = encode(hash_split);
        let enc_data = "";
        let data = aes_lib::encrypt(enc_data.as_bytes(), &enc_hash);
        let dir_exist:bool = Path::new(&VAULT_DIR).is_dir(); 
        if !dir_exist{
            let _ =fs::create_dir(VAULT_DIR); 
        }
        let vault_exist: bool = Path::new(vault.as_str()).is_file();
        if !vault_exist{
            let mut file =  File::create(vault.to_string()).expect("File exist?");
            let _ = file.write_all(data.as_bytes());
            println!("Vault {} was created!", vault_name.trim());
        }else{
            println!("Vault {} already exist!", vault);  
        }
     }
}


// Delete vaults.
fn delete_vaults(){
    let mut vault_name = String::new();
    let mut master_password = String::new();
    println!("{}", "Enter vault name:");
    let _ = stdin().read_line(&mut vault_name);
    let current_exe = env::current_exe().unwrap();
    let current_path = get_current_exe_path(current_exe.as_path().display().to_string());
    let vault=format!("{}{}{}{}{}",current_path,VAULT_DIR,"\\",vault_name.trim(),".x");
    let vault_exist_first: bool = Path::new(vault.as_str()).is_file();
    if !vault_exist_first{
        println!("Vault {} does not exist!", vault_name.trim()); 
        return;
    }
    let mut file = vault;
    let data = fs::read_to_string(&mut file).expect("Something went wrong on read vault data!");
    println!("{}", "Master Password: ");
    let _=stdin().read_line(&mut master_password);
    let password = master_password.trim();
    if password.trim().len() < 12{
        println!("{}", "Password must be at least 12 characters, and must include at least one upper case letter, one lower case letter, one numeric digit, one special character and no space!!");
        return;
    }
    let hash = argon_lib::argon_password_hash(password);
    let split:Vec<_> = hash.split('$').collect();
    let hash_split = split[5];
    let enc_hash = encode(hash_split);
    let decrypted_bytes = aes_lib::decrypt(data.as_str(), &enc_hash).unwrap();
	let decrypt_string = from_utf8(&decrypted_bytes).unwrap(); 
    if decrypt_string != "" && !decrypt_string.contains("{"){
        println!("{}", "Something went wrong. Check master password or vault name!");
    }else{
        fs::remove_file(file).expect("Vault already deleted?");
        println!("Vault {} was deleted!", vault_name.trim().to_string());
    }
}

// Get executable path from current_exe
 fn get_current_exe_path(current_exe:String)->String{
    let cur_split:Vec<_> = current_exe.split("\\").collect();
    let cur_count = current_exe.split("\\").count();
    let mut cur_path = String::new();
    let mut count = cur_count;
    for splits in cur_split{
        count -=1 ;
        if count != 0{
            cur_path.push_str(format!("{}{}",splits,"\\").as_str());
        }
    } 
    return cur_path;
 }

// List vaults.
fn list_vaults() {
    let current_exe = env::current_exe().expect("");
    let current_path = get_current_exe_path(current_exe.as_path().display().to_string());
    let vault = format!("{}{}",current_path,VAULT_DIR);
    println!("List of current vaults:");
    println!("----------------");
    let files_read= fs::read_dir(vault).unwrap();
    for file_vault in  files_read{
        let vault= file_vault.unwrap().path().as_path().display().to_string();
        let split_path:Vec<_> = vault.split('\\').collect();
        let  file_count =  vault.split('\\').count();
        let  file: &str = split_path[file_count - 1].as_ref();
        println!("{}", file.replace(".x", ""));
    }
    println!("----------------");
}

// Add new apllication to vault.
 fn add_applicaitons(){
    let mut vault_name = String::new();
    let mut master_password = String::new();
    let mut application = String::new();
    let mut account = String::new();
    let mut acc_password = String::new();
    let mut count:i32 = 3;
    println!("{}", "Enter vault name:");
    let _ = stdin().read_line(&mut vault_name);
    let current_exe = env::current_exe().unwrap();
    let current_path = get_current_exe_path(current_exe.as_path().display().to_string());
    let vault=format!("{}{}{}{}{}",current_path,VAULT_DIR,"\\",vault_name.trim(),".x");
    let vault_exist_first: bool = Path::new(vault.as_str()).is_file();
    if !vault_exist_first{
        println!("Vault {} does not exist!", vault_name.trim()); 
        return;
    }
    println!("{}", "Master Password: ");
    let _=stdin().read_line(&mut master_password);
    let password = master_password.trim();
    if password.trim().len() < 12{
        println!("{}", "Password must be at least 12 characters, and must include at least one upper case letter, one lower case letter, one numeric digit, one special character and no space!!");
        return;
    }

    let mut decrypt_string = decrypt_vault(vault.to_string(), password.to_string());
    if decrypt_string != "" && !decrypt_string.contains("{"){
        println!("{}", "Something went wrong. Check master password or vault name!");
        return;
    }
    println!("{}", "Enter application name:");
    let _ = stdin().read_line(&mut application);
    let mut app  = String::from(application.trim());
    //TODO: make check exceded tries
    if app.trim().len() < 3{
        println!("{}", "The length of application name should be at least 3 characters!");
        return;
    }

    println!("Enter account name for {}:", app);
    let _ = stdin().read_line(&mut account);
    let mut acc  = String::from(account.trim());
    //TODO: make check exceded tries
    if acc.trim().len() < 3{
        println!("{}", "The length of account name should be at least 3 characters!");
        return;
    }

    println!("Enter password for {}:", acc);
    let _ = stdin().read_line(&mut acc_password);
    let mut pass  = String::from(acc_password.trim());

    //TODO: make check exceded tries
    if pass.trim().len() < 1{
        println!("{}", "Password should not be empty!");
        return;
    }

    let serialize_data= json_lib::json_serialize(app, acc, pass);
    let data_added  =format!("{}{}","\r\n", serialize_data);
    decrypt_string.push_str(data_added.as_str());
    let password = master_password.trim();
    let hash = argon_lib::argon_password_hash(password);
    let split:Vec<_> = hash.split('$').collect();
    let hash_split = split[5];
    let enc_hash = encode(hash_split);
    let data =aes_lib::encrypt(decrypt_string.as_bytes(), &enc_hash);
    if vault_exist_first {
        //TODO: store ecnrypted app in vault file.
        let mut file_open =  File::options().write(true).open(vault).unwrap();
        write!(file_open,"{}", data).unwrap();
        println!("Data for {} is encrypted and added to vault!", vault_name.trim());
    }else{
        println!("Vault {} already exist!", vault_name);  
    }

 } 

 // Read password from vault.
fn read_password(){
    let mut vault_name = String::new();
    let mut master_password = String::new();
    let mut application = String::new();
    let mut account = String::new();
    let mut acc_password = String::new();
    let mut count:i32 = 3;
    println!("{}", "Enter vault name:");
    let _ = stdin().read_line(&mut vault_name);
    let current_exe = env::current_exe().unwrap();
    let current_path = get_current_exe_path(current_exe.as_path().display().to_string());
    let vault=format!("{}{}{}{}{}",current_path,VAULT_DIR,"\\",vault_name.trim(),".x");
    let vault_exist_first: bool = Path::new(vault.as_str()).is_file();
    if !vault_exist_first{
        println!("Vault {} does not exist!", vault_name.trim()); 
        return;
    }
    println!("{}", "Master Password: ");
    let _=stdin().read_line(&mut master_password);
    let password = master_password.trim();
    if password.trim().len() < 12{
        println!("{}", "Password must be at least 12 characters, and must include at least one upper case letter, one lower case letter, one numeric digit, one special character and no space!!");
        return;
    }

    let mut decrypt_string = decrypt_vault(vault.to_string(), password.to_string());
    if decrypt_string != "" && !decrypt_string.contains("{"){
        println!("{}", "Something went wrong. Check master password or vault name!");
        return;
    }
    println!("{}", "Enter application name (leave blank for all applications):");
    let _ = stdin().read_line(&mut application);
    let mut app  = String::from(application.trim());
    //TODO: make check exceeded tries
    if app.trim().len() > 0{
        println!("This is your decrypted data for {}:", app);
    }else{
        println!("This is your decrypted data for the entire vault:");
    }
    let decrypted_lines = decrypt_string.lines();
    for line in decrypted_lines{
        if line.len() >0 && line.contains(&app) {
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

// Check maximum  of tries. used in while loops for exit them at a certain count.
fn check_max_tries()->bool{
    let mut TRIES = Globals{number:3};
    if TRIES.number >= 3 {
        println!("You have exceeded the number of tries!");
        TRIES.up(TRIES.number*-1);
        return true;
    }
    return false;
}
