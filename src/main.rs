use std::env;
mod argon_test;




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
       
    let mut VAULT_DIR:String = "".to_string();
    let arg1 = &args[1];
    println!("{}", arg1);
}

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
