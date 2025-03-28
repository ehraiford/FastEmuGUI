use crate::DisplayFormat;

pub(crate) enum InternalCommand<'a> {
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
    UpdateFrameBuffer {
        buffer: &'a [u8],
    },
}
