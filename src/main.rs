use std::env;
use std::io::{stdin, stdout, Write};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use argon2::{self, Config, ThreadMode, Variant, Version};
mod aes_test;
mod argon_test;

#[path="./libs/argon_lib.rs"]
mod argon_lib;

#[path="./libs/aes_lib.rs"]
mod aes_lib;

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
    if master_password1.len() < 12{
        println!("{}", "Password must be at least 12 characters, and must include at least one upper case letter, one lower case letter, one numeric digit, one special character and no space!!");
        return;
    }
    println!("{}", "Confirm Master Password: ");
    let _=stdin().read_line(&mut master_password2);

     if master_password1 != master_password2{
        println!("{}", "Passwords are not the same!");
     }else{
        let hash = argon_lib::argon_password_hash(master_password1.as_str());
        let data = aes_lib::encrypt("".as_bytes(), hash.as_str());
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

// Check maximum  of tries. used in while loops for exit them at a certain count.
fn check_max_tries()->bool{
    let mut TRIES = Globals{number:0};
    if TRIES.number >= 3 {
        println!("You have exceeded the number of tries!");
        TRIES.up(TRIES.number*-1);
        return true;
    }
    return false;
}
