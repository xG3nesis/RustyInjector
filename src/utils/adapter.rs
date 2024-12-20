use bluer::Adapter;
use std::process::Command;

// Configures key features of the Bluetooth adapter to enable exploitation capabilities.
pub trait Configuration {
    fn disable_ssp(&self);
    fn enable_ssp(&self);
    fn set_name(&self, name: String);
    fn set_class(&self, class: String);
    fn set_address(&self, address: String);
}

impl Configuration for Adapter {
    // Changes the Bluetooth name of the adapter.
    fn set_name(&self, name: String) {
        let iface =  self.name();
        println!("Executing 'hciconfig {} name'.", iface);
        let _set_name = Command::new("hciconfig")
                .args([iface, "name" , &name])
                .output()
                .expect("Failed to set name !");
    }

    // Enables Secure Simple Pairing, required for pairing with the target device.
    fn enable_ssp(&self) {
        let iface =  self.name();
        println!("Executing 'btmgmt --index {} io-cap 1'.", iface);
        let io_cap = Command::new("btmgmt")
            .args(["--index", iface, "io-cap", "1"])
            .spawn()
            .expect("Failed to enable io capabilities !");
        
        io_cap.wait_with_output().unwrap();

        let btmgmt = Command::new("btmgmt")
            .args(["--index", iface, "ssp", "on"])
            .spawn()
            .expect("Failed to enable SSP !");

        btmgmt.wait_with_output().unwrap();
    }

    // Disables Secure Simple Pairing
    fn disable_ssp(&self) {
        let iface =  self.name();
        let btmgmt = Command::new("btmgmt")
            .args(["--index", iface, "ssp", "off"])
            .spawn()
            .expect("Failed to disable SSP !");

        btmgmt.wait_with_output().unwrap();
    }

    // Modifies the device class to impersonate a keyboard.
    fn set_class(&self, class: String) {
        let iface = self.name();
        if is_valid_class_format(&class) {
            let _set_class = Command::new("hciconfig")
                .args([iface, "class" , &class])
                .output()
                .expect("Failed to disable SSP !");

        } else {
            panic!("Unable to set specified class, aborting !");
        }        
    }

    // Modifies the Bluetooth address of the interface.
    fn set_address(&self, address: String) {
        let iface = self.name();
        let check_addr = super::helper::assert_addr(&address);
        match check_addr {
            Ok(addr) => {
                let addr = format!("{}", addr);
                let change_bdaddr = Command::new("bdaddr")
                    .args(["-i", iface , &addr])
                    .output()
                    .expect("Failed to modify bt_addr !");

                println!("{}", String::from_utf8_lossy(&change_bdaddr.stdout));
                println!("{}", String::from_utf8_lossy(&change_bdaddr.stderr));
            },
            Err(_) => panic!("Unable to customize bt_addr, aborting !"),
        }
    }

}

// Check if argument given to "set_class" is in correct format.
fn is_valid_class_format(input: &str) -> bool {
    let re = regex::Regex::new(r"^0x[0-9a-fA-F]{6}$").unwrap();
    re.is_match(input)
}


#[cfg(test)]
mod configuration_tests{
    use super::*;

    #[test]
    fn check_valid_class(){
        assert!(is_valid_class_format("0xA1B2C3"))
    }

    #[test]
    fn check_invalid_class_upper(){
        assert!(!is_valid_class_format("0x002540AB"))
    }

    #[test]
    fn check_invalid_class_lower(){
        assert!(!is_valid_class_format("0x0025"))
    }
}