# Module: k8s.io.v1.volumedevice

{
  # volumeDevice describes a mapping of a raw block device within a container.
  VolumeDevice = {
    devicePath | String | doc m%"
devicePath is the path inside of the container that the device will be mapped to.
"%,
    name | String | doc "name must match the name of a persistentVolumeClaim in the pod",
  },
}
