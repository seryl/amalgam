# Module: composition.apiextensions.crossplane.io

let k8s_io_objectmeta = import "k8s_io" in

{
  Composition = {
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
      compositeTypeRef | {
        apiVersion | String | doc "APIVersion of the type.",
        kind | String | doc "Kind of the type.",
      } | doc m%"
CompositeTypeRef specifies the type of composite resource that this
composition is compatible with.
"%,
      environment | optional | {
        defaultData | optional | {
        } | doc m%"
DefaultData statically defines the initial state of the environment.
It has the same schema-less structure as the data field in
environment configs.
It is overwritten by the selected environment configs.
"%,
        environmentConfigs | optional | Array {
          ref | optional | {
            name | String | doc "The name of the object.",
          } | doc m%"
Ref is a named reference to a single EnvironmentConfig.
Either Ref or Selector is required.
"%,
          selector | optional | {
            matchLabels | optional | Array {
              fromFieldPathPolicy | String | doc m%"
FromFieldPathPolicy specifies the policy for the valueFromFieldPath.
The default is Required, meaning that an error will be returned if the
field is not found in the composite resource.
Optional means that if the field is not found in the composite resource,
that label pair will just be skipped. N.B. other specified label
matchers will still be used to retrieve the desired
environment config, if any.
"% | default = "Required",
              key | String | doc "Key of the label to match.",
              type | String | doc "Type specifies where the value for a label comes from." | default = "FromCompositeFieldPath",
              value | optional | String | doc "Value specifies a literal label value.",
              valueFromFieldPath | optional | String | doc "ValueFromFieldPath specifies the field path to look for the label value.",
            } | doc "MatchLabels ensures an object with matching labels is selected.",
            maxMatch | optional | Number | doc m%"
MaxMatch specifies the number of extracted EnvironmentConfigs in Multiple mode, extracts all if nil.
"%,
            minMatch | optional | Number | doc m%"
MinMatch specifies the required minimum of extracted EnvironmentConfigs in Multiple mode.
"%,
            mode | String | doc "Mode specifies retrieval strategy: \"Single\" or \"Multiple\"." | default = "Single",
            sortByFieldPath | String | doc m%"
SortByFieldPath is the path to the field based on which list of EnvironmentConfigs is alphabetically sorted.
"% | default = "metadata.name",
          } | doc "Selector selects EnvironmentConfig(s) via labels.",
          type | String | doc m%"
Type specifies the way the EnvironmentConfig is selected.
Default is `Reference`
"% | default = "Reference",
        } | doc m%"
EnvironmentConfigs selects a list of `EnvironmentConfig`s. The resolved
resources are stored in the composite resource at
`spec.environmentConfigRefs` and is only updated if it is null.


The list of references is used to compute an in-memory environment at
compose time. The data of all object is merged in the order they are
listed, meaning the values of EnvironmentConfigs with a larger index take
priority over ones with smaller indices.


The computed environment can be accessed in a composition using
`FromEnvironmentFieldPath` and `CombineFromEnvironment` patches.
"%,
        patches | optional | Array {
          combine | optional | {
            strategy | String | doc m%"
Strategy defines the strategy to use to combine the input variable values.
Currently only string is supported.
"%,
            string | optional | {
              fmt | String | doc m%"
Format the input using a Go format string. See
https://golang.org/pkg/fmt/ for details.
"%,
            } | doc m%"
String declares that input variables should be combined into a single
string, using the relevant settings for formatting purposes.
"%,
            variables | Array {
              fromFieldPath | String | doc m%"
FromFieldPath is the path of the field on the source whose value is
to be used as input.
"%,
            } | doc m%"
Variables are the list of variables whose values will be retrieved and
combined.
"%,
          } | doc m%"
Combine is the patch configuration for a CombineFromComposite or
CombineToComposite patch.
"%,
          fromFieldPath | optional | String | doc m%"
FromFieldPath is the path of the field on the resource whose value is
to be used as input. Required when type is FromCompositeFieldPath or
ToCompositeFieldPath.
"%,
          policy | optional | {
            fromFieldPath | optional | String | doc m%"
FromFieldPath specifies how to patch from a field path. The default is
'Optional', which means the patch will be a no-op if the specified
fromFieldPath does not exist. Use 'Required' if the patch should fail if
the specified path does not exist.
"%,
            mergeOptions | optional | {
              appendSlice | optional | Bool | doc "Specifies that already existing elements in a merged slice should be preserved",
              keepMapValues | optional | Bool | doc "Specifies that already existing values in a merged map should be preserved",
            } | doc "MergeOptions Specifies merge options on a field path.",
          } | doc "Policy configures the specifics of patching behaviour.",
          toFieldPath | optional | String | doc m%"
ToFieldPath is the path of the field on the resource whose value will
be changed with the result of transforms. Leave empty if you'd like to
propagate to the same path as fromFieldPath.
"%,
          transforms | optional | Array {
            convert | optional | {
              format | optional | String | doc m%"
The expected input format.


* `quantity` - parses the input as a K8s [`resource.Quantity`](https://pkg.go.dev/k8s.io/apimachinery/pkg/api/resource#Quantity).
Only used during `string -> float64` conversions.
* `json` - parses the input as a JSON string.
Only used during `string -> object` or `string -> list` conversions.


If this property is null, the default conversion is applied.
"%,
              toType | String | doc "ToType is the type of the output of this transform.",
            } | doc "Convert is used to cast the input into the given output type.",
            map | optional | {
            } | doc "Map uses the input as a key in the given map and returns the value.",
            match | optional | {
              fallbackTo | String | doc "Determines to what value the transform should fallback if no pattern matches." | default = "Value",
              fallbackValue | optional | Dyn | doc m%"
The fallback value that should be returned by the transform if now pattern
matches.
"%,
              patterns | optional | Array {
                literal | optional | String | doc m%"
Literal exactly matches the input string (case sensitive).
Is required if `type` is `literal`.
"%,
                regexp | optional | String | doc m%"
Regexp to match against the input string.
Is required if `type` is `regexp`.
"%,
                result | Dyn | doc "The value that is used as result of the transform if the pattern matches.",
                type | String | doc m%"
Type specifies how the pattern matches the input.


* `literal` - the pattern value has to exactly match (case sensitive) the
input string. This is the default.


* `regexp` - the pattern treated as a regular expression against
which the input string is tested. Crossplane will throw an error if the
key is not a valid regexp.
"% | default = "literal",
              } | doc m%"
The patterns that should be tested against the input string.
Patterns are tested in order. The value of the first match is used as
result of this transform.
"%,
            } | doc "Match is a more complex version of Map that matches a list of patterns.",
            math | optional | {
              clampMax | optional | Number | doc "ClampMax makes sure that the value is not bigger than the given value.",
              clampMin | optional | Number | doc "ClampMin makes sure that the value is not smaller than the given value.",
              multiply | optional | Number | doc "Multiply the value.",
              type | String | doc "Type of the math transform to be run." | default = "Multiply",
            } | doc m%"
Math is used to transform the input via mathematical operations such as
multiplication.
"%,
            string | optional | {
              convert | optional | String | doc m%"
Optional conversion method to be specified.
`ToUpper` and `ToLower` change the letter case of the input string.
`ToBase64` and `FromBase64` perform a base64 conversion based on the input string.
`ToJson` converts any input value into its raw JSON representation.
`ToSha1`, `ToSha256` and `ToSha512` generate a hash value based on the input
converted to JSON.
`ToAdler32` generate a addler32 hash based on the input string.
"%,
              fmt | optional | String | doc m%"
Format the input using a Go format string. See
https://golang.org/pkg/fmt/ for details.
"%,
              join | optional | {
                separator | String | doc m%"
Separator defines the character that should separate the values from each
other in the joined string.
"%,
              } | doc "Join defines parameters to join a slice of values to a string.",
              regexp | optional | {
                group | optional | Number | doc "Group number to match. 0 (the default) matches the entire expression.",
                match | String | doc m%"
Match string. May optionally include submatches, aka capture groups.
See https://pkg.go.dev/regexp/ for details.
"%,
              } | doc "Extract a match from the input using a regular expression.",
              trim | optional | String | doc "Trim the prefix or suffix from the input",
              type | String | doc "Type of the string transform to be run." | default = "Format",
            } | doc m%"
String is used to transform the input into a string or a different kind
of string. Note that the input does not necessarily need to be a string.
"%,
            type | String | doc "Type of the transform to be run.",
          } | doc m%"
Transforms are the list of functions that are used as a FIFO pipe for the
input to be transformed.
"%,
          type | String | doc m%"
Type sets the patching behaviour to be used. Each patch type may require
its own fields to be set on the Patch object.
"% | default = "FromCompositeFieldPath",
        } | doc m%"
Patches is a list of environment patches that are executed before a
composition's resources are composed.
"%,
        policy | optional | {
          resolution | String | doc m%"
Resolution specifies whether resolution of this reference is required.
The default is 'Required', which means the reconcile will fail if the
reference cannot be resolved. 'Optional' means this reference will be
a no-op if it cannot be resolved.
"% | default = "Required",
          resolve | optional | String | doc m%"
Resolve specifies when this reference should be resolved. The default
is 'IfNotPresent', which will attempt to resolve the reference only when
the corresponding field is not present. Use 'Always' to resolve the
reference on every reconcile.
"%,
        } | doc m%"
Policy represents the Resolve and Resolution policies which apply to
all EnvironmentSourceReferences in EnvironmentConfigs list.
"%,
      } | doc m%"
Environment configures the environment in which resources are rendered.


THIS IS AN ALPHA FIELD. Do not use it in production. It is not honored
unless the relevant Crossplane feature flag is enabled, and may be
changed or removed without notice.
"%,
      mode | String | doc m%"
Mode controls what type or "mode" of Composition will be used.


"Pipeline" indicates that a Composition specifies a pipeline of
Composition Functions, each of which is responsible for producing
composed resources that Crossplane should create or update.


"Resources" indicates that a Composition uses what is commonly referred
to as "Patch & Transform" or P&T composition. This mode of Composition
uses an array of resources, each a template for a composed resource.


All Compositions should use Pipeline mode. Resources mode is deprecated.
Resources mode won't be removed in Crossplane 1.x, and will remain the
default to avoid breaking legacy Compositions. However, it's no longer
accepting new features, and only accepting security related bug fixes.
"% | default = "Resources",
      patchSets | optional | Array {
        name | String | doc "Name of this PatchSet.",
        patches | Array {
          combine | optional | {
            strategy | String | doc m%"
Strategy defines the strategy to use to combine the input variable values.
Currently only string is supported.
"%,
            string | optional | {
              fmt | String | doc m%"
Format the input using a Go format string. See
https://golang.org/pkg/fmt/ for details.
"%,
            } | doc m%"
String declares that input variables should be combined into a single
string, using the relevant settings for formatting purposes.
"%,
            variables | Array {
              fromFieldPath | String | doc m%"
FromFieldPath is the path of the field on the source whose value is
to be used as input.
"%,
            } | doc m%"
Variables are the list of variables whose values will be retrieved and
combined.
"%,
          } | doc m%"
Combine is the patch configuration for a CombineFromComposite,
CombineFromEnvironment, CombineToComposite or CombineToEnvironment patch.
"%,
          fromFieldPath | optional | String | doc m%"
FromFieldPath is the path of the field on the resource whose value is
to be used as input. Required when type is FromCompositeFieldPath,
FromEnvironmentFieldPath, ToCompositeFieldPath, ToEnvironmentFieldPath.
"%,
          patchSetName | optional | String | doc "PatchSetName to include patches from. Required when type is PatchSet.",
          policy | optional | {
            fromFieldPath | optional | String | doc m%"
FromFieldPath specifies how to patch from a field path. The default is
'Optional', which means the patch will be a no-op if the specified
fromFieldPath does not exist. Use 'Required' if the patch should fail if
the specified path does not exist.
"%,
            mergeOptions | optional | {
              appendSlice | optional | Bool | doc "Specifies that already existing elements in a merged slice should be preserved",
              keepMapValues | optional | Bool | doc "Specifies that already existing values in a merged map should be preserved",
            } | doc "MergeOptions Specifies merge options on a field path.",
          } | doc "Policy configures the specifics of patching behaviour.",
          toFieldPath | optional | String | doc m%"
ToFieldPath is the path of the field on the resource whose value will
be changed with the result of transforms. Leave empty if you'd like to
propagate to the same path as fromFieldPath.
"%,
          transforms | optional | Array {
            convert | optional | {
              format | optional | String | doc m%"
The expected input format.


* `quantity` - parses the input as a K8s [`resource.Quantity`](https://pkg.go.dev/k8s.io/apimachinery/pkg/api/resource#Quantity).
Only used during `string -> float64` conversions.
* `json` - parses the input as a JSON string.
Only used during `string -> object` or `string -> list` conversions.


If this property is null, the default conversion is applied.
"%,
              toType | String | doc "ToType is the type of the output of this transform.",
            } | doc "Convert is used to cast the input into the given output type.",
            map | optional | {
            } | doc "Map uses the input as a key in the given map and returns the value.",
            match | optional | {
              fallbackTo | String | doc "Determines to what value the transform should fallback if no pattern matches." | default = "Value",
              fallbackValue | optional | Dyn | doc m%"
The fallback value that should be returned by the transform if now pattern
matches.
"%,
              patterns | optional | Array {
                literal | optional | String | doc m%"
Literal exactly matches the input string (case sensitive).
Is required if `type` is `literal`.
"%,
                regexp | optional | String | doc m%"
Regexp to match against the input string.
Is required if `type` is `regexp`.
"%,
                result | Dyn | doc "The value that is used as result of the transform if the pattern matches.",
                type | String | doc m%"
Type specifies how the pattern matches the input.


* `literal` - the pattern value has to exactly match (case sensitive) the
input string. This is the default.


* `regexp` - the pattern treated as a regular expression against
which the input string is tested. Crossplane will throw an error if the
key is not a valid regexp.
"% | default = "literal",
              } | doc m%"
The patterns that should be tested against the input string.
Patterns are tested in order. The value of the first match is used as
result of this transform.
"%,
            } | doc "Match is a more complex version of Map that matches a list of patterns.",
            math | optional | {
              clampMax | optional | Number | doc "ClampMax makes sure that the value is not bigger than the given value.",
              clampMin | optional | Number | doc "ClampMin makes sure that the value is not smaller than the given value.",
              multiply | optional | Number | doc "Multiply the value.",
              type | String | doc "Type of the math transform to be run." | default = "Multiply",
            } | doc m%"
Math is used to transform the input via mathematical operations such as
multiplication.
"%,
            string | optional | {
              convert | optional | String | doc m%"
Optional conversion method to be specified.
`ToUpper` and `ToLower` change the letter case of the input string.
`ToBase64` and `FromBase64` perform a base64 conversion based on the input string.
`ToJson` converts any input value into its raw JSON representation.
`ToSha1`, `ToSha256` and `ToSha512` generate a hash value based on the input
converted to JSON.
`ToAdler32` generate a addler32 hash based on the input string.
"%,
              fmt | optional | String | doc m%"
Format the input using a Go format string. See
https://golang.org/pkg/fmt/ for details.
"%,
              join | optional | {
                separator | String | doc m%"
Separator defines the character that should separate the values from each
other in the joined string.
"%,
              } | doc "Join defines parameters to join a slice of values to a string.",
              regexp | optional | {
                group | optional | Number | doc "Group number to match. 0 (the default) matches the entire expression.",
                match | String | doc m%"
Match string. May optionally include submatches, aka capture groups.
See https://pkg.go.dev/regexp/ for details.
"%,
              } | doc "Extract a match from the input using a regular expression.",
              trim | optional | String | doc "Trim the prefix or suffix from the input",
              type | String | doc "Type of the string transform to be run." | default = "Format",
            } | doc m%"
String is used to transform the input into a string or a different kind
of string. Note that the input does not necessarily need to be a string.
"%,
            type | String | doc "Type of the transform to be run.",
          } | doc m%"
Transforms are the list of functions that are used as a FIFO pipe for the
input to be transformed.
"%,
          type | String | doc m%"
Type sets the patching behaviour to be used. Each patch type may require
its own fields to be set on the Patch object.
"% | default = "FromCompositeFieldPath",
        } | doc "Patches will be applied as an overlay to the base resource.",
      } | doc m%"
PatchSets define a named set of patches that may be included by any
resource in this Composition. PatchSets cannot themselves refer to other
PatchSets.


PatchSets are only used by the "Resources" mode of Composition. They
are ignored by other modes.


Deprecated: Use Composition Functions instead.
"%,
      pipeline | optional | Array {
        credentials | optional | Array {
          name | String | doc "Name of this set of credentials.",
          secretRef | optional | {
            name | String | doc "Name of the secret.",
            namespace | String | doc "Namespace of the secret.",
          } | doc m%"
A SecretRef is a reference to a secret containing credentials that should
be supplied to the function.
"%,
          source | String | doc "Source of the function credentials.",
        } | doc "Credentials are optional credentials that the Composition Function needs.",
        functionRef | {
          name | String | doc "Name of the referenced Function.",
        } | doc m%"
FunctionRef is a reference to the Composition Function this step should
execute.
"%,
        input | optional | {
        } | doc m%"
Input is an optional, arbitrary Kubernetes resource (i.e. a resource
with an apiVersion and kind) that will be passed to the Composition
Function as the 'input' of its RunFunctionRequest.
"%,
        step | String | doc "Step name. Must be unique within its Pipeline.",
      } | doc m%"
Pipeline is a list of composition function steps that will be used when a
composite resource referring to this composition is created. One of
resources and pipeline must be specified - you cannot specify both.


The Pipeline is only used by the "Pipeline" mode of Composition. It is
ignored by other modes.
"%,
      publishConnectionDetailsWithStoreConfigRef | {
        name | String | doc "Name of the referenced StoreConfig.",
      } | doc m%"
PublishConnectionDetailsWithStoreConfig specifies the secret store config
with which the connection details of composite resources dynamically
provisioned using this composition will be published.


THIS IS AN ALPHA FIELD. Do not use it in production. It is not honored
unless the relevant Crossplane feature flag is enabled, and may be
changed or removed without notice.
"% | default = {
        name = "default"
      },
      resources | optional | Array {
        base | {
        } | doc "Base is the target resource that the patches will be applied on.",
        connectionDetails | optional | Array {
          fromConnectionSecretKey | optional | String | doc m%"
FromConnectionSecretKey is the key that will be used to fetch the value
from the composed resource's connection secret.
"%,
          fromFieldPath | optional | String | doc m%"
FromFieldPath is the path of the field on the composed resource whose
value to be used as input. Name must be specified if the type is
FromFieldPath.
"%,
          name | optional | String | doc m%"
Name of the connection secret key that will be propagated to the
connection secret of the composition instance. Leave empty if you'd like
to use the same key name.
"%,
          type | optional | String | doc m%"
Type sets the connection detail fetching behaviour to be used. Each
connection detail type may require its own fields to be set on the
ConnectionDetail object. If the type is omitted Crossplane will attempt
to infer it based on which other fields were specified. If multiple
fields are specified the order of precedence is:
1. FromValue
2. FromConnectionSecretKey
3. FromFieldPath
"%,
          value | optional | String | doc m%"
Value that will be propagated to the connection secret of the composite
resource. May be set to inject a fixed, non-sensitive connection secret
value, for example a well-known port.
"%,
        } | doc m%"
ConnectionDetails lists the propagation secret keys from this target
resource to the composition instance connection secret.
"%,
        name | optional | String | doc m%"
A Name uniquely identifies this entry within its Composition's resources
array. Names are optional but *strongly* recommended. When all entries in
the resources array are named entries may added, deleted, and reordered
as long as their names do not change. When entries are not named the
length and order of the resources array should be treated as immutable.
Either all or no entries must be named.
"%,
        patches | optional | Array {
          combine | optional | {
            strategy | String | doc m%"
Strategy defines the strategy to use to combine the input variable values.
Currently only string is supported.
"%,
            string | optional | {
              fmt | String | doc m%"
Format the input using a Go format string. See
https://golang.org/pkg/fmt/ for details.
"%,
            } | doc m%"
String declares that input variables should be combined into a single
string, using the relevant settings for formatting purposes.
"%,
            variables | Array {
              fromFieldPath | String | doc m%"
FromFieldPath is the path of the field on the source whose value is
to be used as input.
"%,
            } | doc m%"
Variables are the list of variables whose values will be retrieved and
combined.
"%,
          } | doc m%"
Combine is the patch configuration for a CombineFromComposite,
CombineFromEnvironment, CombineToComposite or CombineToEnvironment patch.
"%,
          fromFieldPath | optional | String | doc m%"
FromFieldPath is the path of the field on the resource whose value is
to be used as input. Required when type is FromCompositeFieldPath,
FromEnvironmentFieldPath, ToCompositeFieldPath, ToEnvironmentFieldPath.
"%,
          patchSetName | optional | String | doc "PatchSetName to include patches from. Required when type is PatchSet.",
          policy | optional | {
            fromFieldPath | optional | String | doc m%"
FromFieldPath specifies how to patch from a field path. The default is
'Optional', which means the patch will be a no-op if the specified
fromFieldPath does not exist. Use 'Required' if the patch should fail if
the specified path does not exist.
"%,
            mergeOptions | optional | {
              appendSlice | optional | Bool | doc "Specifies that already existing elements in a merged slice should be preserved",
              keepMapValues | optional | Bool | doc "Specifies that already existing values in a merged map should be preserved",
            } | doc "MergeOptions Specifies merge options on a field path.",
          } | doc "Policy configures the specifics of patching behaviour.",
          toFieldPath | optional | String | doc m%"
ToFieldPath is the path of the field on the resource whose value will
be changed with the result of transforms. Leave empty if you'd like to
propagate to the same path as fromFieldPath.
"%,
          transforms | optional | Array {
            convert | optional | {
              format | optional | String | doc m%"
The expected input format.


* `quantity` - parses the input as a K8s [`resource.Quantity`](https://pkg.go.dev/k8s.io/apimachinery/pkg/api/resource#Quantity).
Only used during `string -> float64` conversions.
* `json` - parses the input as a JSON string.
Only used during `string -> object` or `string -> list` conversions.


If this property is null, the default conversion is applied.
"%,
              toType | String | doc "ToType is the type of the output of this transform.",
            } | doc "Convert is used to cast the input into the given output type.",
            map | optional | {
            } | doc "Map uses the input as a key in the given map and returns the value.",
            match | optional | {
              fallbackTo | String | doc "Determines to what value the transform should fallback if no pattern matches." | default = "Value",
              fallbackValue | optional | Dyn | doc m%"
The fallback value that should be returned by the transform if now pattern
matches.
"%,
              patterns | optional | Array {
                literal | optional | String | doc m%"
Literal exactly matches the input string (case sensitive).
Is required if `type` is `literal`.
"%,
                regexp | optional | String | doc m%"
Regexp to match against the input string.
Is required if `type` is `regexp`.
"%,
                result | Dyn | doc "The value that is used as result of the transform if the pattern matches.",
                type | String | doc m%"
Type specifies how the pattern matches the input.


* `literal` - the pattern value has to exactly match (case sensitive) the
input string. This is the default.


* `regexp` - the pattern treated as a regular expression against
which the input string is tested. Crossplane will throw an error if the
key is not a valid regexp.
"% | default = "literal",
              } | doc m%"
The patterns that should be tested against the input string.
Patterns are tested in order. The value of the first match is used as
result of this transform.
"%,
            } | doc "Match is a more complex version of Map that matches a list of patterns.",
            math | optional | {
              clampMax | optional | Number | doc "ClampMax makes sure that the value is not bigger than the given value.",
              clampMin | optional | Number | doc "ClampMin makes sure that the value is not smaller than the given value.",
              multiply | optional | Number | doc "Multiply the value.",
              type | String | doc "Type of the math transform to be run." | default = "Multiply",
            } | doc m%"
Math is used to transform the input via mathematical operations such as
multiplication.
"%,
            string | optional | {
              convert | optional | String | doc m%"
Optional conversion method to be specified.
`ToUpper` and `ToLower` change the letter case of the input string.
`ToBase64` and `FromBase64` perform a base64 conversion based on the input string.
`ToJson` converts any input value into its raw JSON representation.
`ToSha1`, `ToSha256` and `ToSha512` generate a hash value based on the input
converted to JSON.
`ToAdler32` generate a addler32 hash based on the input string.
"%,
              fmt | optional | String | doc m%"
Format the input using a Go format string. See
https://golang.org/pkg/fmt/ for details.
"%,
              join | optional | {
                separator | String | doc m%"
Separator defines the character that should separate the values from each
other in the joined string.
"%,
              } | doc "Join defines parameters to join a slice of values to a string.",
              regexp | optional | {
                group | optional | Number | doc "Group number to match. 0 (the default) matches the entire expression.",
                match | String | doc m%"
Match string. May optionally include submatches, aka capture groups.
See https://pkg.go.dev/regexp/ for details.
"%,
              } | doc "Extract a match from the input using a regular expression.",
              trim | optional | String | doc "Trim the prefix or suffix from the input",
              type | String | doc "Type of the string transform to be run." | default = "Format",
            } | doc m%"
String is used to transform the input into a string or a different kind
of string. Note that the input does not necessarily need to be a string.
"%,
            type | String | doc "Type of the transform to be run.",
          } | doc m%"
Transforms are the list of functions that are used as a FIFO pipe for the
input to be transformed.
"%,
          type | String | doc m%"
Type sets the patching behaviour to be used. Each patch type may require
its own fields to be set on the Patch object.
"% | default = "FromCompositeFieldPath",
        } | doc "Patches will be applied as overlay to the base resource.",
        readinessChecks | Array {
          fieldPath | optional | String | doc "FieldPath shows the path of the field whose value will be used.",
          matchCondition | optional | {
            status | String | doc "Status is the status of the condition you'd like to match." | default = "True",
            type | String | doc "Type indicates the type of condition you'd like to use." | default = "Ready",
          } | doc m%"
MatchCondition specifies the condition you'd like to match if you're using "MatchCondition" type.
"%,
          matchInteger | optional | Number | doc "MatchInt is the value you'd like to match if you're using \"MatchInt\" type.",
          matchString | optional | String | doc "MatchString is the value you'd like to match if you're using \"MatchString\" type.",
          type | String | doc "Type indicates the type of probe you'd like to use.",
        } | doc m%"
ReadinessChecks allows users to define custom readiness checks. All checks
have to return true in order for resource to be considered ready. The
default readiness check is to have the "Ready" condition to be "True".
"% | default = [{
          matchCondition = {
            status = "True",
            type = "Ready"
          },
          type = "MatchCondition"
        }],
      } | doc m%"
Resources is a list of resource templates that will be used when a
composite resource referring to this composition is created.


Resources are only used by the "Resources" mode of Composition. They are
ignored by other modes.


Deprecated: Use Composition Functions instead.
"%,
      writeConnectionSecretsToNamespace | optional | String | doc m%"
WriteConnectionSecretsToNamespace specifies the namespace in which the
connection secrets of composite resource dynamically provisioned using
this composition will be created.
This field is planned to be replaced in a future release in favor of
PublishConnectionDetailsWithStoreConfigRef. Currently, both could be
set independently and connection details would be published to both
without affecting each other as long as related fields at MR level
specified.
"%,
    } | doc "CompositionSpec specifies desired state of a composition.",
  },
}
