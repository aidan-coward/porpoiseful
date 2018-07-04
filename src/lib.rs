#[macro_use]
extern crate lazy_static;

use std::path::{PathBuf};
//use std::path::{Path, PathBuf};

/// Determines which value is to be displayed

#[derive(PartialEq, Debug)] 
pub enum DisplayValue {

    /// For initialization of Config in Config::new()
    NoValue,

    /// Specifies the location of the desired config file\
    ConfigFile,

    /// The current percentage of battery remaining
    BatteryLifePercentage,

    /// The time remaining until the battery is at 0%
    BatteryLifeTime,

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

#[derive(PartialEq, Debug)] 

/// Holds the information of a value that is to be displayed
pub struct Config {
    /// The value that is to be displayed
    pub display_value: DisplayValue,

    /// The path of the command to be run
    pub command_path: Option<PathBuf>,

    /// The path of the config file to be read
    pub config_path: Option<PathBuf>,

    /// The path of the file to be read directly
    pub file_path: Option<PathBuf>,

    /// The arguments passed with the Config
    pub args: Option<Vec<String>>,
}

lazy_static! {


/// The list of all the flags recognized as valid
    static ref FLAG_LIST: Vec<&'static str> =
        vec![
            "--battery",
            "--cpu-temp",
            "--cpu-load",
            "--gpu-load",
            "--gpu-temp",
            "--ram-usage",
            "--date",
            "--time",
            "--audio-volume",
            "--disk-usage",



        ];
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
/// let flag = "--battery";
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
        
        if is_element(arg, FLAG_LIST.to_vec()) {
            return Ok(true);
        } else {
            return Err(format!("{} is not a valid flag", arg).to_string());
        }
    } else {
        Ok(false)
    }
}

/// Checks whether the given argument is part of the given list
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
/// let test_vec: Vec<String> = vec!["--battery".to_string(), "is gucci".to_string()];
///
/// let test_output: Result<Vec<Vec<String>>, String> = Ok(vec![vec!["--battery".to_string(), "is gucci".to_string()]]);
///
/// assert_eq!(arg_parser(&test_vec), test_output);
///
/// ```
///
/// ```
/// use::porpoiseful::{is_flag, arg_parser};
///
/// let test_vec: Vec<String> = vec!["--noot".to_string(), "--battery".to_string(), "is gucci".to_string()];
///
/// let test_output = Err("--noot is not a valid flag".to_string());
///
/// assert_eq!(arg_parser(&test_vec), test_output)
///
/// ```
///
/// ```
/// use::porpoiseful::{is_flag, arg_parser};
///
/// let test_vec: Vec<String> = vec!["--battery".to_string(), "arg1".to_string(),
///                                  "--cpu-temp".to_string(), "arg2".to_string(),
///                                  "--cpu-load".to_string(), "arg3".to_string()];
///
/// let test_output = Ok(vec![vec!["--battery".to_string(), "arg1".to_string()],
///                        vec!["--cpu-temp".to_string(), "arg2".to_string()],
///                        vec!["--cpu-load".to_string(), "arg3".to_string()]]);
///
/// assert_eq!(arg_parser(&test_vec), test_output)
///
/// ```

