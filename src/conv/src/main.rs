extern crate clap;
extern crate core;

use clap::Parser;

/// A simple little program to convert values between units.
///
/// This can be useful as a CLI tool but can also be integrated with things like
/// Alfred for example.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    from: String,
    to_unit: String,

    #[arg(short, long)]
    units: bool,
}

fn main() -> core::Result {
    let args = Args::parse();
    if args.units {
        println!("Available units");
        core::units().iter().for_each(|(k, v)| {
            println!("\n**{}:**", k);
            v.iter()
                .for_each(|u| println!("{} - {}", u.symbol, u.names[0]));
        });

        return core::Value::ok();
    }

    match args.from.parse() {
        Ok(v) => println!("{}", core::convert(v, &args.to_unit)?),
        Err(e) => println!("{}", e),
    }

    core::Value::ok()
}
