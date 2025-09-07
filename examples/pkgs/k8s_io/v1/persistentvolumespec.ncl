let aWSElasticBlockStoreVolumeSource = import "./AWSElasticBlockStoreVolumeSource.ncl" in
let azureDiskVolumeSource = import "./AzureDiskVolumeSource.ncl" in
let azureFilePersistentVolumeSource = import "./AzureFilePersistentVolumeSource.ncl" in
let cephFSPersistentVolumeSource = import "./CephFSPersistentVolumeSource.ncl" in
let cinderPersistentVolumeSource = import "./CinderPersistentVolumeSource.ncl" in
let objectReference = import "./ObjectReference.ncl" in
let cSIPersistentVolumeSource = import "./CSIPersistentVolumeSource.ncl" in
let fCVolumeSource = import "./FCVolumeSource.ncl" in
let flexPersistentVolumeSource = import "./FlexPersistentVolumeSource.ncl" in
let flockerVolumeSource = import "./FlockerVolumeSource.ncl" in
let gCEPersistentDiskVolumeSource = import "./GCEPersistentDiskVolumeSource.ncl" in
let glusterfsPersistentVolumeSource = import "./GlusterfsPersistentVolumeSource.ncl" in
let hostPathVolumeSource = import "./HostPathVolumeSource.ncl" in
let iSCSIPersistentVolumeSource = import "./ISCSIPersistentVolumeSource.ncl" in
let localVolumeSource = import "./LocalVolumeSource.ncl" in
let nFSVolumeSource = import "./NFSVolumeSource.ncl" in
let volumeNodeAffinity = import "./VolumeNodeAffinity.ncl" in
let photonPersistentDiskVolumeSource = import "./PhotonPersistentDiskVolumeSource.ncl" in
let portworxVolumeSource = import "./PortworxVolumeSource.ncl" in
let quobyteVolumeSource = import "./QuobyteVolumeSource.ncl" in
let rBDPersistentVolumeSource = import "./RBDPersistentVolumeSource.ncl" in
let scaleIOPersistentVolumeSource = import "./ScaleIOPersistentVolumeSource.ncl" in
let storageOSPersistentVolumeSource = import "./StorageOSPersistentVolumeSource.ncl" in
let vsphereVirtualDiskVolumeSource = import "./VsphereVirtualDiskVolumeSource.ncl" in

# Module: k8s.io.v1