pub fn arg_parser(args: &[String]) -> Result<Vec<Vec<String>>, String> {
    let mut output_vec: Vec<Vec<String>> = vec![vec![]];
    let mut flag_counter: usize = 0;

    for i in 0..args.len() {
        match is_flag(&args[i]) {
            Ok(x) => if x {
                if !(i == 0) {
                    flag_counter += 1;
                    output_vec.push(vec![]);
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


impl Config {

/// Takes a list of args and creates a Result<Config> based on those values
///
/// Err if first element of args isn't a flag
/// Err if a file path does not refer to a file, does not exist or is inaccessible
///
/// Takes vector from args_flags and creates a Result<Config> with values set
/// 
/// ```
///
/// use porpoiseful::{Config};
///
/// let test_args: Vec<String> = vec!["args".to_string()];
///
/// let test_output: Result<Config, String> = Err("not a valid configuration option".to_string());
///
/// assert_eq!(Config::new(&test_args), test_output);
///
/// ```
/// 
/// ``` 
///
/// use porpoiseful::{Config, DisplayValue};
/// use std::path::PathBuf;
///
/// let test_args: Vec<String> = vec!["--battery".to_string()];
///
/// let test_output: Result<Config, String> = 
///     Ok( Config { display_value: porpoiseful::DisplayValue::BatteryLifePercentage, command_path: None, config_path: None, file_path: Some(PathBuf::from("/sys/class/power_supply/BAT0/capacity")), args: None });
///
/// assert_eq!(Config::new(&test_args), test_output);
///
/// ```

    pub fn new(args: &[String]) -> Result<Config, String> {

        let output_config: Config = 
            Config { display_value: DisplayValue::NoValue, 
                     command_path: None,
                     config_path: None,
                     file_path: None,
                     args: None };
    
        match args[0].as_ref() {

            "--battery" => {
                output_config.new_battery(&args)
            },

            "--config" => {
                output_config.new_conf_file(&args)
            },

            _ => Err("not a valid configuration option".to_string())
        }
    }

    

/// Creates a new config of BatteryLifePercentage
///
/// Err if it is given any arguments
///
/// ```
/// use porpoiseful::{Config,DisplayValue};
/// use std::path::PathBuf;
///
/// let test_args = vec!["--battery".to_string()];
///
/// let test_config = Config { display_value: DisplayValue::NoValue,
///                            command_path: None,
///                            config_path: None,
///                            file_path: None,
///                            args: None };
///
/// let test_output = Ok(Config { display_value: DisplayValue::BatteryLifePercentage,
///                            command_path: None,
///                            config_path: None,
///                            file_path:
///                            Some(PathBuf::from("/sys/class/power_supply/BAT0/capacity")),
///                            args: None });
///
/// assert_eq!(test_config.new_battery(&test_args), test_output);
///
/// ```

    pub fn new_battery(mut self, args: &[String]) -> Result<Config, String> {
        
        if args.len() > 1 { return Err("--battery doesn't take any arguments".to_string()) }

        self.display_value = DisplayValue::BatteryLifePercentage;
        self.file_path = Some(PathBuf::from("/sys/class/power_supply/BAT0/capacity"));
        
        Ok(self)
    }


/// Creates a new Config with ConfigFile and the desired configuration file 
/// Ok with default config if no file is given
///
/// ``` 
/// use porpoiseful::{Config,DisplayValue};
/// use std::path::PathBuf;
///
///
/// let test_args_ok = vec!["--config".to_string()];
///
/// let test_config = Config { display_value: DisplayValue::NoValue,
///                            command_path: None,
///                            config_path: None,
///                            file_path: None,
///                            args: None };
///
/// let test_output_ok = Ok(Config { display_value: DisplayValue::ConfigFile,
///                            command_path: None,
///                            config_path: 
///                            Some(PathBuf::from("/etc/porpoiseful/porpoiseful.conf")),
///                            file_path: None,
///                            args: None });
///
///
/// assert_eq!(test_config.new_conf_file(&test_args_ok), test_output_ok);
///
/// let test_config_err = Config { display_value: DisplayValue::NoValue,
///                            command_path: None,
///                            config_path: None,
///                            file_path: None,
///                            args: None };
///
/// let test_args_err = vec!["--config".to_string(), "/root/porpoiseful.conf".to_string()];
///
/// let test_output_err = Err("the given configuration file does not exist or is inaccessible".to_string());
///
/// assert_eq!(test_config_err.new_conf_file(&test_args_err), test_output_err);
/// ```


    pub fn new_conf_file(mut self, args: &[String]) -> Result<Config, String> {

        self.display_value = DisplayValue::ConfigFile;

        match args.len() { 

            // Return Err if more than one argument is given with the file - at least two plus the
            // file
            d if d > 2 => {
                return Err("--config flag takes up to one argument: an absolute file path to the config file desired".to_string());
            },

            // If a single argument is passed and it is a valid file, return Ok using the given
            // file as the config file
            // If it is not valid, return Err
            2 => {
                let given_config_path = PathBuf::from(args[1].clone());
                if given_config_path.exists() {
                    self.config_path = Some(given_config_path);
                    return Ok(self);
                } else {
                    return Err("the given configuration file does not exist or is inaccessible".to_string());
                }
            },

            // If there is only one argument given, return Ok and use the default configuration file
            _ => {
                self.config_path = Some(PathBuf::from("/etc/porpoiseful/porpoiseful.conf"));
                return Ok(self);
                },
        }
    }

    }







