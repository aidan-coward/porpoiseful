use std::path::{Path, PathBuf}

/// Determines which value is to be displayed

enum DisplayValue {

/// The current percentage of battery remaining
/// In percentage by default
  Battery,
  CPU,
  CPUTemperature,
  GPU,
  GPUTemperature,
  Ram,
  RamTemperature,
  Date,
  Time,
  AudioVolume,
  HardDriveUsage,
}

