pub mod cpu;
pub mod ppu;
pub mod address;
pub mod interconnect;

// pub struct Nes {
//     cpu: Cpu,
//     interconnect: Interconnect,

//     rom_header: [u8; 16],
//     cycles: usize,
// }

// impl Nes {
//     pub fn new(rom_buffer: Vec<u8>) -> Nes {
//         let header = &rom_buffer[0..16];
//         if !validate_header(header) {
//             panic!("header validation failed: {:?}", &header);
//         }

//         // TODO const these
//         let bank0_start = 16;
//         let bank0_end = 16 * 1024 + bank0_start;
//         let bank1_start = bank0_end;
//         let bank1_end = 16 * 1024 + bank1_start;

//         let memory = if rom_is_double_banked(header) {
//             println!("ROM is double banked");
//             let bank0 = address::Bank::new(&rom_buffer[bank0_start..bank0_end]);
//             let bank1 = address::Bank::new(&rom_buffer[bank1_start..bank1_end]);
//             address::AddressSpace::new_double_bank(bank0, bank1)
//         } else {
//             println!("ROM is single banked");
//             let bank = address::Bank::new(&rom_buffer[bank0_start..bank0_end]);
//             address::AddressSpace::new_single_bank(bank)
//         };

//         Nes {
//             cpu: cpu::Cpu::new(memory),

//             rom_header: header,
//             cycles: 0,
//         }
//     }

//     pub fn start(&mut self) {
//     }
// }
