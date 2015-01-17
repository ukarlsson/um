#![feature(box_syntax)]
#![allow(unstable)]

extern crate arena;

use std::os;
use std::io::File;
use std::io::IoErrorKind::EndOfFile;

fn main() {
  let args: Vec<String> = os::args();

  let mut registers = [0u32; 8];

  let mut arrays : Vec<Vec<u32>> = vec!();

  let mut program = File::open(&Path::new(args[1].clone()));

  let mut array0 = vec!();

  loop {
    match program.read_be_u32() {
      Err(why) => match why.kind {
        EndOfFile => break,
        _ => panic!("! {:?}", why.kind),

      },
      Ok(n) => array0.push(n)
    }
  }

  arrays.push(array0);

  let mut finger = 0u32;

  loop {
    let instruction: u32 = arrays[0][finger as usize];

    let op = instruction >> 28;
    let c = ((instruction >> 0) & 0b111u32) as usize;
    let b = ((instruction >> 3) & 0b111u32) as usize;
    let a = ((instruction >> 6) & 0b111u32) as usize;

    // println!("F {} O {} A {} B {} C {}", finger, op, a, b, c);

    finger += 1;

    match op {
      0u32 => if registers[c] != 0 { registers[a] = registers[b] },

      1u32 => registers[a] = arrays[registers[b] as usize][registers[c] as usize],

      2u32 => arrays[registers[a] as usize][registers[b] as usize] = registers[c],

      3u32 => registers[a] = registers[b] + registers[c],

      4u32 => registers[a] = registers[b] * registers[c],

      5u32 => registers[a] = registers[b] / registers[c],

      6u32 => registers[a] = !(registers[b] & registers[c]),

      7u32 => break,

      8u32 => {
        let mut array = vec!();
        array.resize(registers[c] as usize, 0u32);
        registers[b] = arrays.len() as u32;
        arrays.push(array);
      }

      9u32 => arrays[registers[c] as usize] = vec!(),

      10u32 => 
        match std::io::stdout().write_u8(registers[c] as u8) {
          Err(why) => panic!("! {:?}", why.kind),
          Ok(_) => ()
        },

      11u32 =>
        registers[c] = match std::io::stdin().read_u8() {
          Err(why) => match why.kind {
            EndOfFile => !0u32,
            _ => panic!("! {:?}", why.kind),

          },
          Ok(n) => n as u32,
        },

      12u32 => {
        if registers[b] != 0 {
          arrays[0] = arrays[registers[b] as usize].clone();
        }
        finger = registers[c];
      },

      13u32 => registers[((instruction >> 25) & 0b111u32) as usize] =
        instruction & 0b0000_0001_1111_1111_1111_1111_1111_1111u32,

      v => panic!("invalid instruction {:?}", v), 
    }
  }
  std::io::stdin().read_u8();
}
