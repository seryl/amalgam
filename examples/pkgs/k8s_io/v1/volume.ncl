let aWSElasticBlockStoreVolumeSource = import "./AWSElasticBlockStoreVolumeSource.ncl" in
let azureDiskVolumeSource = import "./AzureDiskVolumeSource.ncl" in
let azureFileVolumeSource = import "./AzureFileVolumeSource.ncl" in
let cephFSVolumeSource = import "./CephFSVolumeSource.ncl" in
let cinderVolumeSource = import "./CinderVolumeSource.ncl" in
let configMapVolumeSource = import "./ConfigMapVolumeSource.ncl" in
let cSIVolumeSource = import "./CSIVolumeSource.ncl" in
let downwardAPIVolumeSource = import "./DownwardAPIVolumeSource.ncl" in
let emptyDirVolumeSource = import "./EmptyDirVolumeSource.ncl" in
let ephemeralVolumeSource = import "./EphemeralVolumeSource.ncl" in
let fCVolumeSource = import "./FCVolumeSource.ncl" in
let flexVolumeSource = import "./FlexVolumeSource.ncl" in
let flockerVolumeSource = import "./FlockerVolumeSource.ncl" in
let gCEPersistentDiskVolumeSource = import "./GCEPersistentDiskVolumeSource.ncl" in
let gitRepoVolumeSource = import "./GitRepoVolumeSource.ncl" in
let glusterfsVolumeSource = import "./GlusterfsVolumeSource.ncl" in
let hostPathVolumeSource = import "./HostPathVolumeSource.ncl" in
let imageVolumeSource = import "./ImageVolumeSource.ncl" in
let iSCSIVolumeSource = import "./ISCSIVolumeSource.ncl" in
let nFSVolumeSource = import "./NFSVolumeSource.ncl" in
let persistentVolumeClaimVolumeSource = import "./PersistentVolumeClaimVolumeSource.ncl" in
let photonPersistentDiskVolumeSource = import "./PhotonPersistentDiskVolumeSource.ncl" in
let portworxVolumeSource = import "./PortworxVolumeSource.ncl" in
let projectedVolumeSource = import "./ProjectedVolumeSource.ncl" in
let quobyteVolumeSource = import "./QuobyteVolumeSource.ncl" in
let rBDVolumeSource = import "./RBDVolumeSource.ncl" in
let scaleIOVolumeSource = import "./ScaleIOVolumeSource.ncl" in
let secretVolumeSource = import "./SecretVolumeSource.ncl" in
let storageOSVolumeSource = import "./StorageOSVolumeSource.ncl" in
let vsphereVirtualDiskVolumeSource = import "./VsphereVirtualDiskVolumeSource.ncl" in

# Module: k8s.io.v1

