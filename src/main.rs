// Adicione esta linha para corrigir os erros E0433 (fmt::Display, fmt::Formatter, etc.)
use std::fmt;

// Adicione esta linha para corrigir os erros E0425 (OsRng not found)
use rand::rngs::OsRng;

// Suas linhas de importação anteriores (exemplo):
use num_bigint::{BigInt, BigUint, RandBigInt, Sign};
use num_integer::Integer; 
use num_traits::{One, Zero}; 
use rand::Rng; 
// ... outros imports ...

// --- Helpers / types -------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub n: BigUint,
    pub e: BigUint,
}

#[derive(Debug, Clone)]
pub struct PrivateKey {
    pub n: BigUint,
    pub d: BigUint,
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey {{ n: {}, e: {} }}", self.n, self.e)
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PrivateKey {{ n: {}, d: {} }}", self.n, self.d)
    }
}

// --- Math utilities -------------------------------------------------------

/// Miller-Rabin probabilistic primality test.
/// rounds: number of bases to test (40 is common for good confidence).
fn is_probable_prime(n: &BigUint, rounds: usize) -> bool {
    if *n < BigUint::from(2u8) {
        return false;
    }
    let small_primes = [2u8, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    for p in small_primes {
        if &BigUint::from(p) == n {
            return true;
        }
        if n % p == Zero::zero() {
            return false;
        }
    }

    // write n-1 as 2^r * d with d odd
    let one = BigUint::one();
    let n_minus1 = n - &one;
    let mut d = n_minus1.clone();
    let mut r = 0u32;
    while &d % 2u8 == Zero::zero() {
        d /= 2u8;
        r += 1;
    }

    let mut rng = OsRng;
    for _ in 0..rounds {
        let a = rng.gen_biguint_range(&BigUint::from(2u8), &(n_minus1));
        let mut x = a.modpow(&d, n);
        if x == one || x == n_minus1 {
            continue;
        }
        let mut composite = true;
        for _ in 0..(r - 1) {
            x = x.modpow(&BigUint::from(2u8), n);
            if x == n_minus1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

/// Generate a random prime BigUint with approximate `bits` bits.
fn generate_prime(bits: usize) -> BigUint {
    let mut rng = OsRng;
    loop {
        // ensure the top bit is set so the number has desired bit length
        let mut p = rng.gen_biguint(bits as u64);
        
       p.set_bit((bits - 1) as u64, true);

       p.set_bit((bits - 1) as u64, true);
        // ensure odd
        p.set_bit(0, true);
        // quick small-prime check done in is_probable_prime
        if is_probable_prime(&p, 40) {
            return p;
        }
    }
}

/// Extended GCD for BigInt; returns (g, x, y) such that a*x + b*y = g = gcd(a,b)
fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        (a.clone(), BigInt::one(), BigInt::zero())
    } else {
        let (g, x1, y1) = extended_gcd(b, &(a % b));
        let x = y1.clone();
        let y = x1 - (a / b) * &y1;
        (g, x, y)
    }
}

/// Modular inverse of a mod m (returns Option<BigInt>)
fn modinv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _) = extended_gcd(a, m);
    if g != BigInt::one() && g != BigInt::from(1) {
        None
    } else {
        // ensure positive result
        let mut res = x % m;
        if res.sign() == Sign::Minus {
            res += m;
        }
        Some(res)
    }
}

// --- RSA key generation, encrypt, decrypt ---------------------------------

/// Generates RSA keypair with modulus size `bits` (total bits for n ~ bits).
/// Internally picks p and q of size bits/2.
pub fn generate_keys(bits: usize) -> (PublicKey, PrivateKey) {
    assert!(bits >= 64, "bits must be >= 64");

    // generate two primes p and q
    let half = bits / 2;
    let mut p = generate_prime(half);
    let mut q = generate_prime(bits - half);

    // ensure p != q
    while p == q {
        q = generate_prime(bits - half);
    }

    let n = &p * &q;
    let one = BigUint::one();
    let phi = (&p - &one) * (&q - &one);

    // common public exponent
    let e_default = BigUint::from(65537u32);
    let mut e = e_default.clone();
    if e.gcd(&phi) != One::one() {
        // fallback: pick random odd e until gcd(e, phi) == 1
        let mut rng = OsRng;
        loop {
            let candidate = rng.gen_biguint_range(&BigUint::from(3u8), &phi);
            if &candidate % 2u8 == One::one() && candidate.gcd(&phi) == One::one() {
                e = candidate;
                break;
            }
        }
    }

    // compute d = e^{-1} mod phi
    // convert to signed BigInt for extended_gcd
    let e_bi = BigInt::from_biguint(Sign::Plus, e.clone());
    let phi_bi = BigInt::from_biguint(Sign::Plus, phi.clone());
    let d_bi = modinv(&e_bi, &phi_bi).expect("modinv should exist");
    let d = d_bi.to_biguint().expect("positive d");

    (
        PublicKey { n: n.clone(), e: e.clone() },
        PrivateKey { n, d },
    )
}

/// Encrypt a message (bytes) using the public key. Returns ciphertext BigUint.
pub fn encrypt(pk: &PublicKey, message: &[u8]) -> BigUint {
    // convert message bytes to integer m
    let m = BigUint::from_bytes_be(message);
    if m >= pk.n {
        panic!("message too large for modulus; use padding/splitting");
    }
    m.modpow(&pk.e, &pk.n)
}

/// Decrypt ciphertext BigUint using private key. Returns message bytes.
pub fn decrypt(sk: &PrivateKey, ciphertext: &BigUint) -> Vec<u8> {
    let m = ciphertext.modpow(&sk.d, &sk.n);
    m.to_bytes_be()
}

// --- Example / simple CLI demo --------------------------------------------

fn main() {
    // choose key size in bits (for demo keep small like 512; for real usage use >= 2048)
    let bits = 512;
    println!("Gerando chaves RSA ({} bits)... isto pode demorar um pouco", bits);
    let (pk, sk) = generate_keys(bits);
    println!("Public key: n ({} bits), e = {}", pk.n.bits(), pk.e);
    println!("Private key: d ({} bits)", sk.d.bits());

    let message = b"Ola RSA em Rust!"; // plaintext as bytes
    println!("Mensagem original: {:?}", String::from_utf8_lossy(message));

    let ciphertext = encrypt(&pk, message);
    println!("Ciphertext (numérico): {}", ciphertext);

    let plaintext_bytes = decrypt(&sk, &ciphertext);
    println!("Mensagem decifrada: {:?}", String::from_utf8_lossy(&plaintext_bytes));

    // sanity check
    assert_eq!(message.to_vec(), plaintext_bytes);
    println!("Sucesso: mensagem recuperada com sucesso!");
}
