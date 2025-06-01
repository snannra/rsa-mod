use rug::{Integer, rand::RandState};
use std::io::{self, Write};

fn generate_large_prime(bits: u32, threshold: &Integer) -> Integer {
    let mut rand_state = RandState::new_mersenne_twister();

    loop {
        let p = Integer::from(Integer::random_bits(bits, &mut rand_state));
        if &p > threshold && p.is_probably_prime(25) != rug::integer::IsPrime::No {
            return p;
        }
    }
}

fn generate_keys(bits: u32) -> (Integer, Integer, Integer) {
    let threshold = Integer::from(1) << (bits - 1);
    let p = generate_large_prime(bits / 2, &threshold);
    let q = generate_large_prime(bits / 2, &threshold);
    let n = &p * &q;
    let phi = (Integer::from(&p) - 1) * (Integer::from(&q) - 1);

    let e = Integer::from(65537);
    let d = e
        .clone()
        .invert(&phi)
        .expect("e and phi(n) must be coprime");

    return (n.into(), e, d);
}

fn encrypt(m: &Integer, e: &Integer, n: &Integer) -> Integer {
    m.clone()
        .pow_mod(e, n)
        .expect("modular exponentiation failed")
}

fn decrypt(c: &Integer, d: &Integer, n: &Integer) -> Integer {
    c.clone()
        .pow_mod(d, n)
        .expect("modular exponentiation failed")
}

fn main() {
    let (n, e, d) = generate_keys(128);

    println!("Public Key: (n = {}, e = {})", n, e);
    println!("Private Key: d = {}", d);

    print!("Enter a number to encrypt: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let bytes = input.trim().as_bytes();
    let m = Integer::from_digits(bytes, rug::integer::Order::Lsf);

    if &m >= &n {
        panic!("Message too large for modulus");
    }

    let c = encrypt(&m, &e, &n);
    println!("Encrypted: {}", c);

    let decrypted = decrypt(&c, &d, &n);
    let decrypted_bytes = decrypted.to_digits(rug::integer::Order::Lsf);
    let decrypted_str = String::from_utf8(decrypted_bytes).expect("Invalid UTF-8");
    println!("Decrypted: {}", decrypted_str);
}