# Volume represents a named volume in a pod that may be accessed by any container in the pod.
{
  awsElasticBlockStore
     | aWSElasticBlockStoreVolumeSource
     | doc m%"
awsElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Deprecated: AWSElasticBlockStore is deprecated. All operations for the in-tree awsElasticBlockStore type are redirected to the ebs.csi.aws.com CSI driver. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%
     | optional,
  azureDisk
     | azureDiskVolumeSource
     | doc m%"
azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod. Deprecated: AzureDisk is deprecated. All operations for the in-tree azureDisk type are redirected to the disk.csi.azure.com CSI driver.
"%
     | optional,
  azureFile
     | azureFileVolumeSource
     | doc m%"
azureFile represents an Azure File Service mount on the host and bind mount to the pod. Deprecated: AzureFile is deprecated. All operations for the in-tree azureFile type are redirected to the file.csi.azure.com CSI driver.
"%
     | optional,
  cephfs
     | cephFSVolumeSource
     | doc m%"
cephFS represents a Ceph FS mount on the host that shares a pod's lifetime. Deprecated: CephFS is deprecated and the in-tree cephfs type is no longer supported.
"%
     | optional,
  cinder
     | cinderVolumeSource
     | doc m%"
cinder represents a cinder volume attached and mounted on kubelets host machine. Deprecated: Cinder is deprecated. All operations for the in-tree cinder type are redirected to the cinder.csi.openstack.org CSI driver. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
     | optional,
  configMap
     | configMapVolumeSource
     | doc "configMap represents a configMap that should populate this volume"
     | optional,
  csi
     | cSIVolumeSource
     | doc m%"
csi (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers.
"%
     | optional,
  downwardAPI
     | downwardAPIVolumeSource
     | doc m%"
downwardAPI represents downward API about the pod that should populate this volume
"%
     | optional,
  emptyDir
     | emptyDirVolumeSource
     | doc m%"
emptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
"%
     | optional,
  ephemeral
     | ephemeralVolumeSource
     | doc m%"
ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed.
Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity
   tracking are needed,
c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through
   a PersistentVolumeClaim (see EphemeralVolumeSource for more
   information on the connection between this volume type
   and PersistentVolumeClaim).
Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod.
Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information.
A pod can use both types of ephemeral volumes and persistent volumes at the same time.
"%
     | optional,
  fc
     | fCVolumeSource
     | doc m%"
fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
"%
     | optional,
  flexVolume
     | flexVolumeSource
     | doc m%"
flexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin. Deprecated: FlexVolume is deprecated. Consider using a CSIDriver instead.
"%
     | optional,
  flocker
     | flockerVolumeSource
     | doc m%"
flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running. Deprecated: Flocker is deprecated and the in-tree flocker type is no longer supported.
"%
     | optional,
  gcePersistentDisk
     | gCEPersistentDiskVolumeSource
     | doc m%"
gcePersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Deprecated: GCEPersistentDisk is deprecated. All operations for the in-tree gcePersistentDisk type are redirected to the pd.csi.storage.gke.io CSI driver. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%
     | optional,
  gitRepo
     | gitRepoVolumeSource
     | doc m%"
gitRepo represents a git repository at a particular revision. Deprecated: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
"%
     | optional,
  glusterfs
     | glusterfsVolumeSource
     | doc m%"
glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. Deprecated: Glusterfs is deprecated and the in-tree glusterfs type is no longer supported. More info: https://examples.k8s.io/volumes/glusterfs/README.md
"%
     | optional,
  hostPath
     | hostPathVolumeSource
     | doc m%"
hostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
"%
     | optional,
  image
     | imageVolumeSource
     | doc m%"
image represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine. The volume is resolved at pod startup depending on which PullPolicy value is provided:
- Always: the kubelet always attempts to pull the reference. Container creation will fail If the pull fails. - Never: the kubelet never pulls the reference and only uses a local image or artifact. Container creation will fail if the reference isn't present. - IfNotPresent: the kubelet pulls if the reference isn't already present on disk. Container creation will fail if the reference isn't present and the pull fails.
The volume gets re-resolved if the pod gets deleted and recreated, which means that new remote content will become available on pod recreation. A failure to resolve or pull the image during pod startup will block containers from starting and may add significant latency. Failures will be retried using normal volume backoff and will be reported on the pod reason and message. The types of objects that may be mounted by this volume are defined by the container runtime implementation on a host machine and at minimum must include all valid types supported by the container image field. The OCI object gets mounted in a single directory (spec.containers[*].volumeMounts.mountPath) by merging the manifest layers in the same way as for container images. The volume will be mounted read-only (ro) and non-executable files (noexec). Sub path mounts for containers are not supported (spec.containers[*].volumeMounts.subpath) before 1.33. The field spec.securityContext.fsGroupChangePolicy has no effect on this volume type.
"%
     | optional,
  iscsi
     | iSCSIVolumeSource
     | doc m%"
iscsi represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
"%
     | optional,
  name
     | String
     | doc m%"
name of the volume. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%,
  nfs
     | nFSVolumeSource
     | doc m%"
nfs represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%
     | optional,
  persistentVolumeClaim
     | persistentVolumeClaimVolumeSource
     | doc m%"
persistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
"%
     | optional,
  photonPersistentDisk
     | photonPersistentDiskVolumeSource
     | doc m%"
photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine. Deprecated: PhotonPersistentDisk is deprecated and the in-tree photonPersistentDisk type is no longer supported.
"%
     | optional,
  portworxVolume
     | portworxVolumeSource
     | doc m%"
portworxVolume represents a portworx volume attached and mounted on kubelets host machine. Deprecated: PortworxVolume is deprecated. All operations for the in-tree portworxVolume type are redirected to the pxd.portworx.com CSI driver when the CSIMigrationPortworx feature-gate is on.
"%
     | optional,
  projected
     | projectedVolumeSource
     | doc "projected items for all in one resources secrets, configmaps, and downward API"
     | optional,
  quobyte
     | quobyteVolumeSource
     | doc m%"
quobyte represents a Quobyte mount on the host that shares a pod's lifetime. Deprecated: Quobyte is deprecated and the in-tree quobyte type is no longer supported.
"%
     | optional,
  rbd
     | rBDVolumeSource
     | doc m%"
rbd represents a Rados Block Device mount on the host that shares a pod's lifetime. Deprecated: RBD is deprecated and the in-tree rbd type is no longer supported. More info: https://examples.k8s.io/volumes/rbd/README.md
"%
     | optional,
  scaleIO
     | scaleIOVolumeSource
     | doc m%"
scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes. Deprecated: ScaleIO is deprecated and the in-tree scaleIO type is no longer supported.
"%
     | optional,
  secret
     | secretVolumeSource
     | doc m%"
secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
"%
     | optional,
  storageos
     | storageOSVolumeSource
     | doc m%"
storageOS represents a StorageOS volume attached and mounted on Kubernetes nodes. Deprecated: StorageOS is deprecated and the in-tree storageos type is no longer supported.
"%
     | optional,
  vsphereVolume
     | vsphereVirtualDiskVolumeSource
     | doc m%"
vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine. Deprecated: VsphereVolume is deprecated. All operations for the in-tree vsphereVolume type are redirected to the csi.vsphere.vmware.com CSI driver.
"%
     | optional
}