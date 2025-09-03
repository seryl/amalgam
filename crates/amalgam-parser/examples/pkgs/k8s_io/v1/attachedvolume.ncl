# Module: k8s.io.v1

# AttachedVolume describes a volume attached to a node
{
  devicePath | optional | String | doc "DevicePath represents the device path where the volume should be available",
  name | optional | String | doc "Name of the attached volume"
}
