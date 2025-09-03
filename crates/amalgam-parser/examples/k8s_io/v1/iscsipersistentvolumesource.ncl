# ISCSIPersistentVolumeSource represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI volumes support ownership management and SELinux relabeling.
{
  chapAuthDiscovery | optional | Bool | doc "chapAuthDiscovery defines whether support iSCSI Discovery CHAP authentication",
  chapAuthSession | optional | Bool | doc "chapAuthSession defines whether support iSCSI Session CHAP authentication",
  fsType | optional | String | doc m%"
fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi
"%,
  initiatorName | optional | String | doc m%"
initiatorName is the custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.
"%,
  iqn | optional | String | doc "iqn is Target iSCSI Qualified Name.",
  iscsiInterface | optional | String | doc m%"
iscsiInterface is the interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).
"%,
  lun | optional | Number | doc "lun is iSCSI Target Lun number.",
  portals | optional | Array String | doc m%"
portals is the iSCSI Target Portal List. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
"%,
  readOnly | optional | Bool | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
"%,
  secretRef | optional | SecretReference | doc "secretRef is the CHAP Secret for iSCSI target and initiator authentication",
  targetPortal | optional | String | doc m%"
targetPortal is iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
"%,
}
