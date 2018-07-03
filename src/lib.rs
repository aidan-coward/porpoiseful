use std::path::{Path, PathBuf};

/// Determines which value is to be displayed

enum DisplayValue {

/// The current percentage of battery remaining
  BatteryLifePercentage,
/// 
  CPULoadAverage,
/// The average load of all the cpu cores in percent
  
  CPULoadCore0,
/// the load of the 0th core

  CPULoadCore1,
/// the load of the 1st core

  CPULoadCore2,
/// the load of the 2nd core

  CPULoadCore3,
/// the load of the 3rd core

  CPULoadCore4,
/// the load of the 4th core

  CPULoadCore5,
/// the load of the 5th core

  CPULoadCore6,
/// the load of the 6th core

  CPULoadCore7,
/// the load of the 7th core

  CPUTemperatureAverage,
/// Average temperature of all the cpu cores

  CPUTemperatureCore0,
/// The temperature of the 0th core

  CPUTemperatureCore1,
/// The temperature of the 1st core

  CPUTemperatureCore2,
/// The temperature of the 2nd core

  CPUTemperatureCore3,
/// The temperature of the 3rd core

  CPUTemperatureCore4,
/// The temperature of the 4th core

  CPUTemperatureCore5,
/// The temperature of the 5th core

  CPUTemperatureCore6,
/// The temperature of the 6th core

  CPUTemperatureCore7,
/// The temperature of the 7th core

  GPULoad1,
/// The load of the 1st GPU in percentage

  GPULoad2,
/// The load of the 2nd GPU in percentage

  GPUTemperature1,
/// The temperature of the 0th GPU

  GPUTemperature2,
/// The temperature of the 1st GPU

  RAMUsage,
/// Usage of RAM

  RAMUsagePercentage,
/// Usage of RAM in percentage

  Date,
  Time,
  AudioVolume,
  HardDriveUsage1,
  HardDriveUsagePercentage1,
  HardDriveUsage2,
  HardDriveUsagePercentage2,
}

