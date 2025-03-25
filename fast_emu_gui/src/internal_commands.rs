use crate::DisplayFormat;

pub(crate) enum InternalCommand {
    UpdateRegisterValue {
        group_name: String,
        register_name: String,
        value: u64,
    },
    UpdateRegisterFormat {
        group_name: String,
        register_name: String,
        new_format: DisplayFormat,
    },
}
