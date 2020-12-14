use std::time::SystemTime;
use std::collections::HashMap;
use std::convert::TryFrom;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data14.txt")
}

pub fn run() {
    print_day(14);

    let start = SystemTime::now();

    // Let's do this...
    let res1 = run_decode(data(), DecodeMode::ContentsMask);
    let res2 = run_decode(data(), DecodeMode::MemoryMask);

    let timed = SystemTime::now().duration_since(start).unwrap();

    println!("Decoder with content masking -> {}", fmt_bright(&res1));
    println!("Decoder with scattering address masking -> {}", fmt_bright(&res2));
    
    print_duration(timed);
}

fn run_decode(prog: &str, mode: DecodeMode) -> u64{
    let mut decoder = Decoder::new(mode);
    for line in prog.lines() {
        decoder.process(line);
    }

    decoder.sums()
}

enum DecodeMode {
    ContentsMask,
    MemoryMask,
}

struct Decoder {
    mode: DecodeMode,
    mem: HashMap<u64, u64>,
    mask: [char; 36],
}

impl Decoder {
    fn new(mode: DecodeMode) -> Self {
        Decoder {
            mode,
            mem: HashMap::new(),
            mask: ['x'; 36],
        }
    }

    fn process(&mut self, command: &str) {
        match &command[0..3] {
            "mas" => self.process_mask(command),
            "mem" => self.process_write(command),
            _ => {},
        }
    }
    fn process_mask(&mut self, command: &str) {
        // command is like "mask = XXX10X1XX0X"
        for (i, c) in command[7..].chars().into_iter().enumerate() {
            self.mask[i] = c;
        }
    }
    
    fn process_write(&mut self, command: &str) {
        // Commands like : mem[7] = 101
        // Find the memory address.
        let split_command: Vec<_> = command.split(']').collect();
        let mem = &split_command[0][4..].parse::<u64>().unwrap();
        let decimal_val = &split_command[1][3..].parse::<u64>().unwrap();
        match self.mode {
            DecodeMode::ContentsMask => self.process_contents_mode(*mem, *decimal_val),
            DecodeMode::MemoryMask => self.process_memory_mode(*mem, *decimal_val),
        };
    }
    
    fn process_memory_mode(&mut self, mem: u64, decimal_val: u64) {
        // Need to transform memory address into multiple.
        let mem_bits: Vec<_> = format!("{:#036b}", mem).chars().collect();

        let mut addresses: Vec<u64> = vec![0]; // We always start with one address
        for (i, c) in self.mask.iter().enumerate() {
            let addition: u64 = 2_u64.pow(35_u32 - u32::try_from(i).unwrap());
            let mut next_addresses = vec![]; 
            match c {
                // If '1', add addition unconditionally.
                '1' => {
                    for address in &addresses {
                        next_addresses.push(*address + addition);
                    }
                },
                // If '0' *and* the mem bit is one, add it.
                '0' => {
                    if mem_bits[i] == '1' {
                        for address in &addresses {
                            next_addresses.push(*address + addition);
                        }
                    } else {
                        for address in &addresses {
                            next_addresses.push(*address);
                        }
                    }
                }
                // The interesting case we push both with and without.
                'X' | 'x' => {
                    for address in &addresses {
                        next_addresses.push(*address);
                        next_addresses.push(*address + addition);
                    }
                },
                _ => panic!("Unexpected mask"),
            };

            // Now copy the addresses.
            addresses = next_addresses;
        }


        // We should now have a whole heap o' addresses to write to.
        for address in addresses {
            self.mem.insert(address, decimal_val);
        }
    }

    fn process_contents_mode(&mut self, mem: u64, decimal_val: u64) {
        let val = format!("{:#036b}", decimal_val);

        // Need to mask the value.
        let mut new_val = 0;
        for (i, c) in val.chars().enumerate() {
            let addition: u64 = 2_u64.pow(35_u32 - u32::try_from(i).unwrap());
            let add_val = match self.mask[i] {
                '1' => addition,
                '0' => 0,
                'X' | 'x' => if c == '1' { addition } else { 0 },
                _ => panic!("Unexpected mask"),
            };
            new_val += add_val;
        }

        self.mem.insert(mem, new_val);
    }

    fn sums(&self) -> u64 {
        let mut sum = 0;
        for v in self.mem.values() {
            sum += v;
        }
        sum
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_small() {
        let data_small = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let data_small2 = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        assert_eq!(165, run_decode(data_small, DecodeMode::ContentsMask));
        assert_eq!(208, run_decode(data_small2, DecodeMode::MemoryMask));
    }

    #[test]
    fn test_all() {
        assert_eq!(4297467072083, run_decode(data(), DecodeMode::ContentsMask));
        assert_eq!(5030603328768, run_decode(data(), DecodeMode::MemoryMask));
    }
}
