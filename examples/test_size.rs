use sase_forge_verify::policy::LicensePolicy;
use sase_forge_verify::tier::LicenseTier;
use sase_forge_verify::MAX_POLICY_SIZE;
use postcard::to_allocvec;

fn main() {
    let policy = LicensePolicy::new(LicenseTier::Pro)
        .max_tokens(1000)
        .max_requests(100)
        .hw_bound(true)
        .expiry(2000000000);
    
    let postcard = to_allocvec(&policy).unwrap();
    println!("Serialized size: {} bytes", postcard.len());
    println!("MAX_POLICY_SIZE: {}", MAX_POLICY_SIZE);
    println!("Policy: {:?}", policy);
    println!("Serialized: {:?}", postcard);
}
