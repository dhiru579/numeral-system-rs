use clap::Parser;
use numeral_system_rs::NumberWithBase;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// base of the number
    input_base: u8,

    /// number value
    input_value: String,

    /// convert to base
    #[arg(short, default_value_t = 10_u8)]
    output_base: u8,
}

fn main() {
    let data = Args::parse();

    let num: Result<NumberWithBase, String> =
        NumberWithBase::from(data.input_base, data.input_value.clone());

    match num {
        Ok(mut num1) => match num1.mutate_to_base_n(data.output_base) {
            Ok(_) => {
                println!("status => PASS");
                println!(
                    "input  => value = \"{}\"\tbase = \"{}\"",
                    data.input_value, data.input_base
                );
                println!(
                    "output => value = \"{}\"\tbase = \"{}\"",
                    num1.get_value(),
                    num1.get_base()
                );
            }
            Err(strx) => {
                println!("status => FAIL");
                println!("reason => {}", strx);
            }
        },
        Err(strx) => {
            println!("status => FAIL");
            println!("reason => {}", strx);
        }
    }
}
