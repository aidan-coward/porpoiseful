#[macro_use]
extern crate lazy_static;

use std::path::{Path, PathBuf};

/// Determines which value is to be displayed

pub enum DisplayValue {

/// The current percentage of battery remaining
  BatteryLifePercentage,

/// The average load of all the cpu cores in percent
  CPULoadAverage,
  
/// the load of the 0th core
  CPULoadCore0,

/// the load of the 1st core
  CPULoadCore1,

/// the load of the 2nd core
  CPULoadCore2,

/// the load of the 3rd core
  CPULoadCore3,

/// the load of the 4th core
  CPULoadCore4,

/// the load of the 5th core
  CPULoadCore5,

/// the load of the 6th core
  CPULoadCore6,

/// the load of the 7th core
  CPULoadCore7,

/// Average temperature of all the cpu cores
  CPUTemperatureAverage,

/// The temperature of the 0th core
  CPUTemperatureCore0,

/// The temperature of the 1st core
  CPUTemperatureCore1,

/// The temperature of the 2nd core
  CPUTemperatureCore2,

/// The temperature of the 3rd core
  CPUTemperatureCore3,

/// The temperature of the 4th core
  CPUTemperatureCore4,

/// The temperature of the 5th core
  CPUTemperatureCore5,

/// The temperature of the 6th core
  CPUTemperatureCore6,

/// The temperature of the 7th core
  CPUTemperatureCore7,

/// The load of the 1st GPU in percentage
  GPULoad0,

/// The load of the 2nd GPU in percentage
  GPULoad1,

/// The temperature of the 0th GPU
  GPUTemperature1,

/// The temperature of the 1st GPU
  GPUTemperature2,

/// Usage of RAM
  RAMUsage,

/// Usage of RAM in percentage
  RAMUsagePercentage,

/// System date 
  Date,

/// System time in 12 hour format
  Time12Hour,

///System time in 24 hour format
  Time24Hour,

/// Audio volume alsa
  AudioVolumeAlsa,

/// Audio volume oss
  AudioVolumeOSS,

/// Audio volume pulseaudio
  AudioVolumePulseAudio,

/// The usage of the 0th hard drive
  HardDriveUsage0,

/// The usage of the 0th hard drive in percent
  HardDriveUsagePercentage0,

/// The usage of the 1st hard drive
  HardDriveUsage1,

/// The usage of the 1st hard drive in percent
  HardDriveUsagePercentage1,

}


/// Holds the information of a value that is to be displayed
pub struct Config {

/// The value that is to be displayed
    pub display_value: DisplayValue,

/// The absolute path of the file to be read
    pub file_path: Option<PathBuf>,

/// The absolute path of the command to be run
    pub command_path: Option<PathBuf>,

/// The arguments passed to the command
    pub args: Option<Vec<String>>,

}

/// Takes vector of args and separates them into vectors of args and their related data
///
/// This is used for the arguments given via the command line
///
/// Returns Err if an invalid flag is given
///
/// # Examples 
///
/// ``` 
/// use::porpoiseful::{is_flag, arg_parser};
///
/// let test_vec: Vec<String> = vec!["--battery-percentage".to_string(), "is gucci".to_string()];
///
/// let test_output: Result<Vec<Vec<String>>, String> = Ok(vec![vec!["--battery-percentage".to_string(), "is gucci".to_string()]]);
///
/// assert_eq!(arg_parser(&test_vec), test_output);
///
/// ```
///
/// ```
/// use::porpoiseful::{is_flag, arg_parser};
///
/// let test_vec: Vec<String> = vec!["--noot".to_string(), "--battery-percentage".to_string(), "is gucci".to_string()];
///
/// let test_output = Err("--noot is not a valid flag".to_string());
///
/// assert_eq!(arg_parser(&test_vec), test_output)
pub fn arg_parser(args: &[String]) -> Result<Vec<Vec<String>>, String > {
    let mut output_vec: Vec<Vec<String>> = vec![vec![]];
    let mut flag_counter: usize = 0;

    for i in 0..args.len() {

        match is_flag(&args[i]) {

            Ok(x) => if x {
                if !(i == 0) { 
                    flag_counter += 1;
                }
                output_vec[flag_counter].push(args[i].clone());
                
            } else {
                output_vec[flag_counter].push(args[i].clone());
            },

            Err(why) => return Err(why),
        }
    }
        Ok(output_vec)
}


/// Determines whether the given value is a valid flag 
///
/// Returns Ok if a valid flag(true) or not a flag at all(false)
///
/// Returns Err if a flag but invalid
///
/// # Examples
///
/// ```
/// use::porpoiseful::is_flag;
///
/// let flag = "--battery-percentage";
///
/// assert_eq!(is_flag(flag), Ok(true));
///
/// ```
///
/// ```
/// use::porpoiseful::is_flag;
///
/// let not_flag = "/home/user/.porpoiseful/config_file.conf";
///
/// assert_eq!(is_flag(not_flag), Ok(false));
///
/// ```
///
/// ```
/// use::porpoiseful::is_flag;
///
/// let invalid_flag = "--noot";
///
/// assert_eq!(is_flag(invalid_flag), Err("--noot is not a valid flag".to_string()));
///
/// ```
pub fn is_flag(arg: &str) -> Result<bool, String> {

    if &arg[0..2] == "--" {

        //match arg {
        //          "--battery-percentage" 
        //        | "--cpu-temp=average"  
        //        | "--cpu-temp=each"
        //    
        //    => return Ok(true),
        //    _ => { 
        //        return Err(format!("{} is not a valid flag", arg).to_string())
        //    }
        //};

        if is_element(arg, FLAG_LIST.to_vec()) {
            return Ok(true);
        } else {
            return Err(format!("{} is not a valid flag", arg).to_string());
        }
    } else { 
        Ok(false) 
    }
}

/// Checks whether arg is part of list
///
/// Made for use with FLAG_LIST
pub fn is_element(arg: &str, list: Vec<&'static str>) -> bool {
    for i in 0..list.len() {
        if list[i] == arg {
            return true;
        }
    }
    false
}

lazy_static! {


/// The list of all the flags recognized as valid
    static ref FLAG_LIST: Vec<&'static str> = 
        vec![
            "--battery-percentage",
            "--cpu_temp=average",
            "--cpu_temp=each" 
        ];
}


