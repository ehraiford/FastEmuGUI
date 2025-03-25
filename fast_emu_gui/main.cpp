#include <iostream>
#include <thread>
#include <chrono>
#include "fast_emu_gui.h"

void emulator_loop()
{
    for (int i = 0; i < 0x10; i++)
    {
        for (int j = 0; j < 0x1000; j++)
        {
            update_register_value("General Purpose", "R1", 0x1000 + j);
            update_register_value("General Purpose", "R2", j);
            std::this_thread::sleep_for(std::chrono::microseconds(1));
        }
        update_register_format("General Purpose", "R1", static_cast<DisplayFormat>(i % 4));
        update_register_format("General Purpose", "R2", static_cast<DisplayFormat>(i % 4));
    }
}

int main()
{
    std::thread emu_thread(emulator_loop);
    start_fast_emu_gui();
    emu_thread.join();
    return 0;
}
