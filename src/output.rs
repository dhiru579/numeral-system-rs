use clap::ValueEnum;

#[derive(Debug)]
pub struct ConvertionResult {
    status: bool,
    reason: String,
    input: (String, String),
    output: (String, String),
}

impl ConvertionResult {
    pub fn new(base: u8, value: String) -> Self {
        ConvertionResult {
            status: false,
            reason: "-".to_string(),
            input: (base.to_string(), value),
            output: ("-".to_string(), "-".to_string()),
        }
    }

    pub fn set_true(self: &mut ConvertionResult, output_base: u8, output_val: String) {
        self.status = true;
        self.output = (output_base.to_string(), output_val);
    }

    pub fn set_false(self: &mut ConvertionResult, reason: String) {
        self.reason = reason;
    }

    pub fn get_status(self: &ConvertionResult) -> bool {
        self.status
    }

    pub fn get_pass_or_fail(self: &ConvertionResult) -> String {
        (if self.status { "PASS" } else { "FAIL" }).to_string()
    }

    pub fn get_outputs(self: &ConvertionResult) -> (String, String) {
        (self.output.0.to_string(), self.output.1.clone())
    }

    pub fn get_inputs(self: &ConvertionResult) -> (String, String) {
        (self.input.0.to_string(), self.input.1.clone())
    }

    pub fn get_reason(self: &ConvertionResult) -> String {
        self.reason.clone()
    }
}

trait OutputPrinter {
    fn print_console_output(_conversion_res: &ConvertionResult) {}
}

struct VerbosePrinter;

impl OutputPrinter for VerbosePrinter {
    fn print_console_output(conversion_res: &ConvertionResult) {
        let (inp_base, inp_val) = conversion_res.get_inputs();
        let (out_base, out_val) = conversion_res.get_outputs();

        println!("status => {}", conversion_res.get_pass_or_fail());
        if conversion_res.get_status() {
            println!("input  => base = \"{}\"\tvalue = \"{}\"", inp_base, inp_val);
            println!("output => base = \"{}\"\tvalue = \"{}\"", out_base, out_val);
        } else {
            println!("reason => {}", conversion_res.get_reason());
        }
    }
}

struct JsonPrinter;

impl OutputPrinter for JsonPrinter {
    fn print_console_output(conversion_res: &ConvertionResult) {
        let (inp_base, inp_val) = conversion_res.get_inputs();
        let (out_base, out_val) = conversion_res.get_outputs();

        println!("{{");
        println!("    \"status\": \"{}\",", conversion_res.get_pass_or_fail());
        println!("    \"is_success\": {},", conversion_res.get_status());
        println!("    \"reason\": \"{}\",", conversion_res.get_reason());
        println!("    \"input\": {{");
        println!("        \"base\": \"{}\",", inp_base);
        println!("        \"value\": \"{}\"", inp_val);
        println!("    }},");
        println!("    \"output\": {{");
        println!("        \"base\": \"{}\",", out_base);
        println!("        \"value\": \"{}\"", out_val);
        println!("    }}");
        println!("}}");
    }
}

struct CleanPrinter;

impl OutputPrinter for CleanPrinter {
    fn print_console_output(conversion_res: &ConvertionResult) {
        println!("{}", conversion_res.get_outputs().1);
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputType {
    Json,
    Verbose,
    Clean,
}

impl OutputType {
    pub fn print_console_output(&self, conversion_res: &ConvertionResult) {
        match self {
            OutputType::Verbose => VerbosePrinter::print_console_output(conversion_res),
            OutputType::Clean => CleanPrinter::print_console_output(conversion_res),
            OutputType::Json => JsonPrinter::print_console_output(conversion_res),
        };
    }
}
