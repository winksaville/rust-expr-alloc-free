use std::error::Error;

use expr_alloc_free::Keys;

//use Keys;

fn main() -> Result<(), Box<dyn Error>> {
    let mut keys = Box::new(Keys::default());
    println!("&keys: {:p} ak: {:p} sk: {:p}", &keys, &keys.api_key, &keys.secret_key);

    keys.api_key = Some("api-key".to_string());
    keys.secret_key = Some("secret-key".to_string());
    let sk_bytes: &[u8] = &keys.get_sk_vec_u8_or_err()?;
    println!("sk_bytes: {:?} &sk_bytes[0]: {:p} [1]: {:p}", sk_bytes, &sk_bytes[0], &sk_bytes[1]);
    let ak_bytes: &[u8] = &keys.get_ak_vec_u8_or_err()?;
    println!("ak_bytes: {:?} &ak_bytes[0]: {:p} [1]: {:p}", ak_bytes, &ak_bytes[0], &ak_bytes[1]);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let keys = Keys {
            api_key: Some("api_key".to_string()),
            secret_key: Some("secret_key".to_string()),
        };

        assert_eq!(keys.get_ak_or_err().unwrap(), "api_key");
        assert_eq!(keys.get_sk_or_err().unwrap(), "secret_key");
    }
}