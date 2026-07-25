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

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputType {
    Csv,
    Json,
    Verbose,
    Clean,
}
