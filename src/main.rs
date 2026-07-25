use clap::Parser;
use numeral_system_rs::NumberWithBase;
mod output;
use output::{ConvertionResult, OutputType};

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
    
    /// type of the output
    #[arg(value_enum, default_value_t = OutputType::Verbose)]
    output_type: OutputType,
}

fn main() {
    let data = Args::parse();

    let num: Result<NumberWithBase, String> =
        NumberWithBase::from(data.input_base, data.input_value.clone());

    let mut conversion_res: ConvertionResult = ConvertionResult::new(data.input_base, data.input_value.clone());

    match num {
        Ok(mut num1) => match num1.mutate_to_base_n(data.output_base) {
            Ok(_) => {
                conversion_res.set_true(num1.get_base(), num1.get_value());
            }
            Err(strx) => {
                conversion_res.set_false(strx);
            }
        },
        Err(strx) => {
            conversion_res.set_false(strx);
        }
    }

    match data.output_type {
        OutputType::Verbose => {
            if conversion_res.get_status() {
                println!("status => PASS");
                println!(
                    "input  => value = \"{}\"\tbase = \"{}\"",
                    data.input_value, data.input_base
                );
                let (out_base, out_val) = conversion_res.get_output();
                println!(
                    "output => value = \"{}\"\tbase = \"{}\"", out_base, out_val
                );
            } else {
                println!("status => FAIL");
                println!("reason => {}", conversion_res.get_reason());
            }
        },
        _ => {
            println!("NOTE: this OutputType is yet be implemented");
            todo!();
        }
    }
}
