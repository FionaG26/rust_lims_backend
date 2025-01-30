use rand::{distributions::Alphanumeric, Rng};

pub fn generate_secret_key() -> String {
    // Generate a random secret key using Alphanumeric characters
    let secret_key: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32) // Length of the secret key (you can adjust the length)
        .map(char::from)
        .collect();
    secret_key
}
