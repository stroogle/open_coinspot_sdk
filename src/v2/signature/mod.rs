use std::error::Error;

use sha2::Sha512;
use hmac::{Hmac, Mac};
use hex;

pub fn sign(input: &str, key: &str) -> Result<String, Box<dyn Error>> {

    type HmacSha512 = Hmac<Sha512>;

    let mut mac = HmacSha512::new_from_slice(key.as_bytes())?;

    mac.update(input.as_bytes());

    let bytes = mac.finalize().into_bytes();

    let slice: &[u8] = bytes.as_slice();

    let string = hex::encode(slice);
    
    Ok(string)

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sign() {
        let res = sign("input2", "key").expect("Wait no...");
        println!("{}", res);
    }

}