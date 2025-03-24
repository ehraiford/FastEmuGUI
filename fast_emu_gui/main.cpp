#include <iostream>
#include <thread>
#include <chrono>

extern "C" void start_fast_emu_gui();
extern "C" void update_register_value(char* name, uint64_t value);

void emulator_loop() {
    for (int i = 0; i < 1000000; i++) {
        update_register_value("R1", 0x1000 + i);
        update_register_value("R2", i);
        std::this_thread::sleep_for(std::chrono::microseconds(1)); 
    }
}

int main() {
    std::thread emu_thread(emulator_loop);
    start_fast_emu_gui(); 
    emu_thread.join(); 
    return 0;
}
