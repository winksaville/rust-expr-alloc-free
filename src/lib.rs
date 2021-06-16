use std::error::Error;

pub struct Keys {
    pub secret_key: Option<String>,
    pub api_key: Option<String>,
}

impl Default for Keys {
    #[inline(never)]
    fn default() -> Self {
        Keys {
            api_key: None,
            secret_key: None,
        }
    }
}

impl Keys {
    //#[inline(always)]
    #[inline(never)]
    pub fn get_ak_or_err(&self) -> Result<&str, Box<dyn Error>> {
        match &self.api_key {
            Some(ak) => Ok(ak.as_str()),
            None => Err("No api-key".into()),
        }
    }

    // So this is doing one alloc and one memcpy with the to_string
    // but the into_bytes just returns the Vec<u8> from the String so
    // no clone/alloc/free!
    //#[inline(always)]
    #[inline(never)]
    pub fn get_ak_vec_u8_or_err(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.get_ak_or_err()?.to_string().into_bytes())
    }

    //#[inline(always)]
    #[inline(never)]
    pub fn get_sk_or_err(&self) -> Result<&str, Box<dyn Error>> {
        match &self.secret_key {
            Some(sk) => Ok(sk.as_str()),
            None => Err("No secret-key".into()),
        }
    }

    // So this is doing one alloc and one memcpy with the to_string
    // but the into_bytes just returns the Vec<u8> from the String so
    // no clone/alloc/free!
    //#[inline(always)]
    #[inline(never)]
    pub fn get_sk_vec_u8_or_err(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.get_sk_or_err()?.to_string().into_bytes())
    }

    //#[inline(always)]
    //#[inline(never)]
    //pub fn get_sk_vec_u8_or_err(&self) -> Result<Vec<u8>, Box<dyn Error>> {
    //    // Each of these will end making 2 copies one by as_bytes
    //    // another copying into the vec. The interesting thing is that
    //    // all we want to do is read the contents of an immutable String
    //    // it would seem if we can coordinate the life time of the String
    //    // we shouldn't have to coyp it! I wonder if this is possible?

    //    //// This is much larger I assume much slower!
    //    //let sk = &self.get_sk_or_err()?;
    //    //let mut v_u8 = Vec::<u8>::with_capacity(sk.len() * std::mem::size_of::<char>());
    //    //for b in sk.bytes() {  // bytes() actaully calls as_bytes as currently implmented
    //    //    v_u8.push(b);
    //    //}
    //    //Ok(v_u8)
	
    //    //// As of rust 1.52.0 the asm:
    //    ////   $ cargo asm  --no-color --lib expr_alloc_free::Keys::get_sk_vec_u8_or_err > get_sk_vec_u8_or_err.four-lines.s
    //    //// is identical to the below code
    //    //let s = &self.get_sk_or_err()?; // No copy
    //    ////println!("sk s: {:p}", &s);
    //    //let b = s.as_bytes(); // Creates a copy of the strings bytes
    //    ////println!("sk b: {:p} &b[0]: {:p} &b[1]: {:p}", b, &b[0], &b[1]);
    //    //let v = b.to_vec(); // Copies the bytesinto_bytes in b to the v
    //    ////println!("sk v: {:p} &v[0]: {:p} &v[1]: {:p}", &v, &v[0], &v[1]);
    //    //Ok(v)

    //    //// As of rust 1.52.0 the asm:
    //    ////   $ cargo asm  --no-color --lib expr_alloc_free::Keys::get_sk_vec_u8_or_err > get_sk_vec_u8_or_err.one-line.s
    //    //// is identical to the above code
    //    //Ok(self.get_sk_or_err()?.as_bytes().to_vec())


    //    //// So this is doing only one copy, AFAICT!
    //    //let c = self.get_sk_or_err()?.clone();
    //    //let c_ptr = c.as_ptr();
    //    //println!("sk c_ptr: {:p}", c_ptr);
    //    //let b = c.into_bytes();
    //    //println!("sk &b: {:p} &b[0]: {:p} &b[1]: {:p}", &b, &b[0], &b[1]);
    //    //Ok(b)

    //    // So this is doing one alloca and one copy!
    //    Ok(self.get_sk_or_err()?.to_string().into_bytes())
    //}

}