# PersistentVolumeSpec is the specification of a persistent volume.
{
  accessModes | optional | Array String | doc m%"
accessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes
"%,
  awsElasticBlockStore | optional | aWSElasticBlockStoreVolumeSource | doc m%"
awsElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Deprecated: AWSElasticBlockStore is deprecated. All operations for the in-tree awsElasticBlockStore type are redirected to the ebs.csi.aws.com CSI driver. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%,
  azureDisk | optional | azureDiskVolumeSource | doc m%"
azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod. Deprecated: AzureDisk is deprecated. All operations for the in-tree azureDisk type are redirected to the disk.csi.azure.com CSI driver.
"%,
  azureFile | optional | azureFilePersistentVolumeSource | doc m%"
azureFile represents an Azure File Service mount on the host and bind mount to the pod. Deprecated: AzureFile is deprecated. All operations for the in-tree azureFile type are redirected to the file.csi.azure.com CSI driver.
"%,
  capacity | optional | { .. } | doc m%"
capacity is the description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
"%,
  cephfs | optional | cephFSPersistentVolumeSource | doc m%"
cephFS represents a Ceph FS mount on the host that shares a pod's lifetime. Deprecated: CephFS is deprecated and the in-tree cephfs type is no longer supported.
"%,
  cinder | optional | cinderPersistentVolumeSource | doc m%"
cinder represents a cinder volume attached and mounted on kubelets host machine. Deprecated: Cinder is deprecated. All operations for the in-tree cinder type are redirected to the cinder.csi.openstack.org CSI driver. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%,
  claimRef | optional | objectReference | doc m%"
claimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-nil when bound. claim.VolumeName is the authoritative bind between PV and PVC. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding
"%,
  csi | optional | cSIPersistentVolumeSource | doc "csi represents storage that is handled by an external CSI driver.",
  fc | optional | fCVolumeSource | doc m%"
fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
"%,
  flexVolume | optional | flexPersistentVolumeSource | doc m%"
flexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin. Deprecated: FlexVolume is deprecated. Consider using a CSIDriver instead.
"%,
  flocker | optional | flockerVolumeSource | doc m%"
flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running. Deprecated: Flocker is deprecated and the in-tree flocker type is no longer supported.
"%,
  gcePersistentDisk | optional | gCEPersistentDiskVolumeSource | doc m%"
gcePersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin. Deprecated: GCEPersistentDisk is deprecated. All operations for the in-tree gcePersistentDisk type are redirected to the pd.csi.storage.gke.io CSI driver. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%,
  glusterfs | optional | glusterfsPersistentVolumeSource | doc m%"
glusterfs represents a Glusterfs volume that is attached to a host and exposed to the pod. Provisioned by an admin. Deprecated: Glusterfs is deprecated and the in-tree glusterfs type is no longer supported. More info: https://examples.k8s.io/volumes/glusterfs/README.md
"%,
  hostPath | optional | hostPathVolumeSource | doc m%"
hostPath represents a directory on the host. Provisioned by a developer or tester. This is useful for single-node development and testing only! On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
"%,
  iscsi | optional | iSCSIPersistentVolumeSource | doc m%"
iscsi represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin.
"%,
  local | optional | localVolumeSource | doc "local represents directly-attached storage with node affinity",
  mountOptions | optional | Array String | doc m%"
mountOptions is the list of mount options, e.g. ["ro", "soft"]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options
"%,
  nfs | optional | nFSVolumeSource | doc m%"
nfs represents an NFS mount on the host. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%,
  nodeAffinity | optional | volumeNodeAffinity | doc m%"
nodeAffinity defines constraints that limit what nodes this volume can be accessed from. This field influences the scheduling of pods that use this volume.
"%,
  persistentVolumeReclaimPolicy | optional | String | doc m%"
persistentVolumeReclaimPolicy defines what happens to a persistent volume when released from its claim. Valid options are Retain (default for manually created PersistentVolumes), Delete (default for dynamically provisioned PersistentVolumes), and Recycle (deprecated). Recycle must be supported by the volume plugin underlying this PersistentVolume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming
"%,
  photonPersistentDisk | optional | photonPersistentDiskVolumeSource | doc m%"
photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine. Deprecated: PhotonPersistentDisk is deprecated and the in-tree photonPersistentDisk type is no longer supported.
"%,
  portworxVolume | optional | portworxVolumeSource | doc m%"
portworxVolume represents a portworx volume attached and mounted on kubelets host machine. Deprecated: PortworxVolume is deprecated. All operations for the in-tree portworxVolume type are redirected to the pxd.portworx.com CSI driver when the CSIMigrationPortworx feature-gate is on.
"%,
  quobyte | optional | quobyteVolumeSource | doc m%"
quobyte represents a Quobyte mount on the host that shares a pod's lifetime. Deprecated: Quobyte is deprecated and the in-tree quobyte type is no longer supported.
"%,
  rbd | optional | rBDPersistentVolumeSource | doc m%"
rbd represents a Rados Block Device mount on the host that shares a pod's lifetime. Deprecated: RBD is deprecated and the in-tree rbd type is no longer supported. More info: https://examples.k8s.io/volumes/rbd/README.md
"%,
  scaleIO | optional | scaleIOPersistentVolumeSource | doc m%"
scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes. Deprecated: ScaleIO is deprecated and the in-tree scaleIO type is no longer supported.
"%,
  storageClassName | optional | String | doc m%"
storageClassName is the name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass.
"%,
  storageos | optional | storageOSPersistentVolumeSource | doc m%"
storageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod. Deprecated: StorageOS is deprecated and the in-tree storageos type is no longer supported. More info: https://examples.k8s.io/volumes/storageos/README.md
"%,
  volumeAttributesClassName | optional | String | doc m%"
Name of VolumeAttributesClass to which this persistent volume belongs. Empty value is not allowed. When this field is not set, it indicates that this volume does not belong to any VolumeAttributesClass. This field is mutable and can be changed by the CSI driver after a volume has been updated successfully to a new class. For an unbound PersistentVolume, the volumeAttributesClassName will be matched with unbound PersistentVolumeClaims during the binding process. This is a beta field and requires enabling VolumeAttributesClass feature (off by default).
"%,
  volumeMode | optional | String | doc m%"
volumeMode defines if a volume is intended to be used with a formatted filesystem or to remain in raw block state. Value of Filesystem is implied when not included in spec.
"%,
  vsphereVolume | optional | vsphereVirtualDiskVolumeSource | doc m%"
vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine. Deprecated: VsphereVolume is deprecated. All operations for the in-tree vsphereVolume type are redirected to the csi.vsphere.vmware.com CSI driver.
"%
}