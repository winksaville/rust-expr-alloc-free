# Test allocation and free

See if I can "profile" alloc/free to get an insite on
what the compiler generates. So ATM I'm using cargo-asm
to "profile" and see the alloc/free.

This Keys code is from code that has "api keys" as a
key pair and for using the encryption libraries, HmacSha256,
and these need the signing key as a &[u8]. So I need
to convert String's to &[u8]. I had trouble getting it to
work and finally came up with returning a Vec<u8>.

Ideally, if we know that a String points to a Vec<u8> and
if the String is immutable we should just be able to "cast" the
pointer and length in the String to a Vec<u8>.

The next best solution I came up with is to use `String::into_bytes()` which
consumes the String so no copy is done compared to the `String::as_bytes()`. So
what I've done currently is that `get_xk_or_err()` return a &str so no copy
doing that:
```
    pub fn get_sk_or_err(&self) -> Result<&str, Box<dyn Error>> {
        match &self.secret_key {
            Some(sk) => Ok(sk.as_str()),
            None => Err("No secret-key".into()),
        }
    }
```
And the asm:
```
wink@3900x:~/prgs/rust/projects/expr-alloc-free (main)
$ cargo asm  --no-color --lib expr_alloc_free::Keys::get_sk_or_err
expr_alloc_free::Keys::get_sk_or_err:
 push    rbx
 mov     rbx, rdi
 mov     rax, qword, ptr, [rsi]
 test    rax, rax
 je      .LBB3_1
 mov     rcx, qword, ptr, [rsi, +, 16]
 mov     qword, ptr, [rbx, +, 8], rax
 mov     qword, ptr, [rbx, +, 16], rcx
 xor     eax, eax
 mov     qword, ptr, [rbx], rax
 mov     rax, rbx
 pop     rbx
 ret
.LBB3_1:
 lea     rdi, [rip, +, .L__unnamed_2]
 mov     esi, 13
 call    qword, ptr, [rip, +, _ZN3std5error115_$LT$impl$u20$core..convert..From$LT$$RF$str$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$GT$$GT$4from17h4da703b9e7d4d91bE@GOTPCREL]
 mov     qword, ptr, [rbx, +, 8], rax
 mov     qword, ptr, [rbx, +, 16], rdx
 mov     eax, 1
 mov     qword, ptr, [rbx], rax
 mov     rax, rbx
 pop     rbx
 ret
```

And `get_sk_vec_u8_or_err()` then uses `to_string()` on the
`&str` which does one `alloc` and a `memcpy` (i.e. a clone) to create a string and
then the `into_types()` "consumes" the `String` and returns it Vec<u8> so
no additional clone.
```
    #[inline(never)]
    pub fn get_sk_vec_u8_or_err(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.get_sk_or_err()?.to_string().into_bytes())
    }
```
And the asm:
```
wink@3900x:~/prgs/rust/projects/expr-alloc-free (main)
$ cargo asm  --no-color --lib expr_alloc_free::Keys::get_sk_vec_u8_or_err
expr_alloc_free::Keys::get_sk_vec_u8_or_err:
 push    r15
 push    r14
 push    r13
 push    r12
 push    rbx
 sub     rsp, 32
 mov     rbx, rdi
 lea     rdi, [rsp, +, 8]
 call    qword, ptr, [rip, +, _ZN15expr_alloc_free4Keys13get_sk_or_err17h611d6c1b41df8517E@GOTPCREL]
 mov     r15, qword, ptr, [rsp, +, 16]
 mov     r14, qword, ptr, [rsp, +, 24]
 cmp     qword, ptr, [rsp, +, 8], 1
 jne     .LBB4_1
 mov     qword, ptr, [rbx, +, 8], r15
 mov     qword, ptr, [rbx, +, 16], r14
 mov     eax, 1
 jmp     .LBB4_7
.LBB4_1:
 test    r14, r14
 je      .LBB4_2
 mov     esi, 1
 mov     rdi, r14
 call    qword, ptr, [rip, +, __rust_alloc@GOTPCREL]
 test    rax, rax
 je      .LBB4_8
 mov     r12, rax
 mov     r13, r14
 jmp     .LBB4_5
.LBB4_2:
 mov     r12d, 1
 xor     r13d, r13d
.LBB4_5:
 mov     rdi, r12
 mov     rsi, r15
 mov     rdx, r14
 call    qword, ptr, [rip, +, memcpy@GOTPCREL]
 mov     qword, ptr, [rbx, +, 8], r12
 mov     qword, ptr, [rbx, +, 16], r13
 mov     qword, ptr, [rbx, +, 24], r14
 xor     eax, eax
.LBB4_7:
 mov     qword, ptr, [rbx], rax
 mov     rax, rbx
 add     rsp, 32
 pop     rbx
 pop     r12
 pop     r13
 pop     r14
 pop     r15
 ret
.LBB4_8:
 mov     esi, 1
 mov     rdi, r14
 call    qword, ptr, [rip, +, _ZN5alloc5alloc18handle_alloc_error17h706aeb8ce5c22123E@GOTPCREL]
 ud2
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
