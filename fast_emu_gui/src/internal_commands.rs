use crate::{DisplayFormat, emu_data::Frequency, frame_buffer::MutexWrapper};

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
        mutex: MutexWrapper,
    },
    SetFrequency(Frequency),
}
