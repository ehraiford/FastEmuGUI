extern "C"
{
    typedef enum
    {
        Hex = 0,
        Binary = 1,
        Decimal = 2,
        Octal = 3,
    } DisplayFormat;
}

extern "C" void start_fast_emu_gui();
extern "C" void update_register_value(const char *group_name, const char *register_name, uint64_t value);
extern "C" void update_register_format(const char *group_name, const char *register_name, DisplayFormat display_format);
