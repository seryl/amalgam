# ServiceBackendPort is the service port being referenced.
{
  name | optional | String | doc m%"
name is the name of the port on the Service. This is a mutually exclusive setting with "Number".
"%,
  number | optional | Number | doc m%"
number is the numerical port number (e.g. 80) on the Service. This is a mutually exclusive setting with "Name".
"%,
}
