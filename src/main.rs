use dialoguer::Select;
use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use rodio::source::SineWave;
use rodio::{Sink, Source};
use std::time::{Duration, Instant};
use std::{fs, path::PathBuf, usize};
const HEIGHT: usize = 32;
const WIDTH: usize = 64;
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
const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
const KEY_MAP: [Key; 16] = [
    Key::X,    // 0 -> X
    Key::Key1, // 1 -> 1
    Key::Key2, // 2 -> 2
    Key::Key3, // 3 -> 3
    Key::Q,    // 4 -> Q
    Key::W,    // 5 -> W
    Key::E,    // 6 -> E
    Key::A,    // 7 -> A
    Key::S,    // 8 -> S
    Key::D,    // 9 -> D
    Key::Z,    // A -> Z
    Key::C,    // B -> C
    Key::Key4, // C -> 4
    Key::R,    // D -> R
    Key::F,    // E -> F
    Key::V,    // F -> V
];

// fn get_rom_file() -> PathBuf {
//     println!("Enter a the rom file path to run on the emulator: ");
//     let mut path_str = String::new();
//     io::stdin()
//         .read_line(&mut path_str)
//         .expect("Enter a file name present in the assets folder or the entier file path");
//     path_str = path_str.trim().to_string();
//     let path = PathBuf::from(&path_str);
//     if path.exists() {
//         if path.is_file() {
//             path
//         } else {
//             panic!("A file path is required");
//         }
//     } else {
//         let assets_path_str = format!("{}/assets/{}", env!("CARGO_MANIFEST_DIR"), path_str);
//         let assets_path = PathBuf::from(&assets_path_str);
//         if assets_path.exists() && assets_path.is_file() {
//             assets_path
//         } else {
//             panic!("Enter a valid path");
//         }
//     }
// }
fn select_rom_files() -> PathBuf {
    let assets_path_str = format!("{}/assets/", env!("CARGO_MANIFEST_DIR"));
    let assets_path = PathBuf::from(&assets_path_str);
    let mut filenames = Vec::new();
    let mut filepaths = Vec::new();
    if assets_path.exists() && assets_path.is_dir() {
        for file in fs::read_dir(assets_path).unwrap() {
            match file {
                Ok(f) => {
                    let path = f.path();
                    if path.is_file() {
                        filenames.push(
                            path.file_name()
                                .expect("File name needed")
                                .to_string_lossy()
                                .to_string(),
                        );
                        filepaths.push(path);
                    }
                }
                Err(e) => println!("Nothing exists {}", e),
            }
        }
    }
    let selection = Select::new()
        .with_prompt("Select a rom file to run and press enter")
        .items(&filenames)
        .default(0)
        .interact()
        .unwrap();
    println!("Running {}", filenames[selection]);
    return filepaths[selection].clone();
}
fn load_sprites(emulator: &mut Emulator) {
    for i in 0..FONT_SET.len() {
        emulator.ram[i] = FONT_SET[i];
    }
}
fn load_program(emulator: &mut Emulator) {
    //in sixth register i am setting it to 12
    //select_rom_files();
    let path = select_rom_files();
    let bytes = fs::read(path).expect("Unable to read the file");
    let mut counter: usize = emulator.pc as usize;
    for byte in bytes.iter() {
        emulator.ram[counter] = byte.clone();
        counter += 1;
    }
    //let instuction_set: [u16; 12] = [
    // 00 E0
    //    0x6000,
    //    0x6100,
    //    0xA300,
    //    0xD002,
    //    0xD002];

    //    0x00e0, 0x60f0, 0x6190, 0x6290, 0x6390, 0x64f0, 0x650C, 0x6608, 0xa300, 0xf455, 0xd565,0x1200,];
    // for i in &instuction_set {
    //     emulator.ram[counter] = ((i >> 8) & 0x00ff) as u8;
    //     emulator.ram[counter + 1] = (i & 0x00ff) as u8;
    //     counter += 2;
    // }
}
fn execute_opcode(emulator: &mut Emulator, window: &mut Window) {
    let left_opcode = emulator.ram[emulator.pc as usize] as u16;
    //emulator.pc = emulator.pc + 1;
    let right_opcode = emulator.ram[emulator.pc as usize + 1] as u16;
    emulator.pc = emulator.pc + 2;
    let opcode = left_opcode << 8 | right_opcode;
    //println!("Hello");
    //println!("{},{:X}", emulator.pc, opcode);
    let first_nibble = opcode >> 12;
    let second_nibble = (opcode >> 8) & 0x000f;
    let third_nibble = (opcode >> 4) & 0x000f;
    let fourth_nibble = opcode & 0x000f;
    match first_nibble {
        0 => {
            if second_nibble == 0 && third_nibble == 0xe && fourth_nibble == 0 {
                emulator.buffer = [0; 8 * 32];
            } else if second_nibble == 0 && third_nibble == 0xe && fourth_nibble == 0xe {
                emulator.stack_pointer -= 1;
                emulator.pc = emulator.stack[emulator.stack_pointer as usize];
            } else {
                println!("Yet to implment {}", opcode);
            }
        }
        1 => {
            let pc_value = (opcode & 0x0fff) as u16;
            emulator.pc = pc_value;
        }
        2 => {
            emulator.stack[emulator.stack_pointer as usize] = emulator.pc;
            emulator.pc = opcode & 0x0fff;
            emulator.stack_pointer += 1;
        }
        3 => {
            if emulator.registers[second_nibble as usize] == ((opcode & 0x00ff) as u8) {
                emulator.pc += 2;
            }
        }

        4 => {
            if emulator.registers[second_nibble as usize] != ((opcode & 0x00ff) as u8) {
                emulator.pc += 2;
            }
        }
        5 => {
            if emulator.registers[second_nibble as usize]
                == emulator.registers[third_nibble as usize]
            {
                emulator.pc += 2;
            }
        }
        6 => {
            let number = opcode & 0x00ff;
            emulator.registers[second_nibble as usize] = number as u8;
        }

        7 => {
            let number = opcode & 0x00ff;
            emulator.registers[second_nibble as usize] =
                emulator.registers[second_nibble as usize].wrapping_add(number as u8);
        }
        8 => {
            if fourth_nibble == 0 {
                emulator.registers[second_nibble as usize] =
                    emulator.registers[third_nibble as usize];
            } else if fourth_nibble == 1 {
                emulator.registers[second_nibble as usize] = emulator.registers
                    [second_nibble as usize]
                    | emulator.registers[third_nibble as usize];
            } else if fourth_nibble == 2 {
                emulator.registers[second_nibble as usize] = emulator.registers
                    [second_nibble as usize]
                    & emulator.registers[third_nibble as usize];
            } else if fourth_nibble == 3 {
                emulator.registers[second_nibble as usize] = emulator.registers
                    [second_nibble as usize]
                    ^ emulator.registers[third_nibble as usize];
            } else if fourth_nibble == 4 {
                let add_value: u16 = emulator.registers[second_nibble as usize] as u16
                    + emulator.registers[third_nibble as usize] as u16;
                if add_value > 255 {
                    emulator.registers[15] = 1;
                } else {
                    emulator.registers[15] = 0;
                }
                emulator.registers[second_nibble as usize] = (add_value & 0x00ff) as u8;
            } else if fourth_nibble == 5 {
                if emulator.registers[second_nibble as usize]
                    >= emulator.registers[third_nibble as usize]
                {
                    emulator.registers[15] = 1;
                } else {
                    emulator.registers[15] = 0;
                }
                emulator.registers[second_nibble as usize] = emulator.registers
                    [second_nibble as usize]
                    .wrapping_sub(emulator.registers[third_nibble as usize]);
            } else if fourth_nibble == 6 {
                let vx = emulator.registers[second_nibble as usize].clone();
                emulator.registers[second_nibble as usize] >>= 1;
                emulator.registers[15] = vx & 1;
            } else if fourth_nibble == 7 {
                emulator.registers[second_nibble as usize] = emulator.registers
                    [third_nibble as usize]
                    .wrapping_sub(emulator.registers[second_nibble as usize]);

                if emulator.registers[second_nibble as usize]
                    < emulator.registers[third_nibble as usize]
                {
                    emulator.registers[15] = 1;
                } else {
                    emulator.registers[15] = 0;
                }
            } else if fourth_nibble == 0xE {
                let vx = emulator.registers[second_nibble as usize].clone();
                emulator.registers[second_nibble as usize] <<= 1;
                emulator.registers[15] = vx >> 7 & 1;
            } else {
                println!("Yet to implment {}", opcode);
            }
        }
        9 => {
            if fourth_nibble == 0 {
                if emulator.registers[second_nibble as usize]
                    != emulator.registers[third_nibble as usize]
                {
                    emulator.pc += 2;
                }
            } else {
                println!("Yet to implment {}", opcode);
            }
        }
        10 => {
            emulator.i = opcode & 0x0fff;
        }
        11 => {
            emulator.pc = (emulator.registers[0] as u16).wrapping_add((opcode & 0x0fff) as u16);
            // println!("{},{}", emulator.registers[0], opcode & 0x0fff);
            // println!("{}", emulator.pc);
        }
        12 => {
            let rand_num: u8 = rand::rng().random_range(0..=255);
            emulator.registers[second_nibble as usize] = rand_num & ((opcode & 0x00ff) as u8);
        }
        13 => {
            emulator.registers[15] = 0;
            let x_cord = (emulator.registers[second_nibble as usize] as usize) % 64;
            let y_cord = (emulator.registers[third_nibble as usize] as usize) % 32;
            let byte_no = x_cord / 8;
            let bit_no = x_cord % 8;
            let x1 = byte_no;
            let x2 = byte_no + 1;
            for n in 0..fourth_nibble {
                // println!("{},{}", x1, x2);
                let sprite_value = emulator.ram[(emulator.i + n) as usize];
                let buffer_value1 = x1 + (y_cord + n as usize) * WIDTH / 8;
                if buffer_value1 < 256 {
                    let initial_value1 = emulator.buffer[buffer_value1];
                    emulator.buffer[buffer_value1] ^= sprite_value >> bit_no;

                    let final_value1 = emulator.buffer[buffer_value1];
                    emulator.registers[15] |= if initial_value1 & !final_value1 == 0 {
                        0
                    } else {
                        1
                    };
                }
                let buffer_value2 = x2 + (y_cord + n as usize) * WIDTH / 8;
                if buffer_value2 < 256 {
                    let initial_value2 = emulator.buffer[buffer_value2];
                    if bit_no != 0 {
                        emulator.buffer[buffer_value2] ^= sprite_value << (8 - bit_no);
                        let final_value2 = emulator.buffer[buffer_value2];
                        emulator.registers[15] |= if initial_value2 & !final_value2 == 0 {
                            0
                        } else {
                            1
                        };
                    }
                }
            }
        }
        14 => {
            if third_nibble == 9 && fourth_nibble == 0xe {
                if emulator.registers[second_nibble as usize] <= 0xf {
                    if window
                        .is_key_down(KEY_MAP[emulator.registers[second_nibble as usize] as usize])
                    {
                        emulator.pc += 2;
                    }
                }
            } else if third_nibble == 0xa && fourth_nibble == 1 {
                if emulator.registers[second_nibble as usize] <= 0xf {
                    if !window
                        .is_key_down(KEY_MAP[emulator.registers[second_nibble as usize] as usize])
                    {
                        emulator.pc += 2;
                    }
                }
            } else {
                println!("Yet to implment {}", opcode);
            }
        }
        15 => {
            if third_nibble == 5 && fourth_nibble == 5 {
                for k in 0..=second_nibble {
                    emulator.ram[emulator.i as usize + k as usize] = emulator.registers[k as usize]
                }
            } else if third_nibble == 3 && fourth_nibble == 3 {
                let mut value = emulator.registers[second_nibble as usize].clone();
                emulator.ram[emulator.i as usize] = value / 100 as u8;
                value = value % 100;
                emulator.ram[emulator.i as usize + 1] = value / 10 as u8;
                value = value % 10;
                emulator.ram[emulator.i as usize + 2] = value;
            } else if third_nibble == 6 && fourth_nibble == 5 {
                for k in 0..=second_nibble {
                    emulator.registers[k as usize] = emulator.ram[emulator.i as usize + k as usize];
                }
            } else if third_nibble == 1 && fourth_nibble == 0xe {
                emulator.i += emulator.registers[second_nibble as usize] as u16;
            } else if third_nibble == 2 && fourth_nibble == 9 {
                emulator.i = (emulator.registers[second_nibble as usize] as u16) * 5;
            } else if third_nibble == 0 && fourth_nibble == 7 {
                emulator.registers[second_nibble as usize] = emulator.delay_timer;
            } else if third_nibble == 1 && fourth_nibble == 5 {
                emulator.delay_timer = emulator.registers[second_nibble as usize];
            } else if third_nibble == 0 && fourth_nibble == 0xa {
                'wait_for_key: loop {
                    for (i, key) in KEY_MAP.iter().enumerate() {
                        window.update();
                        if window.is_key_down(*key) {
                            emulator.registers[second_nibble as usize] = i as u8;
                            break 'wait_for_key;
                        }
                    }
                }
            } else if third_nibble == 1 && fourth_nibble == 8 {
                emulator.sound_timer = emulator.registers[second_nibble as usize];
            } else {
                println!("Yet to implment {}", opcode);
            }
        }
        _ => {
            println!("Yet to implement");
        }
    }
}
// fn print_registers(emulator: &Emulator) {
//     for i in 0..16 {
//         println!("register {} :{} ", i, emulator.registers[i]);
//     }
// }
fn beep(sink: &mut Sink) {
    let source = SineWave::new(440.0)
        .take_duration(Duration::from_millis(50))
        .amplify(0.20);
    sink.append(source);
}
fn main() {
    let mut emulator = Emulator {
        pc: 0x200, //the pc starts incrementing from 512 bytes
        i: 0,
        registers: [0; 16],
        ram: [0; 4096],
        stack: [0; 64],
        stack_pointer: 0,
        buffer: [0; 8 * 32],
        delay_timer: 0,
        sound_timer: 0,
    };
    load_sprites(&mut emulator);
    load_program(&mut emulator);
    let stram_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("Not able to open default stream");
    let mut sink = rodio::Sink::connect_new(&stram_handle.mixer());

    let mut window = Window::new(
        "chip-8 emulator",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X16,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let mut display_buffer: Vec<u32> = vec![0; HEIGHT * WIDTH];
    let mut last_tick = Instant::now();
    while window.is_open() {
        let mut counter = 0;
        for i in &emulator.buffer {
            for j in 0..8 {
                let bit = (i >> (7 - j)) & 1;
                if bit == 1 {
                    display_buffer[counter] = 0xedc8f7;
                } else {
                    display_buffer[counter] = 0x000000;
                }
                counter += 1;
            }
        }
        window
            .update_with_buffer(&display_buffer, WIDTH, HEIGHT)
            .unwrap();
        execute_opcode(&mut emulator, &mut window);
        if last_tick.elapsed().as_millis() >= 16 {
            last_tick = Instant::now();
            if emulator.delay_timer > 0 {
                emulator.delay_timer -= 1;
            }
            if emulator.sound_timer > 0 {
                beep(&mut sink);
                emulator.sound_timer -= 1;
            }
        }

        //print_registers(&emulator);
    }
}
