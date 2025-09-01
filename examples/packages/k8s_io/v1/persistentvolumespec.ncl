# Module: k8s.io.v1.persistentvolumespec

let gcepersistentdiskvolumesource = import "./gcepersistentdiskvolumesource.ncl" in
let hostpathvolumesource = import "./hostpathvolumesource.ncl" in
let flexpersistentvolumesource = import "./flexpersistentvolumesource.ncl" in
let flockervolumesource = import "./flockervolumesource.ncl" in
let cephfspersistentvolumesource = import "./cephfspersistentvolumesource.ncl" in
let nfsvolumesource = import "./nfsvolumesource.ncl" in
let quobytevolumesource = import "./quobytevolumesource.ncl" in
let objectreference = import "./objectreference.ncl" in
let cinderpersistentvolumesource = import "./cinderpersistentvolumesource.ncl" in
let scaleiopersistentvolumesource = import "./scaleiopersistentvolumesource.ncl" in
let glusterfspersistentvolumesource = import "./glusterfspersistentvolumesource.ncl" in
let storageospersistentvolumesource = import "./storageospersistentvolumesource.ncl" in
let awselasticblockstorevolumesource = import "./awselasticblockstorevolumesource.ncl" in
let portworxvolumesource = import "./portworxvolumesource.ncl" in
let vspherevirtualdiskvolumesource = import "./vspherevirtualdiskvolumesource.ncl" in
let volumenodeaffinity = import "./volumenodeaffinity.ncl" in
let csipersistentvolumesource = import "./csipersistentvolumesource.ncl" in
let azurediskvolumesource = import "./azurediskvolumesource.ncl" in
let azurefilepersistentvolumesource = import "./azurefilepersistentvolumesource.ncl" in
let localvolumesource = import "./localvolumesource.ncl" in
let rbdpersistentvolumesource = import "./rbdpersistentvolumesource.ncl" in
let iscsipersistentvolumesource = import "./iscsipersistentvolumesource.ncl" in
let photonpersistentdiskvolumesource = import "./photonpersistentdiskvolumesource.ncl" in
let fcvolumesource = import "./fcvolumesource.ncl" in

{
  # PersistentVolumeSpec is the specification of a persistent volume.
  PersistentVolumeSpec = {
    accessModes | optional | Array String | doc m%"
accessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes
"%,
    awsElasticBlockStore | optional | awselasticblockstorevolumesource.AWSElasticBlockStoreVolumeSource | doc m%"
awsElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%,
    azureDisk | optional | azurediskvolumesource.AzureDiskVolumeSource | doc m%"
azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
"%,
    azureFile | optional | azurefilepersistentvolumesource.AzureFilePersistentVolumeSource | doc m%"
azureFile represents an Azure File Service mount on the host and bind mount to the pod.
"%,
    capacity | optional | { .. } | doc m%"
capacity is the description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
"%,
    cephfs | optional | cephfspersistentvolumesource.CephFSPersistentVolumeSource | doc "cephFS represents a Ceph FS mount on the host that shares a pod's lifetime",
    cinder | optional | cinderpersistentvolumesource.CinderPersistentVolumeSource | doc m%"
cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%,
    claimRef | optional | objectreference.ObjectReference | doc m%"
claimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-nil when bound. claim.VolumeName is the authoritative bind between PV and PVC. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding
"%,
    csi | optional | csipersistentvolumesource.CSIPersistentVolumeSource | doc "csi represents storage that is handled by an external CSI driver (Beta feature).",
    fc | optional | fcvolumesource.FCVolumeSource | doc m%"
fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
"%,
    flexVolume | optional | flexpersistentvolumesource.FlexPersistentVolumeSource | doc m%"
flexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
"%,
    flocker | optional | flockervolumesource.FlockerVolumeSource | doc m%"
flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running
"%,
    gcePersistentDisk | optional | gcepersistentdiskvolumesource.GCEPersistentDiskVolumeSource | doc m%"
gcePersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%,
    glusterfs | optional | glusterfspersistentvolumesource.GlusterfsPersistentVolumeSource | doc m%"
glusterfs represents a Glusterfs volume that is attached to a host and exposed to the pod. Provisioned by an admin. More info: https://examples.k8s.io/volumes/glusterfs/README.md
"%,
    hostPath | optional | hostpathvolumesource.HostPathVolumeSource | doc m%"
hostPath represents a directory on the host. Provisioned by a developer or tester. This is useful for single-node development and testing only! On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
"%,
    iscsi | optional | iscsipersistentvolumesource.ISCSIPersistentVolumeSource | doc m%"
iscsi represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin.
"%,
    local | optional | localvolumesource.LocalVolumeSource | doc "local represents directly-attached storage with node affinity",
    mountOptions | optional | Array String | doc m%"
mountOptions is the list of mount options, e.g. ["ro", "soft"]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options
"%,
    nfs | optional | nfsvolumesource.NFSVolumeSource | doc m%"
nfs represents an NFS mount on the host. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%,
    nodeAffinity | optional | volumenodeaffinity.VolumeNodeAffinity | doc m%"
nodeAffinity defines constraints that limit what nodes this volume can be accessed from. This field influences the scheduling of pods that use this volume.
"%,
    persistentVolumeReclaimPolicy | optional | String | doc m%"
persistentVolumeReclaimPolicy defines what happens to a persistent volume when released from its claim. Valid options are Retain (default for manually created PersistentVolumes), Delete (default for dynamically provisioned PersistentVolumes), and Recycle (deprecated). Recycle must be supported by the volume plugin underlying this PersistentVolume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming
"%,
    photonPersistentDisk | optional | photonpersistentdiskvolumesource.PhotonPersistentDiskVolumeSource | doc m%"
photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
"%,
    portworxVolume | optional | portworxvolumesource.PortworxVolumeSource | doc m%"
portworxVolume represents a portworx volume attached and mounted on kubelets host machine
"%,
    quobyte | optional | quobytevolumesource.QuobyteVolumeSource | doc "quobyte represents a Quobyte mount on the host that shares a pod's lifetime",
    rbd | optional | rbdpersistentvolumesource.RBDPersistentVolumeSource | doc m%"
rbd represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
"%,
    scaleIO | optional | scaleiopersistentvolumesource.ScaleIOPersistentVolumeSource | doc m%"
scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
"%,
    storageClassName | optional | String | doc m%"
storageClassName is the name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass.
"%,
    storageos | optional | storageospersistentvolumesource.StorageOSPersistentVolumeSource | doc m%"
storageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod More info: https://examples.k8s.io/volumes/storageos/README.md
"%,
    volumeAttributesClassName | optional | String | doc m%"
Name of VolumeAttributesClass to which this persistent volume belongs. Empty value is not allowed. When this field is not set, it indicates that this volume does not belong to any VolumeAttributesClass. This field is mutable and can be changed by the CSI driver after a volume has been updated successfully to a new class. For an unbound PersistentVolume, the volumeAttributesClassName will be matched with unbound PersistentVolumeClaims during the binding process. This is a beta field and requires enabling VolumeAttributesClass feature (off by default).
"%,
    volumeMode | optional | String | doc m%"
volumeMode defines if a volume is intended to be used with a formatted filesystem or to remain in raw block state. Value of Filesystem is implied when not included in spec.
"%,
    vsphereVolume | optional | vspherevirtualdiskvolumesource.VsphereVirtualDiskVolumeSource | doc m%"
vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
"%,
  },
}
