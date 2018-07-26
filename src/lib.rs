use std::path::{PathBuf};
//use std::path::{Path, PathBuf};

pub mod flag_options;

/// Determines which value is to be displayed, including newlines

#[derive(PartialEq, Debug)] 
pub enum DisplayValue {

    /// If the value to be displayed 
    Flag(String),

    /// To indicate a newline
    Newline,

    /// Solely for initialization of Config
    NoValue,
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
    pub arguments: Option<Vec<String>>,

    // pub color: Option
    // option of color type
    // figure how to represent color
}

/// Takes vector of args and separates them into vectors of args and their related data
///
/// This is used for the arguments given via the command line
///
/// # Examples
///
/// ```
/// use::porpoiseful::{arg_parser};
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
/// use::porpoiseful::{arg_parser};
///
/// let test_vec: Vec<String> = vec!["--noot".to_string(), "--battery".to_string(), "is gucci".to_string()];
///
/// let test_output = Ok(vec![vec!["--noot".to_string()], vec!["--battery".to_string(), "is gucci".to_string()]]);
///
/// assert_eq!(arg_parser(&test_vec), test_output)
///
/// ```
///
/// ```
/// use::porpoiseful::{arg_parser};
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
    let mut output_vec: Vec<Vec<String>> = vec![];
    let mut flag_counter: usize = 0;

    for (index, item) in args.iter().enumerate() {

        match &item[0..2] {

            "--" => {
                output_vec.push(vec![item.clone()]);
                if !(index == 0) {
                    flag_counter += 1;
                }
            }

            _ => {
                if index == 0 {
                    return Err(format!("{} is not a valid flag", item).to_string());
                }

                output_vec[flag_counter].push(item.clone());
            }
        }
    }

    Ok(output_vec)
}

/// Takes a vector of vectors - the arguments being the first element of the vectors and their
/// arguments as the remaining elements
/// Verifies if the first element of each vector is a valid flag and the remaining arguments are
/// valid given the flag
/// Returns Ok(true) everything is valid
/// Returns Err if an invalid flag or argument is given
///
/// ``` 
///
/// use porpoiseful::flag_verify;
/// pub mod flag_options;
///
/// let test_args = vec![vec!["--battery".to_string()]];
///
/// let test_output = Err("--battery requires at least 1 arguments".to_string());
///
/// assert_eq!(flag_verify(&test_args), test_output);
///
/// ```
///
/// ``` 
///
/// use porpoiseful::flag_verify;
/// pub mod flag_options;
///
/// let test_args = vec![vec!["--battery".to_string(), "time".to_string()]];
///
/// let test_output = Ok(());
///
/// assert_eq!(flag_verify(&test_args), test_output);
///
/// ```
///
/// ``` 
///
/// use porpoiseful::flag_verify;
/// pub mod flag_options;
///
/// let test_args = vec![vec!["--battery".to_string(), "rocks".to_string()]];
///
/// let test_output = Err("rocks is not a valid argument for --battery".to_string());
///
/// assert_eq!(flag_verify(&test_args), test_output);
///
/// ```


pub fn flag_verify(args: &Vec<Vec<String>>) -> Result<(), String> {

    let flags = flag_options::flag_vec();
    
    let flag_options = flag_options::flag_options_vec();

    for item in args.iter() { 

        // check if the first element of each vector is a valid flag
        if flags.contains(&item[0]) {

            // if flag is a newline or a novalue, skip it 
            // if there are arguments given, return Err
            if item[0] == "--newline".to_string() {
                if item.len() > 1 {
                    return Err(format!("{} does not take any arguments", item[0]));
                } else {
                    continue;
                }
            }

            // iterate through the FlagOptions to find the right one 
            for option in flag_options.iter() {

                println!("{} is the current flag", item[0]);

                // if the flag matches that of a FlagOption
                if item[0] == option.flag {

                    println!("yassss");

                    // check if number of arguments is acceptable given the FlagOption
                    match item[1..].len() {

                        // not enough arguments
                        d if d < option.min_arguments => {
                            return Err(format!("{} requires at least {} arguments", item[0], option.min_arguments));
                        },

                        // too many arguments
                        d if d > option.max_arguments => {
                            return Err(format!("{} requires no more than {} arguments", item[0], option.max_arguments));
                        },

                        // right number of arguments
                        d if ((d >= option.min_arguments) && (d <= option.max_arguments)) => {

                            // check if arguments are valid
                            'inner: for arg in item[1..].iter() {
                                if option.arguments.clone().unwrap().contains(&arg) {
                                    println!("doot");
                                    continue 'inner ;
                                } else {
                                    return Err(format!("{} is not a valid argument for {}", arg, item[0]));
                                }
                            }
                        },

                        _ => {
                            return Err(format!("Something went wrong processing the arguments of {}", item[0]));
                        }
                    }

                    // all arguments have been verified to be true, continue to next flag
                    break;

                    // if no flag matching that of a FlagOption is found, return Err with the name of the invalid flag
                } else {
                    return Err(format!("{} is not a valid flag", item[0]));
                }
            }
        }
    }
    
    return Ok(())
}


impl Config {

// Takes a list of args and creates a Result<Config> based on those values
//
// Err if first element of args isn't a flag
// Err if a file path does not refer to a file, does not exist or is inaccessible
//
// Takes vector from args_flags and creates a Result<Config> with values set
// 
// ```
//
// use porpoiseful::{Config};
//
// let mut test_args: Vec<String> = vec!["args".to_string()];
//
// let test_output: Result<Config, String> = Err("not a valid configuration option".to_string());
//
// assert_eq!(Config::new(&mut test_args), test_output);
//
// ```
// 
// ``` 
//
// use porpoiseful::{Config, DisplayValue};
// use std::path::PathBuf;
//
// let mut test_args: Vec<String> = vec!["--battery".to_string()];
//
// let test_output: Result<Config, String> = 
//     Ok( Config { display_value: porpoiseful::DisplayValue::BatteryLifePercentage, command_path: None, config_path: None, file_path: Some(PathBuf::from("/sys/class/power_supply/BAT0/capacity")), arguments: None });
//
// assert_eq!(Config::new(&mut test_args), test_output);
//
// ```
//
//    pub fn new(args: &mut [String]) -> Result<Config, String> {
//
//        let output_config: Config = 
//            Config { display_value: DisplayValue::NoValue, 
//                     command_path: None,
//                     config_path: None,
//                     file_path: None,
//                     arguments: None };
//
//        Ok(output_config)
//
//    }
//
//
}
