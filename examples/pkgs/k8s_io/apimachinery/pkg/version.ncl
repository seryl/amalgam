{
  buildDate
     | String,
  compiler
     | String,
  emulationMajor
     | String
     | doc "EmulationMajor is the major version of the emulation version"
     | optional,
  emulationMinor
     | String
     | doc "EmulationMinor is the minor version of the emulation version"
     | optional,
  gitCommit
     | String,
  gitTreeState
     | String,
  gitVersion
     | String,
  goVersion
     | String,
  major
     | String
     | doc "Major is the major version of the binary version",
  minCompatibilityMajor
     | String
     | doc "MinCompatibilityMajor is the major version of the minimum compatibility version"
     | optional,
  minCompatibilityMinor
     | String
     | doc "MinCompatibilityMinor is the minor version of the minimum compatibility version"
     | optional,
  minor
     | String
     | doc "Minor is the minor version of the binary version",
  platform
     | String
}