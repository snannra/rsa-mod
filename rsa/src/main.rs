use rand::thread_rng;
use rug::{Integer, rand::RandState};

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
    let phi = (&p - 1) * (&q - 1);

    let e = Integer::from(65537);
    let d = e.invert(&phi).expect("e and phi(n) must be coprime");

    return (n.into(),e,d);
}
fn main() {}
