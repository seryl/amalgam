# Module: k8s.io.v1

# Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.
{
  items | optional | Array downwardapivolumefile.DownwardAPIVolumeFile | doc "Items is a list of DownwardAPIVolume file"
}
