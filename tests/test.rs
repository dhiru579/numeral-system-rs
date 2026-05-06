use parameterized::parameterized;

use numeral_system_rs::NumberWithBase;

#[parameterized(
    test_case={
        // ()
        (1, "10", "provided base is not supported"),
        (2, "10", "1010"),
        (3, "10", "101"),
        (3, "11", "102"),
        (4, "10", "22"),
        (5, "10", "20"),
        (6, "10", "14"),
        (7, "10", "13"),
        (8, "10", "12"),
        (9, "10", "11"),
        (10, "10", "10"),
        (11, "10", "A"),
        (12, "11", "B"),
        (13, "12", "C"),
        (14, "13", "D"),
        (15, "14", "E"),
        (16, "15", "F"),
        (16, "1008732", "F645C"),
        (17, "16", "provided base is not supported"),
        (16, "abc", "Invalid value for the given base"),
        (2, "00_", "Invalid value for the given base"),
        (2, "00A", "Invalid value for the given base"),
        (2, "00a", "Invalid value for the given base"),
        (16, "G645C", "Invalid value for the given base"),
    }
)]
fn test_base10_data(test_case: (u8, &str, &str)) {
    let (out_base, inp_val, expected) = test_case;
    let cust_base_number2: Result<NumberWithBase, String> =
        NumberWithBase::from_base10(inp_val.to_string());
    match cust_base_number2 {
        Ok(numx) => {
            let newnum: Result<NumberWithBase, String> = numx.convert_to_base_n(out_base);
            match newnum {
                Err(x) => {
                    assert_eq!(expected, x);
                }
                Ok(numy) => {
                    assert_eq!(numy.get_base(), out_base);
                    assert_eq!(numy.get_value(), expected);
                }
            };
        }
        Err(my_str) => {
            assert_eq!(expected.to_string(), my_str);
        }
    }
}

#[parameterized(
    test_case={
        (10, "4567", 2, "1000111010111"),
        (12, "1B0", 8, "424"),
        (16, "1ff0", 13, "394C"),
    }
)]
fn test_conversion(test_case: (u8, &str, u8, &str)) {
    let (inp_base, inp_val, out_base, expected) = test_case;
    let cust_base_number: Result<NumberWithBase, String> =
        NumberWithBase::from(inp_base, inp_val.to_string());
    match cust_base_number {
        Ok(mut numx) => {
            match numx.mutate_to_base_n(out_base) {
                Err(x) => {
                    assert_eq!(expected, x);
                }
                Ok(_) => {
                    assert_eq!(numx.get_base(), out_base);
                    assert_eq!(numx.get_value(), expected);
                }
            };
        }
        Err(my_str) => {
            assert_eq!(expected.to_string(), my_str);
        }
    }
}
