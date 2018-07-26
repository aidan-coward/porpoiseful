#[derive(PartialEq, Debug)] 

/// Holds the information relative to the arguments of the flag

pub struct FlagOption {

    /// The flag 
    pub flag: String,

    /// The valid commands or arguments 
    pub arguments: Option<Vec<String>>,

    /// The minimum number of arguments for the flag
    pub min_arguments: usize,

    /// The maximum number of arguments for the flag
    pub max_arguments: usize,

    /// Whether the flag takes a path to a configuration file
    pub config_path: bool,

    /// Whether the flag takes a path to a executable
    pub command_path: bool,

    /// Whether the flag takes a path to a file to be read directly
    pub file_path: bool,
}

/// Outputs a vector of all the FlagOptions
pub fn flag_options_vec() -> Vec<FlagOption> {

    return vec![

        FlagOption {
            flag: "--battery".to_string(),
            arguments: Some(vec![
                            "time",
                            "percentage"]
                            .iter()
                            .map(|i| i.to_string())
                            .collect()),
            min_arguments: 1,
            max_arguments: 2,
            config_path: false,
            command_path: false,
            file_path: false,
        },

       FlagOption {
           flag: "--cpu_temp".to_string(),
           //arguments: Some(vec!["average".to_string, "each".to_string(), "fahrenheidt".to_string(), "celcius".to_string()]),
           arguments: Some(vec![
                           "average", 
                           "each", 
                           "fahrenheidt", 
                           "celcius"]
                           .iter()
                           .map(|i| i.to_string())
                           .collect()),
           min_arguments: 0,
           max_arguments: 2,
           config_path: false,
           command_path: false,
           file_path: true,
       },
    
       FlagOption {
           flag: "--cpu_load".to_string(),
           arguments: Some(vec![
                           "average",
                           "each"]
                           .iter()
                           .map(|i| i.to_string())
                           .collect()),
           min_arguments: 0,
           max_arguments: 1,
           config_path: false,
           command_path: true,
           file_path: false,
       },

       FlagOption { 
           flag: "--gpu_temp".to_string(),
           arguments: Some(vec![
                           "average",
                           "each"]
                           .iter()
                           .map(|i| i.to_string())
                           .collect()),
           min_arguments: 0,
           max_arguments: 1,
           config_path: false,
           command_path: true,
           file_path: false,
       },

       FlagOption {
           flag: "--gpu_load".to_string(),
           arguments: Some(vec![
                           "average",
                           "each"]
                           .iter()
                           .map(|i| i.to_string())
                           .collect()),
           min_arguments: 0,
           max_arguments: 1,
           config_path: false,
           command_path: true,
           file_path: false,
       },

       FlagOption {
           flag: "--ram_usage".to_string(),
           arguments: Some(vec![
                           "percentage",
                           "amount"]
                           .iter()
                           .map(|i| i.to_string())
                           .collect()),
           min_arguments: 0,
           max_arguments: 1,
           config_path: false,
           command_path: true,
           file_path: false,
       },

       FlagOption {
           flag: "--date".to_string(),
           arguments: Some(vec![
                           "day-long",
                           "day-short",
                           "month-long",
                           "month-short",
                           "year-long",
                           "year-short"]
                           .iter()
                           .map(|i| i.to_string())
                           .collect()),
            min_arguments: 1,
            max_arguments: 3,
            config_path: false,
            command_path: true,
            file_path: false,
       },

       FlagOption {
           flag: "--time".to_string(),
           arguments: Some(vec![
                           "12-hour",
                           "24-hour"]
                           .iter()
                           .map(|i| i.to_string())
                           .collect()),
           min_arguments: 1,
           max_arguments: 1,
           config_path: false,
           command_path: true,
           file_path: false,
       },

       FlagOption {
           flag: "--audio-volume".to_string(),
           arguments: Some(vec![
                           "oss",
                           "alsa",
                           "pulseaudio"]
                           .iter()
                           .map(|i| i.to_string())
                           .collect()),
           min_arguments: 1,
           max_arguments: 1,
           config_path: false,
           command_path: true,
           file_path: true,
       },

       FlagOption {
           flag: "--disk_usage".to_string(),
           arguments: Some(vec![
                           "percentage",
                           "bytes"]
                           .iter()
                           .map(|i| i.to_string())
                           .collect()),
           min_arguments: 1,
           max_arguments: 2,
           config_path: false,
           command_path: true,
           file_path: true,
       },
    ]
}

/// Iterates through all the current FlagOptions and produces a vector of all the possible flags
///
/// ``` 
/// use porpoiseful::flag_options::*;
///
/// let mut test_vec = vec![];
/// for item in flag_options_vec().iter() {
///     test_vec.push(item.flag.clone());
///     }
///
/// assert_eq!(flag_vec(), test_vec);
///
/// ```
pub fn flag_vec() -> Vec<String> {
   let mut output_vec = vec![];

   for item in flag_options_vec().iter() {
       output_vec.push(item.flag.clone());
   }

   output_vec
}

