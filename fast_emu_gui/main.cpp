#include <iostream>

// Declare the Rust function (adjust library name for Windows)
extern "C" void start_debugger();
extern "C" int add(int, int);

int main() {
    start_debugger();
    return 0;
}
