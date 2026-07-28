use std::char::from_digit;

const SUPPORTED_BASE_MIN: u8 = 2;
const SUPPORTED_BASE_MAX: u8 = 32;

/// A number with base. (Supports Bases from 2 to 32)
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
/// // input base  -> 10
/// // input value -> 128
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
    /// create an instance of `NumberWithBase` with input base & value arguments.
    /// 
    /// # Examples
    ///
    /// ```
    /// use numeral_system_rs::NumberWithBase;
    /// 
    /// // input base  -> 24
    /// // input value -> 8J8
    /// let num0_res: Result<NumberWithBase, String> = NumberWithBase::from(24, String::from("8J8"));
    /// let mut num0: NumberWithBase = num0_res.unwrap();
    /// 
    /// assert_eq!(num0.get_base(), 24);
    /// assert_eq!(num0.get_value(), "8J8");
    /// ```
    ///
    pub fn from(base: u8, value: String) -> Result<NumberWithBase, String> {
        check_if_base_allowed(&base)?;
        if !check_value_valid_for_base(&(base as u32), &value) {
            return Err("Invalid value for the given base".to_string());
        }
        return Ok(NumberWithBase { base, value });
    }

    /// create an instance of `NumberWithBase` with input value arguments.
    /// (Input base is 10) 
    /// 
    /// # Examples
    ///
    /// ```
    /// use numeral_system_rs::NumberWithBase;
    /// 
    /// let num0_res: Result<NumberWithBase, String> = NumberWithBase::from_base10(String::from("48"));
    /// let mut num0: NumberWithBase = num0_res.unwrap();
    /// 
    /// assert_eq!(num0.get_base(), 10);
    /// assert_eq!(num0.get_value(), "48");
    /// ```
    ///
    pub fn from_base10(value: String) -> Result<NumberWithBase, String> {
        NumberWithBase::from(10, value)
    }

    /// fetches the base of the Number
    /// 
    /// # Examples
    ///
    /// ```
    /// use numeral_system_rs::NumberWithBase;
    /// 
    /// let num0_res: Result<NumberWithBase, String> = NumberWithBase::from(24, String::from("8J8"));
    /// 
    /// assert_eq!(num0_res.unwrap().get_base(), 24);
    /// ```
    ///
    pub fn get_base(self: &NumberWithBase) -> u8 {
        self.base
    }

    /// fetches the value of the Number
    /// 
    /// # Examples
    ///
    /// ```
    /// use numeral_system_rs::NumberWithBase;
    /// 
    /// let num0_res: Result<NumberWithBase, String> = NumberWithBase::from(24, String::from("8J8"));
    /// 
    /// assert_eq!(num0_res.unwrap().get_base(), 24);
    /// ```
    ///
    pub fn get_value(self: &NumberWithBase) -> String {
        self.value.clone()
    }

    /// fetches the base & value of the Number
    /// 
    /// # Examples
    ///
    /// ```
    /// use numeral_system_rs::NumberWithBase;
    /// 
    /// let num0_res: Result<NumberWithBase, String> = NumberWithBase::from(18, String::from("8A8"));
    /// 
    /// assert_eq!(num0_res.unwrap().get_base_and_value(), (18_u8, String::from("8A8")));
    /// ```
    ///
    pub fn get_base_and_value(self: &NumberWithBase) -> (u8, String) {
        (self.get_base(), self.get_value())
    }

    /// fetches the base & value of the Number after conversion
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

    /// mutates the current instance by conveting to output base provided.
    /// 
    /// # Examples
    ///
    /// ```
    /// use numeral_system_rs::NumberWithBase;
    /// 
    /// let num0_res: Result<NumberWithBase, String> = NumberWithBase::from(18, String::from("8HHFA8"));
    /// let mut num0: NumberWithBase = num0_res.unwrap();
    /// 
    /// num0.mutate_to_base_n(31);
    /// 
    /// assert_eq!(num0.get_base(), 31);
    /// assert_eq!(num0.get_value(), "ICPDU");
    /// ```
    ///
    pub fn mutate_to_base_n(self: &mut NumberWithBase, base: u8) -> Result<(), String> {
        let (base, out_str) = Self::get_base_and_value_post_base_update(base, &self.base, &self.value)?;

        self.base = base;
        self.value = out_str;
        Ok(())
    }

    /// new instance of the number by converting to output base provided.
    /// 
    /// # Examples
    ///
    /// ```
    /// use numeral_system_rs::NumberWithBase;
    /// 
    /// let num0_res: Result<NumberWithBase, String> = NumberWithBase::from(4, String::from("12321"));
    /// 
    /// let num0: NumberWithBase = num0_res.unwrap();
    /// let num1: NumberWithBase = num0.convert_to_base_n(27).unwrap();
    /// 
    /// assert_eq!(num0.get_base(), 4);
    /// assert_eq!(num0.get_value(), "12321");
    /// assert_eq!(num1.get_base(), 27);
    /// assert_eq!(num1.get_value(), "G9");
    /// ```
    ///
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
