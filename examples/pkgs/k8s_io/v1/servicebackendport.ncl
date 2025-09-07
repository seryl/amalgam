# Module: k8s.io.v1

# ServiceBackendPort is the service port being referenced.
{
  name
     | String
     | doc m%"
name is the name of the port on the Service. This is a mutually exclusive setting with "Number".
"%
     | optional,
  number
     | Number
     | doc m%"
number is the numerical port number (e.g. 80) on the Service. This is a mutually exclusive setting with "Name".
"%
     | optional
}