# Module: storeconfig.secrets.crossplane.io

let k8s_io_objectmeta = import "k8s_io" in

{
  StoreConfig = {
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
    spec | {
      defaultScope | String | doc m%"
DefaultScope used for scoping secrets for "cluster-scoped" resources.
If store type is "Kubernetes", this would mean the default namespace to
store connection secrets for cluster scoped resources.
In case of "Vault", this would be used as the default parent path.
Typically, should be set as Crossplane installation namespace.
"%,
      kubernetes | optional | {
        auth | {
          env | optional | {
            name | String | doc "Name is the name of an environment variable.",
          } | doc m%"
Env is a reference to an environment variable that contains credentials
that must be used to connect to the provider.
"%,
          fs | optional | {
            path | String | doc "Path is a filesystem path.",
          } | doc m%"
Fs is a reference to a filesystem location that contains credentials that
must be used to connect to the provider.
"%,
          secretRef | optional | {
            key | String | doc "The key to select.",
            name | String | doc "Name of the secret.",
            namespace | String | doc "Namespace of the secret.",
          } | doc m%"
A SecretRef is a reference to a secret key that contains the credentials
that must be used to connect to the provider.
"%,
          source | String | doc "Source of the credentials.",
        } | doc "Credentials used to connect to the Kubernetes API.",
      } | doc m%"
Kubernetes configures a Kubernetes secret store.
If the "type" is "Kubernetes" but no config provided, in cluster config
will be used.
"%,
      plugin | optional | {
        configRef | optional | {
          apiVersion | String | doc "APIVersion of the referenced config.",
          kind | String | doc "Kind of the referenced config.",
          name | String | doc "Name of the referenced config.",
        } | doc "ConfigRef contains store config reference info.",
        endpoint | optional | String | doc "Endpoint is the endpoint of the gRPC server.",
      } | doc "Plugin configures External secret store as a plugin.",
      type | String | doc m%"
Type configures which secret store to be used. Only the configuration
block for this store will be used and others will be ignored if provided.
Default is Kubernetes.
"% | default = "Kubernetes",
    } | doc "A StoreConfigSpec defines the desired state of a StoreConfig.",
  },
}
