# Module: k8s.io.v1.azurefilepersistentvolumesource

{
  # AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
  AzureFilePersistentVolumeSource = {
    readOnly | optional | Bool | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%,
    secretName | optional | String | doc m%"
secretName is the name of secret that contains Azure Storage Account Name and Key
"%,
    secretNamespace | optional | String | doc m%"
secretNamespace is the namespace of the secret that contains Azure Storage Account Name and Key default is the same as the Pod
"%,
    shareName | optional | String | doc "shareName is the azure Share Name",
  },
}
