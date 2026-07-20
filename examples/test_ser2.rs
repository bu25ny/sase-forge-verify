use sase_forge_verify::policy::LicensePolicy;
use sase_forge_verify::tier::LicenseTier;

fn main() {
    let policy = LicensePolicy::new(LicenseTier::Pro)
        .max_tokens(1000)
        .max_requests(100)
        .hw_bound(true)
        .expiry(2000000000);
    
    println!("Policy: {:?}", policy);
    println!("Features: {:?}", policy.features);
    
    match policy.to_postcard() {
        Ok(postcard) => {
            println!("Serialized size: {} bytes", postcard.len());
            println!("Serialized: {:?}", postcard);
            
            match LicensePolicy::from_postcard(&postcard) {
                Ok(decoded) => {
                    println!("Decoded: {:?}", decoded);
                    println!("Match: {}", policy == decoded);
                }
                Err(e) => println!("Decode error: {:?}", e),
            }
        }
        Err(e) => println!("Encode error: {:?}", e),
    }
    
    match policy.to_json() {
        Ok(json) => {
            println!("JSON: {}", json);
            match LicensePolicy::from_json(&json) {
                Ok(decoded) => println!("JSON decoded match: {}", policy == decoded),
                Err(e) => println!("JSON decode error: {:?}", e),
            }
        }
        Err(e) => println!("JSON encode error: {:?}", e),
    }
}
