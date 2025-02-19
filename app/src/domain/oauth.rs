/// Utility functions to generate URL and parameters of OAuth
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use sha2::{Digest, Sha256};
use web_sys::{js_sys, window};


pub const CLIENT_ID: &str = "03RBggyJxLjTXvX2eU9Uvk9d3WY0etriBficGGSO";
const REDIRECT_ROUTE: &str = "/login";
const PEERING_DB_AUTHORIZE_URL: &str = "https://auth.peeringdb.com/oauth2/authorize/";

pub fn build_oauth_url(code_verifier: &str, state: &str) -> String {
    let code_challenge = generate_code_challenge(code_verifier);
    let redirect_uri = get_redirect_uri();
    format!(
        "{PEERING_DB_AUTHORIZE_URL}?response_type=code&client_id={CLIENT_ID}&redirect_uri={redirect_uri}&state={state}&code_challenge={code_challenge}&code_challenge_method=S256"
    )
}

pub fn get_redirect_uri() -> String {
    let base_uri = get_base_uri().unwrap();
    format!("{base_uri}{REDIRECT_ROUTE}")
}

pub fn generate_code_verifier() -> String {
    // Generate a random code verifier with a length of 43-128 characters (recommended by PKCE spec)
    let length = (random() * (128.0 - 43.0) + 43.0).floor() as usize; // Generates an integer between 43 and 128
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";

    let mut verifier = String::new();
    for _ in 0..length {
        let random_index = (random() * charset.len() as f64) as usize;
        verifier.push(charset[random_index] as char);
    }

    verifier
}

pub fn generate_code_challenge(code_verifier: &str) -> String {
    // Create the SHA256 hash of the code verifier
    let mut hasher = Sha256::new();
    hasher.update(code_verifier);
    let hash = hasher.finalize();

    // Base64 URL-safe encode the hash (without padding)
    let challenge = URL_SAFE_NO_PAD.encode(hash);

    challenge
}

pub fn generate_random_state() -> String {
    // Define the length of the random state string (e.g., 32 characters)
    let state_length = 32;

    // Generate random bytes using JavaScript's Math.random()
    let mut random_bytes = vec![0u8; state_length];

    for byte in &mut random_bytes {
        *byte = (random() * 256.0) as u8; // Math.random() * 256 to get a byte value
    }

    // Convert the random bytes into a hexadecimal string
    random_bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}

fn get_base_uri() -> Option<String> {
    window().and_then(|win| win.location().origin().ok())
}

// Using JS random because the Rust rand crate version either enters in conflict with the Dioxus
// or the wasm implementation don't work well. In later versions of Dioxus the implementation
// of this function can be changed to the Rust rand crate.
fn random() -> f64 {
    js_sys::Math::random()
}
