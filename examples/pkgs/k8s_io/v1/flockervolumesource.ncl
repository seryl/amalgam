# Module: k8s.io.v1

# Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName and datasetUUID should be set. Flocker volumes do not support ownership management or SELinux relabeling.
{
  datasetName
     | String
     | doc m%"
datasetName is Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated
"%
     | optional,
  datasetUUID
     | String
     | doc m%"
datasetUUID is the UUID of the dataset. This is unique identifier of a Flocker dataset
"%
     | optional
}