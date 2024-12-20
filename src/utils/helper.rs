
use regex::Regex;
use bluer::Address;

// Function "assert_addr" used to assert the bluetooth target address is in the right format.
pub fn assert_addr(val: &str) -> Result<Address, String> {
    let re = Regex::new(r"^([0-9A-Fa-f]{2}(:[0-9A-Fa-f]{2}){5})$").unwrap();
    if re.is_match(val) {
        Ok(val.parse().unwrap())
    } else {
        Err(String::from(
            "Invalid Bluetooth address format. Expected format in hexadecimal : XX:XX:XX:XX:XX:XX !",
        ))
    }
}

#[cfg(test)]
mod check_addr_tests {
    use super::*;

    #[test]
    fn check_nice_addr_format(){
        let addr = Address::from([0xA1, 0xB1, 0xC1, 0xD1, 0xE1, 0xF1]);
        let result = assert_addr("A1:B1:C1:D1:E1:F1").unwrap();
        assert_eq!(result, addr);
    }

    #[test]
    #[should_panic]
    fn wrong_format_size_upper(){
        let _result = assert_addr("A1:B1:C1:D1:E1:F1:G1").unwrap();
    }

    #[test]
    #[should_panic]
    fn wrong_format_size_lower(){
        let _result = assert_addr("A1:B1:C1:D1").unwrap();
    }
}