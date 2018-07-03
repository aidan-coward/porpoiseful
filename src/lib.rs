use std::path::{Path, PathBuf};

/// Determines which value is to be displayed

enum DisplayValue {

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

