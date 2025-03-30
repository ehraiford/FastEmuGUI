use std::collections::HashMap;

pub(crate) struct Register {
    pub value: u64,
    display_format: DisplayFormat,
    bit_width: u8,
    necessary_precision_for_format: u8,
}

impl Register {
    pub fn new(value: u64, display_format: DisplayFormat, bit_width: u8) -> Self {
        Self {
            value,
            necessary_precision_for_format: display_format.get_required_display_width(bit_width),
            display_format,
            bit_width,
        }
    }
    pub fn update_display_format(&mut self, new_format: DisplayFormat) {
        self.necessary_precision_for_format = new_format.get_required_display_width(self.bit_width);
        self.display_format = new_format;
    }
}

impl Default for Register {
    fn default() -> Self {
        Self {
            value: Default::default(),
            display_format: DisplayFormat::Hex,
            bit_width: 8,
            necessary_precision_for_format: 2,
        }
    }
}

pub(crate) struct RegisterSet {
    pub registers: HashMap<String, Register>,
}

impl RegisterSet {
    pub fn new(registers: HashMap<String, Register>) -> Self {
        Self { registers }
    }
    pub fn get_register_strings(&self) -> Vec<String> {
        self.registers
            .iter()
            .map(|(name, reg)| {
                format!(
                    "{name}: {}",
                    reg.display_format
                        .format_value(reg.value, reg.necessary_precision_for_format as usize)
                )
            })
            .collect()
    }
}

#[derive(Default)]
#[repr(C)]
pub enum DisplayFormat {
    #[default]
    Hex,
    Binary,
    Decimal,
    Octal,
}

impl DisplayFormat {
    pub fn format_value(&self, value: u64, display_precision: usize) -> String {
        match self {
            DisplayFormat::Hex => format!("0x{:0>width$x}", value, width = display_precision),
            DisplayFormat::Binary => format!("0b{:0>width$b}", value, width = display_precision),
            DisplayFormat::Decimal => format!("{:0>width$}", value, width = display_precision),
            DisplayFormat::Octal => format!("0o{:0>width$o}", value, width = display_precision),
        }
    }
    /// Returns the number of characters needed to display the max value of a register of the given width.
    pub fn get_required_display_width(&self, bit_width: u8) -> u8 {
        match self {
            DisplayFormat::Hex => bit_width.div_ceil(4),
            DisplayFormat::Binary => bit_width,
            DisplayFormat::Decimal => bit_width.div_ceil(3),
            DisplayFormat::Octal => bit_width.div_ceil(3),
        }
    }
}
