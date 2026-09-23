# CHIP-8 Emulator

A fully functional CHIP-8 emulator written in Rust with graphics and sound support.

## What is CHIP-8?

CHIP-8 is an interpreted programming language developed in the 1970s for use on microcomputers. It was originally designed to make programming video games easier on these early systems. CHIP-8 programs are run on a CHIP-8 virtual machine, making it a popular choice for emulator development due to its simplicity.

### CHIP-8 Specifications:
- **Memory**: 4KB RAM
- **Display**: 64x32 pixel monochrome display
- **Registers**: 16 8-bit general purpose registers (V0-VF)
- **Stack**: 16 levels for subroutines
- **Timers**: 60Hz delay and sound timers
- **Input**: 16-key hexadecimal keypad
- **Font Set**: Built-in sprites for hexadecimal digits (0-F)

```rust
struct Emulator {
    pc: u16,
    i: u16,
    registers: [u8; 16],
    ram: [u8; 4096],
    stack: [u16; 64],
    stack_pointer: u8,
    buffer: [u8; 8 * 32],
    delay_timer: u8,
    sound_timer: u8,
}
```
## Features

This CHIP-8 emulator includes:

-  **Complete instruction set implementation** - All 35 CHIP-8 opcodes
-  **Graphics rendering** - 64x32 pixel display with 16x scaling
-  **Sound support** - Beep tone when sound timer is active
-  **Interactive ROM selection** - Choose from available ROM files via menu
-  **Proper timing** - 60Hz timer updates for accurate emulation
-  **Full keypad support** - 16-key hexadecimal input mapping
-  **Memory management** - 4KB RAM with proper font loading
-  **Stack operations** - Subroutine calls and returns

### Key Mapping

The original CHIP-8 keypad is mapped to your keyboard as follows:

```
CHIP-8 Keypad    Keyboard
1 2 3 C          1 2 3 4
4 5 6 D    =>    Q W E R
7 8 9 E          A S D F
A 0 B F          Z X C V
```

## Dependencies

This emulator uses the following Rust crates:

- **minifb** - Cross-platform window creation and pixel buffer rendering
- **rodio** - Cross-platform audio playback for sound effects
- **dialoguer** - Interactive command-line user interfaces for ROM selection
- **rand** - Random number generation for opcodes

## Installation and Usage

### Prerequisites

### Running the Emulator

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Nikhithasriram/chip-8-emulator
   cd chip8-emulator
   ```
   
2. **Add ROM files:**
   Place your CHIP-8 ROM files (`.ch8` files) in the `assets` folder:
   ```bash
   cp your-rom-file.ch8 assets/
   ```

3. **Run the emulator:**
   ```bash
   cargo run
   ```

4. **Select a ROM:**
   The emulator will display a menu of available ROM files. Use the arrow keys to select a ROM and press Enter to run it.


## Screenshots
### Chose the rom file
<img width="935" height="439" alt="image" src="https://github.com/user-attachments/assets/dbb4f3da-a129-45be-8de1-086261c331e5" />

### IBM Logo
<img width="1024" height="528" alt="image" src="https://github.com/user-attachments/assets/1b2e2050-2f9e-4c22-9693-21ab2cf75c28" />

### Pong Game
<img width="1029" height="532" alt="image" src="https://github.com/user-attachments/assets/af395900-7ef4-48e6-a44f-1997d56c8259" />

### Tetris
<img width="1024" height="522" alt="image" src="https://github.com/user-attachments/assets/4dd530e8-e593-4716-9f0c-b84ad103eb55" />





