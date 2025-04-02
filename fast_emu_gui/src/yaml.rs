use serde_yaml::Value;
use std::io::Error as IOError;
use std::{fs::File, io::Read};

use crate::{EmuData, FastEmuGUIError};

pub fn read_yaml_file_into_emu_data(file_path: &std::path::Path) -> EmuData {
    match read_file_to_string(file_path).and_then(|yaml_string| get_emu_data_from_yaml_string(yaml_string)) {
        Ok(emu_data) => emu_data,
        Err(error) => {
            println!("{error}");
            println!("Failed to get data from yaml. Initializing with non data.");
            EmuData::default()
        },
    }
}

fn get_emu_data_from_yaml_string(file_contents: String) -> Result<EmuData, FastEmuGUIError> {
    let data: Value = serde_yaml::from_str(&file_contents)?;
    println!("{:#?}", data);

    todo!()
}

fn read_file_to_string(file_path: &std::path::Path) -> Result<String, FastEmuGUIError> {
    let mut file = File::open(file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

#[cfg(test)]
mod test {
    use crate::emu_data::EmuData;

    use super::get_emu_data_from_yaml_string;

    const TEST_YAML: &str = "
        Name: My Emulator GUI
        ClockFrequency: 4.194304 MHz
        Registers: 
        GeneralPurpose:
            - R1: 20
            - R2: 0
            - R3: 
            - R4: 200
        Control:
            - SP: 256
            - PC: 0
        Instructions:
        - NOP: 1
        - LD [HL]: 3
        - SRA A: 2
    ";

    #[test]
    fn test_reading_yaml() {
        let _data: EmuData = get_emu_data_from_yaml_string(TEST_YAML.into()).unwrap();
    }
}
