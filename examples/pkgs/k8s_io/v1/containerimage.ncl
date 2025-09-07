# Module: k8s.io.v1

# Describe a container image
{
  names
     | Array String
     | doc m%"
Names by which this image is known. e.g. ["kubernetes.example/hyperkube:v1.0.7", "cloud-vendor.registry.example/cloud-vendor/hyperkube:v1.0.7"]
"%
     | optional,
  sizeBytes
     | Number
     | doc "The size of the image in bytes."
     | optional
}