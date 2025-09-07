# Module: k8s.io.v1

# AttachedVolume describes a volume attached to a node
{
  devicePath
     | String
     | doc "DevicePath represents the device path where the volume should be available",
  name
     | String
     | doc "Name of the attached volume"
}