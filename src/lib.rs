use std::char::from_digit;

const SUPPORTED_BASE_MIN: u8 = 2;
const SUPPORTED_BASE_MAX: u8 = 32;

/// A number with base. (Supported Bases from 2 to 32)
/// 
/// `NumberWithBase` helps in converting numbers to different bases.
/// 
/// # Examples
///
/// You can create an Instance with [`NumberWithBase::from`] or [`NumberWithBase::from_base10`]:
///
/// ```
/// use numeral_system_rs::NumberWithBase;
/// 
/// let num0_res: Result<NumberWithBase, String> = NumberWithBase::from(10, String::from("128"));
/// let mut num0: NumberWithBase = num0_res.unwrap();
/// 
/// assert_eq!(num0.get_base(), 10);
/// assert_eq!(num0.get_value(), "128");
/// 
/// // mutate to base 12
/// num0.mutate_to_base_n(12);
/// 
/// assert_eq!(num0.get_base(), 12);
/// assert_eq!(num0.get_value(), "A8");
/// 
/// ```
///
#[derive(Debug)]
pub struct NumberWithBase {
    /// Base of the Number
    base: u8,
    /// Value of the Number
    value: String,
}

impl NumberWithBase {
    pub fn from(base: u8, value: String) -> Result<NumberWithBase, String> {
        check_if_base_allowed(&base)?;
        if !check_value_valid_for_base(&(base as u32), &value) {
            return Err("Invalid value for the given base".to_string());
        }
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

    fn get_base_and_value_post_base_update(
        base: u8,
        inp_base: &u8,
        value: &String,
    ) -> Result<(u8, String), String> {
        // if input & output base is same return early
        if base == *inp_base {
            return Ok((base, value.clone().to_ascii_uppercase()));
        }

        check_if_base_allowed(&base)?;

        let inp: u32 = get_base10_value(inp_base, value);

        let mut out_str: String = String::new();
        let mut temp: u32 = inp;
        while temp > 1_u32 {
            let digx: u32 = temp % base as u32;
            let charx: String = match from_digit(digx, base as u32) {
                Some(m) => m.to_string(),
                None => "0".to_string(),
            };
            out_str.insert_str(0, &charx.to_ascii_uppercase());
            temp /= base as u32;
        }
        if temp == 1_u32 {
            out_str.insert_str(0, &temp.to_string());
        }

        return Ok((base, out_str));
    }

    pub fn mutate_to_base_n(self: &mut NumberWithBase, base: u8) -> Result<(), String> {
        let (base, out_str) = Self::get_base_and_value_post_base_update(base, &self.base, &self.value)?;

        self.base = base;
        self.value = out_str;
        Ok(())
    }

    pub fn convert_to_base_n(self: &NumberWithBase, base: u8) -> Result<NumberWithBase, String> {
        let (base, out_str) = Self::get_base_and_value_post_base_update(base, &self.base, &self.value)?;

        Ok(NumberWithBase {
            base,
            value: out_str,
        })
    }
}

fn check_if_base_allowed(n: &u8) -> Result<(), String> {
    if *n < SUPPORTED_BASE_MIN || *n > SUPPORTED_BASE_MAX {
        return Err("provided base is not supported".to_string());
    };
    Ok(())
}

fn check_value_valid_for_base(base: &u32, value: &String) -> bool {
    for i in value.chars() {
        if i.to_digit(*base).is_none() {
            return false;
        }
    }
    true
}

fn get_base10_value(base: &u8, value: &String) -> u32 {
    let mut out_sum: u32 = 0;

    for (ind, chx) in value.chars().rev().enumerate() {
        let digit: u32 = match chx.to_digit(*base as u32) {
            Some(num) => num,
            None => 0_u32,
        };
        out_sum += digit * ((*base as u32).pow(ind as u32));
    }
    out_sum
}
