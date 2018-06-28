#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

use std::path::PathBuf;
use std::path::Path;


#[derive(Debug)]

/// the struct that contains which items are to be displayed
pub struct Config {
  battery: bool,
  cpu: bool, 
  cpu_temperature: bool,
  gpu: bool,
  gpu_temperature: bool,
  ram: bool,
  ram_temperature: bool,
  date: bool,
  time: bool,
  volume: bool,
  hard_drive_usage: bool,
  config_file: bool,
  config_file_path: Option<PathBuf>,
}

impl Config {

  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    let mut battery = false;
    let mut cpu = false;
    let mut cpu_temperature = false;
    let mut gpu = false;
    let mut gpu_temperature = false;
    let mut ram = false;
    let mut ram_temperature = false;
    let mut date = false;
    let mut time = false;
    let mut volume = false;
    let mut hard_drive_usage = false; 
    let mut config_file = false;
    let mut config_file_path: Option<PathBuf> = None;
    
    // this is a counting bit to account for invalid flags
    // if a value is not a valid flag and next_is_config is
    // set to false, then new() returns an Err
    let mut next_is_config: bool = false;
  
    for x in 1..args.len() {
        println!("arg is: {}", args[x]);
      match args[x].as_str() {
        "--battery" => battery = true,
        "--cpu" => cpu = true,
        "--cpu_temperature" => cpu_temperature = true,
        "--gpu" => gpu = true,
        "--gpu_temperature" => gpu_temperature = true,
        "--ram" => ram = true,
        "--ram_temperature" => ram_temperature = true,
        "--date" => date = true,
        "--time" => time = true,
        "--volume" => volume = true,
        "--hard_drive_usage" => hard_drive_usage = true,
        "--config" => {
              if x == ( args.len() - 1 ) {
                  return Err("Must specify config file path if using --config flag")
              } else if Path::new(&args[x + 1]).is_file() {
                config_file_path = Some(PathBuf::from(&args[x + 1].clone()));
                config_file = true;
                next_is_config = true;
              } else {
                  return Err("Invalid config path, check validity and/or permissions")
              }
          },
        _ => {
          if next_is_config { 
            next_is_config = false;
          } else {
            return Err("Invalid flag")
          }
        },
      }
    }
    Ok (Config { battery, cpu, cpu_temperature, gpu, gpu_temperature, ram, ram_temperature, date, time, volume, hard_drive_usage, config_file, config_file_path } )
  }
}
