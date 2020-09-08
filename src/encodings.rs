static FEISTAL_KEYS : [u32;10] = [24,91,72,33,12,5,68,35,90,75];
use std::convert::TryFrom;
use std::string::String;

pub fn ph_to_hex(ph: &String) -> u64 {
  let mut handle : Vec<char> = vec![];
  let chars : Vec<char> = ph.chars().collect();
  for c in chars {
    handle.push(encode_char(c))
  }
  let the_string : String = handle.into_iter().collect();
  return u64::from_str_radix(&the_string, 16).unwrap();
}

pub fn convert_64_to_32s(i:u64) -> (u32,u32) {
  let max_32_len = u32::MAX as u64;
  let right = i % max_32_len;
  let left = i / max_32_len;
  return ( u32::try_from(left).unwrap(), u32::try_from(right).unwrap())
}

pub fn convert_32s_to_64(left:u32, right: u32) -> u64 {
  let max_32_len = u32::MAX as u64;
  let mut return_val :u64 = (left as u64) * max_32_len;
  return_val = return_val + (right as u64);
  return return_val
}

pub fn feistal( left: u32, right: u32, forwards: bool ) -> (u32, u32) {
  let mut keys = FEISTAL_KEYS;
  if !forwards {
    keys.reverse();
  }
  let mut l : u32 = left;
  let mut r : u32 = right;
  for k in keys.iter() {
    let output = feistal_round(&l,&r, k);
    l = output.0;
    r = output.1;
  }
  return (r, l);
}

fn feistal_round( left:&u32, right:&u32, key:&u32 ) -> (u32, u32) {
  let new_left = right.clone();
  let new_right = left ^ (right.wrapping_mul(*key));
  return ( new_left, new_right)
}

fn encode_char(ph_digit:char) -> char {
  if ph_digit == '0' {
    return 'F';
  }
  if ph_digit.is_ascii_digit() {
    return ph_digit;
  }
  match ph_digit {
    '+' => 'A',
    '-' => 'B',
    ' ' => 'C',
    '(' => 'D',
    ')' => 'E',
    _ => panic!()
  }
}

fn decode_char(hex:char) -> char {
  if hex == 'F' {
    return '0';
  }
  if hex.is_ascii_digit() {
    return hex;
  }
  match hex {
    'A' => '+',
    'B' => '-',
    'C' => ' ',
    'D' => '(',
    'E' => ')',
    _ => panic!()
  }
}

pub fn hex_to_ph(num: u64) -> String {
  let hex_rep = format!("{:X}", num);
  let mut ph : Vec<char> = vec![];
  let chars : Vec<char> = hex_rep.chars().collect();
  for c in chars {
    ph.push(decode_char(c))
  }
  let the_string : String = ph.into_iter().collect();
  return the_string;
}