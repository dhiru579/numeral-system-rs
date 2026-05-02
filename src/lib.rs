const SUPPORTED_BASES: [u8; 15] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

const CHAR_DIGIT_MAP_CHARS: [char; 6] = ['A', 'B', 'C', 'D', 'E', 'F'];

const CHAR_DIGIT_MAP_DIGITS: [u8; 6] = [10, 11, 12, 13, 14, 15];

#[derive(Debug)]
pub struct NumberWithBase {
    base: u8,
    value: String,
}

impl NumberWithBase {
    pub fn from(base: u8, value: String) -> Result<NumberWithBase, String> {
        if !SUPPORTED_BASES.contains(&base) {
            return Err("base provided is not supported".to_string());
        }
        // TODO : check if value if correct for the base
        return Ok(NumberWithBase { base, value });
    }

    pub fn from_base10(value: String) -> Result<NumberWithBase, String> {
        NumberWithBase::from(10, value)
    }

    pub fn get_base(self: &NumberWithBase) -> u8 {
        self.base
    }

    pub fn get_value(self: &NumberWithBase) -> String {
        self.value.clone()
    }

    pub fn get_base_and_value(self: &NumberWithBase) -> (u8, String) {
        (self.get_base(), self.get_value())
    }
}

fn parse_string_to_int(val: &String) -> Result<u32, String> {
    let res: Result<u32, std::num::ParseIntError> = val.parse::<u32>();
    match res {
        Ok(val) => Ok(val),
        Err(_) => Err("Value is not a number".to_string()),
    }
}

fn check_if_base_allowed(n: &u8) -> Result<(), String> {
    if !SUPPORTED_BASES.contains(n) {
        return Err("provided base is not supported".to_string());
    };
    Ok(())
}
