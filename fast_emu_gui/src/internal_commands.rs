pub enum InternalCommand {
    UpdateRegisterValue {
        group_name: String,
        register_name: String,
        value: u64,
    },
}
