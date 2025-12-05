# rust-example

a rust project where all the basics (possibly more) of Rust is run

can be used as a cheatsheet once finished (mostly)

## running the repo
```bash
    git clone https://github.com/saivishnu725/rust_examples svnu_rust
    cd svnu_rust
    cargo run
```

<ins>**NOTE:**</ins> Majority of the values are **hard-coded**, so open `src/main.rs` and change them to see different results for a clear understanding of the concept

## concepts covered (so far)

1. variables and mutability (`let mut a: i32 = 10;`)
2. arrays (`[i32; 5]`)
3. tuples (`(i32, f32, char, &str, String, bool)`)
4. strings (`String`)
5. string slice (`&str`)
6. loops
    * `for`
    * `while`
    * `loop`
7. error handling
    * `Result` enum for *recoverable* errors
    * `panic!` macro for *unrecoverable* errors
    * *passing* on the error from a fn to the fn call using `Result` enum -> `divide(u8, u8)`
    * *passing* on the error (technically speaking, its a lack of content) from a fn to the fn call using `Option` enum -> `is_os_even(i32)`
8. struct
9. enum

## license

[GNU General Public License v2.0](https://choosealicense.com/licenses/gpl-2.0/)