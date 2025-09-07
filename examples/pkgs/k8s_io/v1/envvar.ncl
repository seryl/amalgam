let envVarSource = import "./EnvVarSource.ncl" in

# Module: k8s.io.v1

# EnvVar represents an environment variable present in a Container.
{
  name
     | String
     | doc "Name of the environment variable. Must be a C_IDENTIFIER.",
  value
     | String
     | doc m%"
Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
"%
     | optional,
  valueFrom
     | envVarSource
     | doc m%"
Source for the environment variable's value. Cannot be used if value is not empty.
"%
     | optional
}