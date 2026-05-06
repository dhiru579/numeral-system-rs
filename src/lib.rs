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

    fn get_base_and_value_post_base_update(
        base: u8,
        value: &String,
    ) -> Result<(u8, String), String> {
        // TODO : check if same new base as old then do nothing and return
        // TODO : check if base is not base-10 then convert to base-10
        check_if_base_allowed(&base)?;

        let inp: u32 = parse_string_to_int(value)?;

        let mut out_str: String = String::new();
        let mut temp: u32 = inp;
        while temp > 1_u32 {
            let digx: u32 = temp % base as u32;
            let charx: String = if base > 9 {
                match CHAR_DIGIT_MAP_DIGITS.iter().position(|&x| x == digx as u8) {
                    Some(x) => match CHAR_DIGIT_MAP_CHARS.get(x) {
                        Some(y) => y.to_string(),
                        None => digx.to_string(),
                    },
                    None => digx.to_string(),
                }
            } else {
                digx.to_string()
            };
            out_str.insert_str(0, &charx);
            temp /= base as u32;
        }
        if temp == 1_u32 {
            out_str.insert_str(0, &temp.to_string());
        }

        return Ok((base, out_str));
    }

    pub fn mutate_to_base_n(self: &mut NumberWithBase, base: u8) -> Result<(), String> {
        let (base, out_str) = Self::get_base_and_value_post_base_update(base, &self.value)?;

        self.base = base;
        self.value = out_str;
        Ok(())
    }

    pub fn convert_to_base_n(self: &NumberWithBase, base: u8) -> Result<NumberWithBase, String> {
        let (base, out_str) = Self::get_base_and_value_post_base_update(base, &self.value)?;

        Ok(NumberWithBase {
            base,
            value: out_str,
        })
    }
}

fn check_if_base_allowed(n: &u8) -> Result<(), String> {
    if !SUPPORTED_BASES.contains(n) {
        return Err("provided base is not supported".to_string());
    };
    Ok(())
}
