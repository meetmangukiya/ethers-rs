use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use ethers_contract_abigen::Abigen;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Name of the binding type
    #[clap(short, long)]
    _type: String,

    /// Sets a output file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    out: Option<PathBuf>,

    /// Path to ABI
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    abi: Option<PathBuf>,
}

fn read_stdin() -> Result<String, AbigenError> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        let res = stdin.read_line(&mut buffer);
        if !res.is_ok() {
            return Err(AbigenError);
        }

        if res.unwrap() == 0 {
            return Ok(buffer);
        }
    }
}

fn read_abi(args: &Cli) -> Result<String, AbigenError> {
    if args.abi == None {
        let abi = read_stdin();
        return abi;
    } else {
        let abi = std::fs::read_to_string(args.abi.as_ref().unwrap());
        if !abi.is_ok() {
            return Err(AbigenError);
        }
        return Ok(abi.unwrap());
    }
}

#[derive(Debug, Clone)]
struct AbigenError;

fn generate_abi(_type: String, abi: String) -> Result<String, AbigenError> {
    let abigen = Abigen::new_string(&_type, abi);
    if !abigen.is_ok() {
        return Err(AbigenError);
    }

    let bindings = abigen.unwrap().generate();
    if !bindings.is_ok() {
        return Err(AbigenError);
    }

    let tokens = bindings.unwrap().into_tokens();
    let unformatted = tokens.to_string();
    let formatted = ethers_contract_abigen::rustfmt::format(unformatted);
    if !formatted.is_ok() {
        return Err(AbigenError);
    }
    return Ok(formatted.unwrap());
}

fn main() -> Result<(), AbigenError> {
    let args = Cli::parse();
    let abi = read_abi(&args)?;
    let bindings = generate_abi(args._type, abi)?;

    if args.out != None {
        let fpath = args.out.unwrap();
        let file_create = fs::File::create(fpath);
        if !file_create.is_ok() {
            return Err(AbigenError);
        }
        let mut file = file_create.unwrap();
        let res = file.write(bindings.as_bytes());
        if !res.is_ok() {
            return Err(AbigenError);
        }
    } else {
        println!("{}", bindings);
    }
    return Ok(());
}
