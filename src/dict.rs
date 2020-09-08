extern crate savefile;
use savefile::prelude::*;
use super::Words;
use reqwest;
use serde;
use std::fmt;
use dotenv;

#[derive(Debug)]
pub struct DictError;
impl fmt::Display for DictError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Could not load Dictionary")
  }
}

#[derive(Savefile,Debug)]
pub struct Dictionary {
  pub adjectives: Words,
  pub nouns: Words,
}

#[derive(serde::Deserialize)]
struct ResponseJson {
  values: Vec<Vec<String>>
}

pub async fn get_handle(i:u64) -> String {
  let dict = get().await.unwrap();
  let num_adj = dict.adjectives.len() as u64;
  let num_nouns = dict.nouns.len()  as u64;
  let noun_index = i % (num_nouns);
  let noun = dict.nouns[noun_index as usize].clone();
  let mut current_remaining_number : u64 = i / num_nouns;
  let mut handle : Vec<String> = vec![];
  while current_remaining_number > 0 {
    let adjective_index = (current_remaining_number % num_adj) as usize;
    handle.push( dict.adjectives[adjective_index].clone() ); 
    current_remaining_number = current_remaining_number / num_adj;
  }
  handle.push( noun );
  return handle.join(" ");
}

pub async fn get_number(handle:String) -> u64 {
  let mut return_number : u64 = 0;
  let dict = get().await.unwrap();
  let num_adj = dict.adjectives.len() as u64;
  let num_nouns = dict.nouns.len()  as u64;
  let mut words : Vec<&str> = handle.split(' ').collect();
  let noun = words.pop().unwrap();
  words.reverse();
  for adjective in words {
    let adj_index = dict.adjectives.iter().position(|word| word == adjective ).unwrap() as u64;
    return_number = return_number * num_adj;
    return_number = return_number + adj_index;
  }
  let noun_index = dict.nouns.iter().position(|word| word == noun ).unwrap() as u64;
  return_number = return_number * num_nouns;
  return_number = return_number + noun_index;
  return return_number;
}

pub async fn update() -> Result<Dictionary, DictError> {
  let get_dict_result = get_dict().await;
  match get_dict_result {
    Ok(dict) => {
      match save_dict(&dict) {
        Ok(_) => return Ok(dict),
        Err(_)=> return Err(DictError)
      }
    }
    Err(_) => return Err(DictError)
  }
}

pub async fn get() -> Result<Dictionary, DictError> {
  let loaded = load_dict();
  match loaded {
    Ok(dict) => return Ok(dict),
    Err(_) => {
      let updated = update().await;
      match updated {
        Ok(new_dict) => return Ok(new_dict),
        Err(_) => return Err(DictError),
      }
    }
  }
}

fn save_dict(dict:&Dictionary) -> Result<(),savefile::SavefileError> {
  save_file("dict.bin", 0, dict)
}

fn load_dict() -> Result<Dictionary,savefile::SavefileError>  {
  load_file("dict.bin", 0)
}

fn get_google_api_key() -> String {
  match dotenv::var("GOOGLE_API_KEY") {
      Ok(val) => val,
      Err(_) => {
        println!("No GOOGLE_API_KEY env var set to read google sheet. Cannot update dictionary.");
        "".to_string()
      },
  }
}

async fn get_dict() -> Result<Dictionary, reqwest::Error> {
  let key = get_google_api_key();
  let url = format!("https://sheets.googleapis.com/v4/spreadsheets/1AJmUhAi5un3-pdYN85iEgq2_5TnxNUsYTL4IViHwk8Q/values/Sheet1!A:B?majorDimension=COLUMNS&key={}",key);
  let body : ResponseJson = reqwest::get(&url)
    .await?
    .json::<ResponseJson>()
    .await?;
  let mut dict = Dictionary {
    adjectives:vec![],
    nouns: vec![],
  };
  body.values[0].iter().for_each(|word| if word.len() > 0 { dict.adjectives.push(word.clone()) });
  body.values[1].iter().for_each(|word| if word.len() > 0 { dict.nouns     .push(word.clone()) });
  Ok(dict)
}