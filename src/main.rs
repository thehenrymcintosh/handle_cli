mod dict;
mod encodings;
use dotenv;

#[macro_use]
extern crate savefile_derive;

pub type Words = Vec<String>;

#[tokio::main]
async fn main() -> Result<(), dict::DictError> {
  dotenv::dotenv().ok();
  let args : Vec<String> = std::env::args().collect();
  let arg_len = args.len();
  if arg_len < 2 || args[1] == "help" {
    help();
  } else if args[1] == "encode" && arg_len == 3 {
    encode(&args[2]).await;
  } else if args[1] == "decode" && arg_len == 3 {
    decode(&args[2]).await;
  } else if args[1] == "update" {
    let update = dict::update().await;
    match update {
      Err(_) => println!("Error updating dict"),
      Ok(_) => println!("Update successful")
    }
  } else {
    help();
  }
  Ok(())
}

fn help() {
  println!("Handle Help");
  println!("to encode your phone number into a handle: 'encode YOUR_NUMBER_HERE'");
  println!("to decode your handle into a phone number: 'decode \"YOUR_HANDLE_HERE\"'");
  println!("to update the conversion dictionary 'update'");
}

async fn encode( phone_number: &String ) {
  // phone number to str
  let hex = encodings::ph_to_hex(phone_number);
  let broken_up = encodings::convert_64_to_32s(hex);
  let f1 = encodings::feistal(broken_up.0, broken_up.1, true);
  let encoded = encodings::convert_32s_to_64(f1.0, f1.1);
  let handle = dict::get_handle(encoded).await;
  println!("Encoding: {} => {}", phone_number, handle,);
} 

async fn decode( handle: &String ) {
  let decoded_handle = dict::get_number(handle.clone()).await;
  let split_decoded_handle = encodings::convert_64_to_32s(decoded_handle);
  let f2 = encodings::feistal(split_decoded_handle.0, split_decoded_handle.1, false);
  let combined = encodings::convert_32s_to_64(f2.0, f2.1);
  let phone_number = encodings::hex_to_ph(combined);
  println!("Decoding: {} => {}", handle, phone_number);
} 

