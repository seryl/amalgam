# volumeDevice describes a mapping of a raw block device within a container.
{
  devicePath | optional | String | doc m%"
devicePath is the path inside of the container that the device will be mapped to.
"%,
  name | optional | String | doc "name must match the name of a persistentVolumeClaim in the pod",
}
