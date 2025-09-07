let downwardAPIVolumeFile = import "./DownwardAPIVolumeFile.ncl" in

# Module: k8s.io.v1

# Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.
{
  items
     | Array downwardAPIVolumeFile
     | doc "Items is a list of DownwardAPIVolume file"
     | optional
}