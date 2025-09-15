let v1Module = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in

# Module: apiextensions.crossplane.io.v1alpha1

{
  apiVersion
     | String
     | doc m%"
APIVersion defines the versioned schema of this representation of an object.
Servers should convert recognized schemas to the latest internal value, and
may reject unrecognized values.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
     | optional,
  kind
     | String
     | doc m%"
Kind is a string value representing the REST resource this object represents.
Servers may infer this from the endpoint the client submits requests to.
Cannot be updated.
In CamelCase.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
     | optional,
  metadata
     | v1Module.ObjectMeta
     | optional,
  spec
     | {
    connectionDetails
       | Array {
      description
         | String
         | doc "Description of how the key is used.",
      name
         | String
         | doc "Name of the key."
    }
       | doc "ConnectionDetails is an array of connection detail keys and descriptions."
       | optional,
    conversion
       | {
      strategy
         | String
         | doc m%"
strategy specifies how custom resources are converted between versions. Allowed values are:
- `"None"`: The converter only change the apiVersion and would not touch any other field in the custom resource.
- `"Webhook"`: API Server will call to an external webhook to do the conversion. Additional information
  is needed for this option. This requires spec.preserveUnknownFields to be false, and spec.conversion.webhook to be set.
"%,
      webhook
         | {
        clientConfig
           | {
          caBundle
             | String
             | doc m%"
caBundle is a PEM encoded CA bundle which will be used to validate the webhook's server certificate.
If unspecified, system trust roots on the apiserver are used.
"%
             | optional,
          service
             | {
            name
               | String
               | doc m%"
name is the name of the service.
Required
"%,
            namespace
               | String
               | doc m%"
namespace is the namespace of the service.
Required
"%,
            path
               | String
               | doc "path is an optional URL path at which the webhook will be contacted."
               | optional,
            port
               | Number
               | doc m%"
port is an optional service port at which the webhook will be contacted.
`port` should be a valid port number (1-65535, inclusive).
Defaults to 443 for backward compatibility.
"%
               | optional
          }
             | doc m%"
service is a reference to the service for this webhook. Either
service or url must be specified.
If the webhook is running within the cluster, then you should use `service`.
"%
             | optional,
          url
             | String
             | doc m%"
url gives the location of the webhook, in standard URL form
(`scheme://host:port/path`). Exactly one of `url` or `service`
must be specified.
The `host` should not refer to a service running in the cluster; use
the `service` field instead. The host might be resolved via external
DNS in some apiservers (e.g., `kube-apiserver` cannot resolve
in-cluster DNS as that would be a layering violation). `host` may
also be an IP address.
Please note that using `localhost` or `127.0.0.1` as a `host` is
risky unless you take great care to run this webhook on all hosts
which run an apiserver which might need to make calls to this
webhook. Such installs are likely to be non-portable, i.e., not easy
to turn up in a new cluster.
The scheme must be "https"; the URL must begin with "https://".
A path is optional, and if present may be any string permissible in
a URL. You may use the path to pass an arbitrary string to the
webhook, for example, a cluster identifier.
Attempting to use a user or basic auth e.g. "user:password@" is not
allowed. Fragments ("#...") and query parameters ("?...") are not
allowed, either.
"%
             | optional
        }
           | doc m%"
clientConfig is the instructions for how to call the webhook if strategy is `Webhook`.
"%
           | optional,
        conversionReviewVersions
           | Array String
           | doc m%"
conversionReviewVersions is an ordered list of preferred `ConversionReview`
versions the Webhook expects. The API server will use the first version in
the list which it supports. If none of the versions specified in this list
are supported by API server, conversion will fail for the custom resource.
If a persisted Webhook configuration specifies allowed versions and does not
include any versions known to the API Server, calls to the webhook will fail.
"%
      }
         | doc m%"
webhook describes how to call the conversion webhook. Required when `strategy` is set to `"Webhook"`.
"%
         | optional
    }
       | doc "Conversion defines conversion settings for the CRD."
       | optional,
    group
       | String
       | doc m%"
Group is the API group of the defined custom resource.
The custom resources are served under `/apis/<group>/...`.
Must match the name of the CustomResourceDefinition (in the form `<names.plural>.<group>`).
"%,
    names
       | {
      categories
         | Array String
         | doc m%"
categories is a list of grouped resources this custom resource belongs to (e.g. 'all').
This is published in API discovery documents, and used by clients to support invocations like
`kubectl get all`.
"%
         | optional,
      kind
         | String
         | doc m%"
kind is the serialized kind of the resource. It is normally CamelCase and singular.
Custom resource instances will use this value as the `kind` attribute in API calls.
"%,
      listKind
         | String
         | doc m%"
listKind is the serialized kind of the list for this resource. Defaults to "`kind`List".
"%
         | optional,
      plural
         | String
         | doc m%"
plural is the plural name of the resource to serve.
The custom resources are served under `/apis/<group>/<version>/.../<plural>`.
Must match the name of the CustomResourceDefinition (in the form `<names.plural>.<group>`).
Must be all lowercase.
"%,
      shortNames
         | Array String
         | doc m%"
shortNames are short names for the resource, exposed in API discovery documents,
and used by clients to support invocations like `kubectl get <shortname>`.
It must be all lowercase.
"%
         | optional,
      singular
         | String
         | doc m%"
singular is the singular name of the resource. It must be all lowercase. Defaults to lowercased `kind`.
"%
         | optional
    }
       | doc "Names specify the resource and kind names for the custom resource.",
    preserveUnknownFields
       | Bool
       | doc m%"
PreserveUnknownFields indicates that object fields which are not specified
in the OpenAPI schema should be preserved when persisting to storage.
apiVersion, kind, metadata and known fields inside metadata are always preserved.
This field is deprecated in favor of setting `x-preserve-unknown-fields` to true in `spec.versions[*].schema.openAPIV3Schema`.
See https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#field-pruning for details.
"%
       | optional,
    scope
       | String
       | doc m%"
Scope indicates whether the defined custom resource is cluster- or namespace-scoped.
Allowed values are `Cluster` and `Namespaced`.
"%
       = "Namespaced",
    state
       | String
       | doc "State toggles whether the underlying CRD is created or not."
       = "Inactive",
    versions
       | Array {
      additionalPrinterColumns
         | Array {
        description
           | String
           | doc "description is a human readable description of this column."
           | optional,
        format
           | String
           | doc m%"
format is an optional OpenAPI type definition for this column. The 'name' format is applied
to the primary identifier column to assist in clients identifying column is the resource name.
See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for details.
"%
           | optional,
        jsonPath
           | String
           | doc m%"
jsonPath is a simple JSON path (i.e. with array notation) which is evaluated against
each custom resource to produce the value for this column.
"%,
        name
           | String
           | doc "name is a human readable name for the column.",
        "priority"
           | Number
           | doc m%"
priority is an integer defining the relative importance of this column compared to others. Lower
numbers are considered higher priority. Columns that may be omitted in limited space scenarios
should be given a priority greater than 0.
"%
           | optional,
        type
           | String
           | doc m%"
type is an OpenAPI type definition for this column.
See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for details.
"%
      }
         | doc m%"
AdditionalPrinterColumns specifies additional columns returned in Table output.
See https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables for details.
If no columns are specified, a single column displaying the age of the custom resource is used.
"%
         | optional,
      deprecated
         | Bool
         | doc m%"
Deprecated indicates this version of the custom resource API is deprecated.
When set to true, API requests to this version receive a warning header in the server response.
Defaults to false.
"%
         | optional,
      deprecationWarning
         | String
         | doc m%"
DeprecationWarning overrides the default warning returned to API clients.
May only be set when `deprecated` is true.
The default warning indicates this version is deprecated and recommends use
of the newest served version of equal or greater stability, if one exists.
"%
         | optional,
      name
         | String
         | doc m%"
Name is the version name, e.g. “v1”, “v2beta1”, etc.
The custom resources are served under this version at `/apis/<group>/<version>/...` if `served` is true.
"%,
      schema
         | {
        openAPIV3Schema
           | {
        }
           | doc m%"
OpenAPIV3Schema is the OpenAPI v3 schema to use for validation and
pruning.
"%
           | optional
      }
         | doc m%"
Schema describes the schema used for validation, pruning, and defaulting of this version of the custom resource.
"%
         | optional,
      selectableFields
         | Array {
        jsonPath
           | String
           | doc m%"
jsonPath is a simple JSON path which is evaluated against each custom resource to produce a
field selector value.
Only JSON paths without the array notation are allowed.
Must point to a field of type string, boolean or integer. Types with enum values
and strings with formats are allowed.
If jsonPath refers to absent field in a resource, the jsonPath evaluates to an empty string.
Must not point to metdata fields.
Required.
"%
      }
         | doc m%"
SelectableFields specifies paths to fields that may be used as field selectors.
A maximum of 8 selectable fields are allowed.
See https://kubernetes.io/docs/concepts/overview/working-with-objects/field-selectors
"%
         | optional,
      served
         | Bool
         | doc "Served is a flag enabling/disabling this version from being served via REST APIs",
      storage
         | Bool
         | doc m%"
Storage indicates this version should be used when persisting custom resources to storage.
There must be exactly one version with storage=true.
"%,
      subresources
         | {
        scale
           | {
          labelSelectorPath
             | String
             | doc m%"
labelSelectorPath defines the JSON path inside of a custom resource that corresponds to Scale `status.selector`.
Only JSON paths without the array notation are allowed.
Must be a JSON Path under `.status` or `.spec`.
Must be set to work with HorizontalPodAutoscaler.
The field pointed by this JSON path must be a string field (not a complex selector struct)
which contains a serialized label selector in string form.
More info: https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions#scale-subresource
If there is no value under the given path in the custom resource, the `status.selector` value in the `/scale`
subresource will default to the empty string.
"%
             | optional,
          specReplicasPath
             | String
             | doc m%"
specReplicasPath defines the JSON path inside of a custom resource that corresponds to Scale `spec.replicas`.
Only JSON paths without the array notation are allowed.
Must be a JSON Path under `.spec`.
If there is no value under the given path in the custom resource, the `/scale` subresource will return an error on GET.
"%,
          statusReplicasPath
             | String
             | doc m%"
statusReplicasPath defines the JSON path inside of a custom resource that corresponds to Scale `status.replicas`.
Only JSON paths without the array notation are allowed.
Must be a JSON Path under `.status`.
If there is no value under the given path in the custom resource, the `status.replicas` value in the `/scale` subresource
will default to 0.
"%
        }
           | doc m%"
scale indicates the custom resource should serve a `/scale` subresource that returns an `autoscaling/v1` Scale object.
"%
           | optional,
        status
           | {
        }
           | doc m%"
status indicates the custom resource should serve a `/status` subresource.
When enabled:
1. requests to the custom resource primary endpoint ignore changes to the `status` stanza of the object.
2. requests to the custom resource `/status` subresource ignore changes to anything other than the `status` stanza of the object.
"%
           | optional
      }
         | doc m%"
Subresources specify what subresources this version of the defined custom resource have.
"%
         | optional
    }
       | doc m%"
Versions is the list of all API versions of the defined custom resource.
Version names are used to compute the order in which served versions are listed in API discovery.
If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered
lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version),
then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first
by GA > beta > alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing
major version, then minor version. An example sorted list of versions:
v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
"%
  }
     | doc m%"
ManagedResourceDefinitionSpec specifies the desired state of the resource definition.
"%
     | optional,
  status
     | {
    conditions
       | Array {
      lastTransitionTime
         | String
         | doc m%"
LastTransitionTime is the last time this condition transitioned from one
status to another.
"%,
      message
         | String
         | doc m%"
A Message containing details about this condition's last transition from
one status to another, if any.
"%
         | optional,
      observedGeneration
         | Number
         | doc m%"
ObservedGeneration represents the .metadata.generation that the condition was set based upon.
For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date
with respect to the current state of the instance.
"%
         | optional,
      reason
         | String
         | doc "A Reason for this condition's last transition from one status to another.",
      status
         | String
         | doc "Status of this condition; is it currently True, False, or Unknown?",
      type
         | String
         | doc m%"
Type of this condition. At most one of each condition type may apply to
a resource at any point in time.
"%
    }
       | doc "Conditions of the resource."
       | optional
  }
     | doc m%"
ManagedResourceDefinitionStatus shows the observed state of the resource definition.
"%
     | optional
}