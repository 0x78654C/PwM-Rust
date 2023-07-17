use argon2::{self, Config, ThreadMode, Variant, Version};

pub fn argon_password_hash(password: &str) ->String  {
    let pass_substring = &password[2..12];
    let salt = pass_substring.as_bytes();
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 4096,
        time_cost: 40,
        lanes: 2,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    let hash = argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap();
    return hash;
}