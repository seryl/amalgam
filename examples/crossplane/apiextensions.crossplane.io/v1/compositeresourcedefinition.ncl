# Module: compositeresourcedefinition.apiextensions.crossplane.io

let k8s_io_objectmeta = import "../../../k8s_io/v1/objectmeta.ncl" in

{
  CompositeResourceDefinition = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object.
Servers should convert recognized schemas to the latest internal value, and
may reject unrecognized values.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents.
Servers may infer this from the endpoint the client submits requests to.
Cannot be updated.
In CamelCase.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | k8s_io_objectmeta.ObjectMeta,
    spec | optional | {
      claimNames | optional | {
        categories | optional | Array String | doc m%"
categories is a list of grouped resources this custom resource belongs to (e.g. 'all').
This is published in API discovery documents, and used by clients to support invocations like
`kubectl get all`.
"%,
        kind | String | doc m%"
kind is the serialized kind of the resource. It is normally CamelCase and singular.
Custom resource instances will use this value as the `kind` attribute in API calls.
"%,
        listKind | optional | String | doc m%"
listKind is the serialized kind of the list for this resource. Defaults to "`kind`List".
"%,
        plural | String | doc m%"
plural is the plural name of the resource to serve.
The custom resources are served under `/apis/<group>/<version>/.../<plural>`.
Must match the name of the CustomResourceDefinition (in the form `<names.plural>.<group>`).
Must be all lowercase.
"%,
        shortNames | optional | Array String | doc m%"
shortNames are short names for the resource, exposed in API discovery documents,
and used by clients to support invocations like `kubectl get <shortname>`.
It must be all lowercase.
"%,
        singular | optional | String | doc m%"
singular is the singular name of the resource. It must be all lowercase. Defaults to lowercased `kind`.
"%,
      } | doc m%"
ClaimNames specifies the names of an optional composite resource claim.
When claim names are specified Crossplane will create a namespaced
'composite resource claim' CRD that corresponds to the defined composite
resource. This composite resource claim acts as a namespaced proxy for
the composite resource; creating, updating, or deleting the claim will
create, update, or delete a corresponding composite resource. You may add
claim names to an existing CompositeResourceDefinition, but they cannot
be changed or removed once they have been set.
"%,
      connectionSecretKeys | optional | Array String | doc m%"
ConnectionSecretKeys is the list of connection secret keys the
defined XR can publish. If the list is empty, all keys will be
published. If the list isn't empty, any connection secret keys that
don't appear in the list will be filtered out. Only LegacyCluster XRs
support connection secrets.
"%,
      conversion | optional | {
        strategy | String | doc m%"
strategy specifies how custom resources are converted between versions. Allowed values are:
- `"None"`: The converter only change the apiVersion and would not touch any other field in the custom resource.
- `"Webhook"`: API Server will call to an external webhook to do the conversion. Additional information
  is needed for this option. This requires spec.preserveUnknownFields to be false, and spec.conversion.webhook to be set.
"%,
        webhook | optional | {
          clientConfig | optional | {
            caBundle | optional | String | doc m%"
caBundle is a PEM encoded CA bundle which will be used to validate the webhook's server certificate.
If unspecified, system trust roots on the apiserver are used.
"%,
            service | optional | {
              name | String | doc m%"
name is the name of the service.
Required
"%,
              namespace | String | doc m%"
namespace is the namespace of the service.
Required
"%,
              path | optional | String | doc "path is an optional URL path at which the webhook will be contacted.",
              port | optional | Number | doc m%"
port is an optional service port at which the webhook will be contacted.
`port` should be a valid port number (1-65535, inclusive).
Defaults to 443 for backward compatibility.
"%,
            } | doc m%"
service is a reference to the service for this webhook. Either
service or url must be specified.

If the webhook is running within the cluster, then you should use `service`.
"%,
            url | optional | String | doc m%"
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
"%,
          } | doc m%"
clientConfig is the instructions for how to call the webhook if strategy is `Webhook`.
"%,
          conversionReviewVersions | Array String | doc m%"
conversionReviewVersions is an ordered list of preferred `ConversionReview`
versions the Webhook expects. The API server will use the first version in
the list which it supports. If none of the versions specified in this list
are supported by API server, conversion will fail for the custom resource.
If a persisted Webhook configuration specifies allowed versions and does not
include any versions known to the API Server, calls to the webhook will fail.
"%,
        } | doc m%"
webhook describes how to call the conversion webhook. Required when `strategy` is set to `"Webhook"`.
"%,
      } | doc "Conversion defines all conversion settings for the defined Composite resource.",
      defaultCompositeDeletePolicy | String | doc m%"
DefaultCompositeDeletePolicy is the policy used when deleting the Composite
that is associated with the Claim if no policy has been specified.
"% | default = "Background",
      defaultCompositionRef | optional | {
        name | String | doc "Name of the Composition.",
      } | doc m%"
DefaultCompositionRef refers to the Composition resource that will be used
in case no composition selector is given.
"%,
      defaultCompositionUpdatePolicy | String | doc m%"
DefaultCompositionUpdatePolicy is the policy used when updating composites after a new
Composition Revision has been created if no policy has been specified on the composite.
"% | default = "Automatic",
      enforcedCompositionRef | optional | {
        name | String | doc "Name of the Composition.",
      } | doc m%"
EnforcedCompositionRef refers to the Composition resource that will be used
by all composite instances whose schema is defined by this definition.
"%,
      group | String | doc m%"
Group specifies the API group of the defined composite resource.
Composite resources are served under `/apis/<group>/...`. Must match the
name of the XRD (in the form `<names.plural>.<group>`).
"%,
      metadata | optional | {
        annotations | optional | {
        } | doc m%"
Annotations is an unstructured key value map stored with a resource that may be
set by external tools to store and retrieve arbitrary metadata. They are not
queryable and should be preserved when modifying objects.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations
"%,
        labels | optional | {
        } | doc m%"
Map of string keys and values that can be used to organize and categorize
(scope and select) objects. May match selectors of replication controllers
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels
and services.
These labels are added to the composite resource and claim CRD's in addition
to any labels defined by `CompositionResourceDefinition` `metadata.labels`.
"%,
      } | doc m%"
Metadata specifies the desired metadata for the defined composite resource and claim CRD's.
"%,
      names | {
        categories | optional | Array String | doc m%"
categories is a list of grouped resources this custom resource belongs to (e.g. 'all').
This is published in API discovery documents, and used by clients to support invocations like
`kubectl get all`.
"%,
        kind | String | doc m%"
kind is the serialized kind of the resource. It is normally CamelCase and singular.
Custom resource instances will use this value as the `kind` attribute in API calls.
"%,
        listKind | optional | String | doc m%"
listKind is the serialized kind of the list for this resource. Defaults to "`kind`List".
"%,
        plural | String | doc m%"
plural is the plural name of the resource to serve.
The custom resources are served under `/apis/<group>/<version>/.../<plural>`.
Must match the name of the CustomResourceDefinition (in the form `<names.plural>.<group>`).
Must be all lowercase.
"%,
        shortNames | optional | Array String | doc m%"
shortNames are short names for the resource, exposed in API discovery documents,
and used by clients to support invocations like `kubectl get <shortname>`.
It must be all lowercase.
"%,
        singular | optional | String | doc m%"
singular is the singular name of the resource. It must be all lowercase. Defaults to lowercased `kind`.
"%,
      } | doc m%"
Names specifies the resource and kind names of the defined composite
resource.
"%,
      scope | String | doc m%"
Scope of the defined composite resource. Namespaced composite resources
are scoped to a single namespace. Cluster scoped composite resource exist
outside the scope of any namespace. Neither can be claimed. Legacy
cluster scoped composite resources are cluster scoped resources that can
be claimed.
"% | default = "LegacyCluster",
      versions | Array {
        additionalPrinterColumns | optional | Array {
          description | optional | String | doc "description is a human readable description of this column.",
          format | optional | String | doc m%"
format is an optional OpenAPI type definition for this column. The 'name' format is applied
to the primary identifier column to assist in clients identifying column is the resource name.
See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for details.
"%,
          jsonPath | String | doc m%"
jsonPath is a simple JSON path (i.e. with array notation) which is evaluated against
each custom resource to produce the value for this column.
"%,
          name | String | doc "name is a human readable name for the column.",
          priority | optional | Number | doc m%"
priority is an integer defining the relative importance of this column compared to others. Lower
numbers are considered higher priority. Columns that may be omitted in limited space scenarios
should be given a priority greater than 0.
"%,
          type | String | doc m%"
type is an OpenAPI type definition for this column.
See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for details.
"%,
        } | doc m%"
AdditionalPrinterColumns specifies additional columns returned in Table
output. If no columns are specified, a single column displaying the age
of the custom resource is used. See the following link for details:
https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables
"%,
        deprecated | optional | Bool | doc m%"
The deprecated field specifies that this version is deprecated and should
not be used.
"%,
        deprecationWarning | optional | String | doc m%"
DeprecationWarning specifies the message that should be shown to the user
when using this version.
"%,
        name | String | doc m%"
Name of this version, e.g. “v1”, “v2beta1”, etc. Composite resources are
served under this version at `/apis/<group>/<version>/...` if `served` is
true.
"%,
        referenceable | Bool | doc m%"
Referenceable specifies that this version may be referenced by a
Composition in order to configure which resources an XR may be composed
of. Exactly one version must be marked as referenceable; all Compositions
must target only the referenceable version. The referenceable version
must be served. It's mapped to the CRD's `spec.versions[*].storage` field.
"%,
        schema | optional | {
          openAPIV3Schema | optional | {
          } | doc m%"
OpenAPIV3Schema is the OpenAPI v3 schema to use for validation and
pruning.
"%,
        } | doc m%"
Schema describes the schema used for validation, pruning, and defaulting
of this version of the defined composite resource. Fields required by all
composite resources will be injected into this schema automatically, and
will override equivalently named fields in this schema. Omitting this
schema results in a schema that contains only the fields required by all
composite resources.
"%,
        served | Bool | doc "Served specifies that this version should be served via REST APIs.",
      } | doc m%"
Versions is the list of all API versions of the defined composite
resource. Version names are used to compute the order in which served
versions are listed in API discovery. If the version string is
"kube-like", it will sort above non "kube-like" version strings, which
are ordered lexicographically. "Kube-like" versions start with a "v",
then are followed by a number (the major version), then optionally the
string "alpha" or "beta" and another number (the minor version). These
are sorted first by GA > beta > alpha (where GA is a version with no
suffix such as beta or alpha), and then by comparing major version, then
minor version. An example sorted list of versions: v10, v2, v1, v11beta2,
v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
"%,
    } | doc "CompositeResourceDefinitionSpec specifies the desired state of the definition.",
    status | optional | {
      conditions | optional | Array {
        lastTransitionTime | String | doc m%"
LastTransitionTime is the last time this condition transitioned from one
status to another.
"%,
        message | optional | String | doc m%"
A Message containing details about this condition's last transition from
one status to another, if any.
"%,
        observedGeneration | optional | Number | doc m%"
ObservedGeneration represents the .metadata.generation that the condition was set based upon.
For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date
with respect to the current state of the instance.
"%,
        reason | String | doc "A Reason for this condition's last transition from one status to another.",
        status | String | doc "Status of this condition; is it currently True, False, or Unknown?",
        type | String | doc m%"
Type of this condition. At most one of each condition type may apply to
a resource at any point in time.
"%,
      } | doc "Conditions of the resource.",
      controllers | optional | {
        compositeResourceClaimType | optional | {
          apiVersion | String | doc "APIVersion of the type.",
          kind | String | doc "Kind of the type.",
        } | doc m%"
The CompositeResourceClaimTypeRef is the type of composite resource claim
that Crossplane is currently reconciling for this definition. Its version
will eventually become consistent with the definition's referenceable
version. Note that clients may interact with any served type; this is
simply the type that Crossplane interacts with.
"%,
        compositeResourceType | optional | {
          apiVersion | String | doc "APIVersion of the type.",
          kind | String | doc "Kind of the type.",
        } | doc m%"
The CompositeResourceTypeRef is the type of composite resource that
Crossplane is currently reconciling for this definition. Its version will
eventually become consistent with the definition's referenceable version.
Note that clients may interact with any served type; this is simply the
type that Crossplane interacts with.
"%,
      } | doc m%"
Controllers represents the status of the controllers that power this
composite resource definition.
"%,
    } | doc "CompositeResourceDefinitionStatus shows the observed state of the definition.",
  },
}
