# Module: k8s.io.v1

# AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
{
  readOnly
     | Bool
     | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
     | optional,
  secretName
     | String
     | doc m%"
secretName is the  name of secret that contains Azure Storage Account Name and Key
"%,
  shareName
     | String
     | doc "shareName is the azure share Name"
}