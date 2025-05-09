#include "fast_emu_gui.h"
#include <chrono>
#include <iostream>
#include <thread>

void emulator_loop() {
	uint8_t data[40000] = {255};
	std::fill(data, data + 40000, 255);
	for (int i = 0; i < 0x10; i++) {
		for (int j = 0; j < 0x1000; j++) {
			update_register_value("General Purpose", "R1", 0x1000 + j);
			update_register_value("General Purpose", "R2", j);
			std::this_thread::sleep_for(std::chrono::microseconds(10));
			// update_frame_buffer(data, 40000);
		}
		update_register_format("General Purpose", "R1", static_cast<DisplayFormat>(i % 4));
		update_register_format("General Purpose", "R2", static_cast<DisplayFormat>(i % 4));
	}
	std::this_thread::sleep_for(std::chrono::microseconds(100));
}

int main() {
	std::thread emu_thread(emulator_loop);
	start_fast_emu_gui();
	emu_thread.join();
	return 0;
}
