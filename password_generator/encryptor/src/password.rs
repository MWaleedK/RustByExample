
use anyhow::{bail, Error, Result};
use base64::{engine::general_purpose, Engine as _};
use hash::merhash::mersenne_hash;

//cyprto(length=100) you can exchange add, delete characters

const CRYPTO: &str = "!pqHr$*+ST1Vst_uv:?wWS%X&Y-/Z01_2.34<ABl
12 9ECo|x#yDE^F{GHEI[]JK>LM#NOBWPQ:RaKU@}cde56R7=8f/9gIhi,jkzmn";

/// Password generator with hash value, use value's high
/// power to get character from CRYPTO

pub fn generate_password(seed: &str, length: usize) -> Result<String, Error>
{
    if length < 6{
        bail!("length must be >=6"); //return error
    }

    let p = match length{
        6..=10  =>1,
        11..=15 =>2,
        16..=20 =>3,
        _       =>4,
    };

    let mut mer_hash = mersenne_hash(seed).pow(p);

    //calculate password by mersenne_hash

    let mut password = String::new();
    let crypto_len = CRYPTO.len();
    while mer_hash > 9{
        let loc = mer_hash % crypto_len;
        let nthc = CRYPTO.chars().nth(loc).expect("Error while getting char!");
        password.push(nthc);
        mer_hash /= crypto_len;
    }

    //combine seed and password

    let interval = password.clone();
    for c in seed.chars(){
        password.push(c);
        password += &interval;

    }
    //encode password to bae64

    password = general_purpose::STANDARD.encode(password);
    password = password.replace("+", "*").replace("/", "*");

    //length is not enugh, use interval to fill

    let interval = password.clone();
    while password.len() < length{
        password += &interval;
    }
    Ok(format!("{}: {}", seed, &password[..length]))
}