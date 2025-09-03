# Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.
{
  fsType | optional | String | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%,
  lun | optional | Number | doc "lun is Optional: FC target lun number",
  readOnly | optional | Bool | doc m%"
readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%,
  targetWWNs | optional | Array String | doc "targetWWNs is Optional: FC target worldwide names (WWNs)",
  wwids | optional | Array String | doc m%"
wwids Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
"%,
}
