use anyhow::{bail, Result};
use clap::Parser;
use encryptor::password::generate_password;


// A simple password generator for any account

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]

struct Args{
    //Seed to generate password
    #[clap(short, long)]
    seed: String,

    //length of password
    #[clap(short, long, default_value_t=16)]
    length: usize,
}





fn main() -> Result<()> {
    let args = Args::parse();

   
    if args.seed.len() < 4 {
        bail!("Seed '{}' length must be >= 4", &args.seed);
    }

    let (seed, length) = (args.seed, args.length);

    let password = generate_password(&seed[..], length);

    match password {
        Ok(val) => println!("{}", val),
        Err(err) => println!("Error: {}", err),
    }

    Ok(())
}