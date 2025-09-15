# Module: k8s.io.resource.v1beta1

let metav1 = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in
let condition = metav1.Condition in
let objectMeta = metav1.ObjectMeta in
let listMeta = metav1.ListMeta in
let time = metav1.Time in
let v0Module = import "../../v0/mod.ncl" in
let quantity = v0Module.Quantity in
let corev1 = import "../core/v1/mod.ncl" in
let nodeSelector = corev1.NodeSelector in

{
  AllocatedDeviceStatus = {
    conditions
       | Array condition
       | doc m%"
Conditions contains the latest observation of the device's state. If the device has been configured according to the class and claim config references, the `Ready` condition should be True.

Must not contain more than 8 entries.
"%
       | optional,
    data
       | v1Module.RawExtension
       | doc m%"
Data contains arbitrary driver-specific data.

The length of the raw data must be smaller or equal to 10 Ki.
"%
       | optional,
    device
       | String
       | doc m%"
Device references one device instance via its name in the driver's resource pool. It must be a DNS label.
"%,
    driver
       | String
       | doc m%"
Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.

Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.
"%,
    networkData
       | NetworkDeviceData
       | doc "NetworkData contains network-related information specific to the device."
       | optional,
    pool
       | String
       | doc m%"
This name together with the driver name and the device name field identify which device was allocated (`<driver name>/<pool name>/<device name>`).

Must not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.
"%
  },

  AllocationResult = {
    devices
       | DeviceAllocationResult
       | doc "Devices is the result of allocating devices."
       | optional,
    nodeSelector
       | nodeSelector
       | doc m%"
NodeSelector defines where the allocated resources are available. If unset, they are available everywhere.
"%
       | optional
  },

  BasicDevice = {
    allNodes
       | Bool
       | doc m%"
AllNodes indicates that all nodes have access to the device.

Must only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.
"%
       | optional,
    attributes
       | { .. }
       | doc m%"
Attributes defines the set of attributes for this device. The name of each attribute must be unique in that set.

The maximum number of attributes and capacities combined is 32.
"%
       | optional,
    capacity
       | { .. }
       | doc m%"
Capacity defines the set of capacities for this device. The name of each capacity must be unique in that set.

The maximum number of attributes and capacities combined is 32.
"%
       | optional,
    consumesCounters
       | Array DeviceCounterConsumption
       | doc m%"
ConsumesCounters defines a list of references to sharedCounters and the set of counters that the device will consume from those counter sets.

There can only be a single entry per counterSet.

The total number of device counter consumption entries must be <= 32. In addition, the total number in the entire ResourceSlice must be <= 1024 (for example, 64 devices with 16 counters each).
"%
       | optional,
    nodeName
       | String
       | doc m%"
NodeName identifies the node where the device is available.

Must only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.
"%
       | optional,
    nodeSelector
       | nodeSelector
       | doc m%"
NodeSelector defines the nodes where the device is available.

Must use exactly one term.

Must only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.
"%
       | optional,
    taints
       | Array DeviceTaint
       | doc m%"
If specified, these are the driver-defined taints.

The maximum number of taints is 4.

This is an alpha field and requires enabling the DRADeviceTaints feature gate.
"%
       | optional
  },

  CELDeviceSelector = {
    expression
       | String
       | doc m%"
Expression is a CEL expression which evaluates a single device. It must evaluate to true when the device under consideration satisfies the desired criteria, and false when it does not. Any other result is an error and causes allocation of devices to abort.

The expression's input is an object named "device", which carries the following properties:
 - driver (string): the name of the driver which defines this device.
 - attributes (map[string]object): the device's attributes, grouped by prefix
   (e.g. device.attributes["dra.example.com"] evaluates to an object with all
   of the attributes which were prefixed by "dra.example.com".
 - capacity (map[string]object): the device's capacities, grouped by prefix.

Example: Consider a device with driver="dra.example.com", which exposes two attributes named "model" and "ext.example.com/family" and which exposes one capacity named "modules". This input to this expression would have the following fields:

    device.driver
    device.attributes["dra.example.com"].model
    device.attributes["ext.example.com"].family
    device.capacity["dra.example.com"].modules

The device.driver field can be used to check for a specific driver, either as a high-level precondition (i.e. you only want to consider devices from this driver) or as part of a multi-clause expression that is meant to consider devices from different drivers.

The value type of each attribute is defined by the device definition, and users who write these expressions must consult the documentation for their specific drivers. The value type of each capacity is Quantity.

If an unknown prefix is used as a lookup in either device.attributes or device.capacity, an empty map will be returned. Any reference to an unknown field will cause an evaluation error and allocation to abort.

A robust expression should check for the existence of attributes before referencing them.

For ease of use, the cel.bind() function is enabled, and can be used to simplify expressions that access multiple attributes with the same domain. For example:

    cel.bind(dra, device.attributes["dra.example.com"], dra.someBool && dra.anotherBool)

The length of the expression must be smaller or equal to 10 Ki. The cost of evaluating it is also limited based on the estimated number of logical steps.
"%
  },

  Counter = {
    value
       | io.k8s.apimachinery.pkg.api.resource.Quantity
       | doc "Value defines how much of a certain device counter is available."
  },

  CounterSet = {
    counters
       | { .. }
       | doc m%"
Counters defines the set of counters for this CounterSet The name of each counter must be unique in that set and must be a DNS label.

The maximum number of counters is 32.
"%,
    name
       | String
       | doc "Name defines the name of the counter set. It must be a DNS label."
  },

  Device = {
    basic
       | BasicDevice
       | doc "Basic defines one device instance."
       | optional,
    name
       | String
       | doc m%"
Name is unique identifier among all devices managed by the driver in the pool. It must be a DNS label.
"%
  },

  DeviceAllocationConfiguration = {
    opaque
       | OpaqueDeviceConfiguration
       | doc "Opaque provides driver-specific configuration parameters."
       | optional,
    requests
       | Array String
       | doc m%"
Requests lists the names of requests where the configuration applies. If empty, its applies to all requests.

References to subrequests must include the name of the main request and may include the subrequest using the format <main request>[/<subrequest>]. If just the main request is given, the configuration applies to all subrequests.
"%
       | optional,
    source
       | String
       | doc m%"
Source records whether the configuration comes from a class and thus is not something that a normal user would have been able to set or from a claim.
"%
  },

  DeviceAllocationResult = {
    config
       | Array DeviceAllocationConfiguration
       | doc m%"
This field is a combination of all the claim and class configuration parameters. Drivers can distinguish between those based on a flag.

This includes configuration parameters for drivers which have no allocated devices in the result because it is up to the drivers which configuration parameters they support. They can silently ignore unknown configuration parameters.
"%
       | optional,
    results
       | Array DeviceRequestAllocationResult
       | doc "Results lists all allocated devices."
       | optional
  },

  DeviceAttribute = {
    bool
       | Bool
       | doc "BoolValue is a true/false value."
       | optional,
    int
       | Number
       | doc "IntValue is a number."
       | optional,
    string
       | String
       | doc "StringValue is a string. Must not be longer than 64 characters."
       | optional,
    version
       | String
       | doc m%"
VersionValue is a semantic version according to semver.org spec 2.0.0. Must not be longer than 64 characters.
"%
       | optional
  },

  DeviceCapacity = {
    value
       | io.k8s.apimachinery.pkg.api.resource.Quantity
       | doc "Value defines how much of a certain device capacity is available."
  },

  DeviceClaim = {
    config
       | Array DeviceClaimConfiguration
       | doc m%"
This field holds configuration for multiple potential drivers which could satisfy requests in this claim. It is ignored while allocating the claim.
"%
       | optional,
    constraints
       | Array DeviceConstraint
       | doc m%"
These constraints must be satisfied by the set of devices that get allocated for the claim.
"%
       | optional,
    requests
       | Array DeviceRequest
       | doc m%"
Requests represent individual requests for distinct devices which must all be satisfied. If empty, nothing needs to be allocated.
"%
       | optional
  },

  DeviceClaimConfiguration = {
    opaque
       | OpaqueDeviceConfiguration
       | doc "Opaque provides driver-specific configuration parameters."
       | optional,
    requests
       | Array String
       | doc m%"
Requests lists the names of requests where the configuration applies. If empty, it applies to all requests.

References to subrequests must include the name of the main request and may include the subrequest using the format <main request>[/<subrequest>]. If just the main request is given, the configuration applies to all subrequests.
"%
       | optional
  },

  DeviceClass = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | objectMeta
       | doc "Standard object metadata"
       | optional,
    spec
       | DeviceClassSpec
       | doc m%"
Spec defines what can be allocated and how to configure it.

This is mutable. Consumers have to be prepared for classes changing at any time, either because they get updated or replaced. Claim allocations are done once based on whatever was set in classes at the time of allocation.

Changing the spec automatically increments the metadata.generation number.
"%
  },

  DeviceClassConfiguration = {
    opaque
       | OpaqueDeviceConfiguration
       | doc "Opaque provides driver-specific configuration parameters."
       | optional
  },

  DeviceClass = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array DeviceClass
       | doc "Items is the list of resource classes.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | listMeta
       | doc "Standard list metadata"
       | optional
  },

  DeviceClassSpec = {
    config
       | Array DeviceClassConfiguration
       | doc m%"
Config defines configuration parameters that apply to each device that is claimed via this class. Some classses may potentially be satisfied by multiple drivers, so each instance of a vendor configuration applies to exactly one driver.

They are passed to the driver, but are not considered while allocating the claim.
"%
       | optional,
    selectors
       | Array DeviceSelector
       | doc "Each selector must be satisfied by a device which is claimed via this class."
       | optional
  },

  DeviceConstraint = {
    matchAttribute
       | String
       | doc m%"
MatchAttribute requires that all devices in question have this attribute and that its type and value are the same across those devices.

For example, if you specified "dra.example.com/numa" (a hypothetical example!), then only devices in the same NUMA node will be chosen. A device which does not have that attribute will not be chosen. All devices should use a value of the same type for this attribute because that is part of its specification, but if one device doesn't, then it also will not be chosen.

Must include the domain qualifier.
"%
       | optional,
    requests
       | Array String
       | doc m%"
Requests is a list of the one or more requests in this claim which must co-satisfy this constraint. If a request is fulfilled by multiple devices, then all of the devices must satisfy the constraint. If this is not specified, this constraint applies to all requests in this claim.

References to subrequests must include the name of the main request and may include the subrequest using the format <main request>[/<subrequest>]. If just the main request is given, the constraint applies to all subrequests.
"%
       | optional
  },

  DeviceCounterConsumption = {
    counterSet
       | String
       | doc m%"
CounterSet is the name of the set from which the counters defined will be consumed.
"%,
    counters
       | { .. }
       | doc m%"
Counters defines the counters that will be consumed by the device.

The maximum number counters in a device is 32. In addition, the maximum number of all counters in all devices is 1024 (for example, 64 devices with 16 counters each).
"%
  },

  DeviceRequest = {
    adminAccess
       | Bool
       | doc m%"
AdminAccess indicates that this is a claim for administrative access to the device(s). Claims with AdminAccess are expected to be used for monitoring or other management services for a device.  They ignore all ordinary claims to the device with respect to access modes and any resource allocations.

This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.

This is an alpha field and requires enabling the DRAAdminAccess feature gate. Admin access is disabled if this field is unset or set to false, otherwise it is enabled.
"%
       | optional,
    allocationMode
       | String
       | doc m%"
AllocationMode and its related fields define how devices are allocated to satisfy this request. Supported values are:

- ExactCount: This request is for a specific number of devices.
  This is the default. The exact number is provided in the
  count field.

- All: This request is for all of the matching devices in a pool.
  At least one device must exist on the node for the allocation to succeed.
  Allocation will fail if some devices are already allocated,
  unless adminAccess is requested.

If AllocationMode is not specified, the default mode is ExactCount. If the mode is ExactCount and count is not specified, the default count is one. Any other requests must specify this field.

This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.

More modes may get added in the future. Clients must refuse to handle requests with unknown modes.
"%
       | optional,
    count
       | Number
       | doc m%"
Count is used only when the count mode is "ExactCount". Must be greater than zero. If AllocationMode is ExactCount and this field is not specified, the default is one.

This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.
"%
       | optional,
    deviceClassName
       | String
       | doc m%"
DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this request.

A class is required if no subrequests are specified in the firstAvailable list and no class can be set if subrequests are specified in the firstAvailable list. Which classes are available depends on the cluster.

Administrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference.
"%
       | optional,
    firstAvailable
       | Array DeviceSubRequest
       | doc m%"
FirstAvailable contains subrequests, of which exactly one will be satisfied by the scheduler to satisfy this request. It tries to satisfy them in the order in which they are listed here. So if there are two entries in the list, the scheduler will only check the second one if it determines that the first one cannot be used.

This field may only be set in the entries of DeviceClaim.Requests.

DRA does not yet implement scoring, so the scheduler will select the first set of devices that satisfies all the requests in the claim. And if the requirements can be satisfied on more than one node, other scheduling features will determine which node is chosen. This means that the set of devices allocated to a claim might not be the optimal set available to the cluster. Scoring will be implemented later.
"%
       | optional,
    name
       | String
       | doc m%"
Name can be used to reference this request in a pod.spec.containers[].resources.claims entry and in a constraint of the claim.

Must be a DNS label and unique among all DeviceRequests in a ResourceClaim.
"%,
    selectors
       | Array DeviceSelector
       | doc m%"
Selectors define criteria which must be satisfied by a specific device in order for that device to be considered for this request. All selectors must be satisfied for a device to be considered.

This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.
"%
       | optional,
    tolerations
       | Array DeviceToleration
       | doc m%"
If specified, the request's tolerations.

Tolerations for NoSchedule are required to allocate a device which has a taint with that effect. The same applies to NoExecute.

In addition, should any of the allocated devices get tainted with NoExecute after allocation and that effect is not tolerated, then all pods consuming the ResourceClaim get deleted to evict them. The scheduler will not let new pods reserve the claim while it has these tainted devices. Once all pods are evicted, the claim will get deallocated.

The maximum number of tolerations is 16.

This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.

This is an alpha field and requires enabling the DRADeviceTaints feature gate.
"%
       | optional
  },

  DeviceRequestAllocationResult = {
    adminAccess
       | Bool
       | doc m%"
AdminAccess indicates that this device was allocated for administrative access. See the corresponding request field for a definition of mode.

This is an alpha field and requires enabling the DRAAdminAccess feature gate. Admin access is disabled if this field is unset or set to false, otherwise it is enabled.
"%
       | optional,
    device
       | String
       | doc m%"
Device references one device instance via its name in the driver's resource pool. It must be a DNS label.
"%,
    driver
       | String
       | doc m%"
Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.

Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.
"%,
    pool
       | String
       | doc m%"
This name together with the driver name and the device name field identify which device was allocated (`<driver name>/<pool name>/<device name>`).

Must not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.
"%,
    request
       | String
       | doc m%"
Request is the name of the request in the claim which caused this device to be allocated. If it references a subrequest in the firstAvailable list on a DeviceRequest, this field must include both the name of the main request and the subrequest using the format <main request>/<subrequest>.

Multiple devices may have been allocated per request.
"%,
    tolerations
       | Array DeviceToleration
       | doc m%"
A copy of all tolerations specified in the request at the time when the device got allocated.

The maximum number of tolerations is 16.

This is an alpha field and requires enabling the DRADeviceTaints feature gate.
"%
       | optional
  },

  DeviceSelector = {
    cel
       | CELDeviceSelector
       | doc "CEL contains a CEL expression for selecting a device."
       | optional
  },

  DeviceSubRequest = {
    allocationMode
       | String
       | doc m%"
AllocationMode and its related fields define how devices are allocated to satisfy this subrequest. Supported values are:

- ExactCount: This request is for a specific number of devices.
  This is the default. The exact number is provided in the
  count field.

- All: This subrequest is for all of the matching devices in a pool.
  Allocation will fail if some devices are already allocated,
  unless adminAccess is requested.

If AllocationMode is not specified, the default mode is ExactCount. If the mode is ExactCount and count is not specified, the default count is one. Any other subrequests must specify this field.

More modes may get added in the future. Clients must refuse to handle requests with unknown modes.
"%
       | optional,
    count
       | Number
       | doc m%"
Count is used only when the count mode is "ExactCount". Must be greater than zero. If AllocationMode is ExactCount and this field is not specified, the default is one.
"%
       | optional,
    deviceClassName
       | String
       | doc m%"
DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this subrequest.

A class is required. Which classes are available depends on the cluster.

Administrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference.
"%,
    name
       | String
       | doc m%"
Name can be used to reference this subrequest in the list of constraints or the list of configurations for the claim. References must use the format <main request>/<subrequest>.

Must be a DNS label.
"%,
    selectors
       | Array DeviceSelector
       | doc m%"
Selectors define criteria which must be satisfied by a specific device in order for that device to be considered for this subrequest. All selectors must be satisfied for a device to be considered.
"%
       | optional,
    tolerations
       | Array DeviceToleration
       | doc m%"
If specified, the request's tolerations.

Tolerations for NoSchedule are required to allocate a device which has a taint with that effect. The same applies to NoExecute.

In addition, should any of the allocated devices get tainted with NoExecute after allocation and that effect is not tolerated, then all pods consuming the ResourceClaim get deleted to evict them. The scheduler will not let new pods reserve the claim while it has these tainted devices. Once all pods are evicted, the claim will get deallocated.

The maximum number of tolerations is 16.

This is an alpha field and requires enabling the DRADeviceTaints feature gate.
"%
       | optional
  },

  DeviceTaint = {
    effect
       | String
       | doc m%"
The effect of the taint on claims that do not tolerate the taint and through such claims on the pods using them. Valid effects are NoSchedule and NoExecute. PreferNoSchedule as used for nodes is not valid here.
"%,
    key
       | String
       | doc "The taint key to be applied to a device. Must be a label name.",
    timeAdded
       | time
       | doc m%"
TimeAdded represents the time at which the taint was added. Added automatically during create or update if not set.
"%
       | optional,
    value
       | String
       | doc "The taint value corresponding to the taint key. Must be a label value."
       | optional
  },

  DeviceToleration = {
    effect
       | String
       | doc m%"
Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule and NoExecute.
"%
       | optional,
    key
       | String
       | doc m%"
Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys. Must be a label name.
"%
       | optional,
    operator
       | String
       | doc m%"
Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a ResourceClaim can tolerate all taints of a particular category.
"%
       | optional,
    tolerationSeconds
       | Number
       | doc m%"
TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system. If larger than zero, the time when the pod needs to be evicted is calculated as <time when taint was adedd> + <toleration seconds>.
"%
       | optional,
    value
       | String
       | doc m%"
Value is the taint value the toleration matches to. If the operator is Exists, the value must be empty, otherwise just a regular string. Must be a label value.
"%
       | optional
  },

  NetworkDeviceData = {
    hardwareAddress
       | String
       | doc m%"
HardwareAddress represents the hardware address (e.g. MAC Address) of the device's network interface.

Must not be longer than 128 characters.
"%
       | optional,
    interfaceName
       | String
       | doc m%"
InterfaceName specifies the name of the network interface associated with the allocated device. This might be the name of a physical or virtual network interface being configured in the pod.

Must not be longer than 256 characters.
"%
       | optional,
    ips
       | Array String
       | doc m%"
IPs lists the network addresses assigned to the device's network interface. This can include both IPv4 and IPv6 addresses. The IPs are in the CIDR notation, which includes both the address and the associated subnet mask. e.g.: "192.0.2.5/24" for IPv4 and "2001:db8::5/64" for IPv6.

Must not contain more than 16 entries.
"%
       | optional
  },

  OpaqueDeviceConfiguration = {
    driver
       | String
       | doc m%"
Driver is used to determine which kubelet plugin needs to be passed these configuration parameters.

An admission policy provided by the driver developer could use this to decide whether it needs to validate them.

Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.
"%,
    parameters
       | v1Module.RawExtension
       | doc m%"
Parameters can contain arbitrary data. It is the responsibility of the driver developer to handle validation and versioning. Typically this includes self-identification and a version ("kind" + "apiVersion" for Kubernetes types), with conversion between different versions.

The length of the raw data must be smaller or equal to 10 Ki.
"%
  },

  ResourceClaim = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | objectMeta
       | doc "Standard object metadata"
       | optional,
    spec
       | ResourceClaimSpec
       | doc m%"
Spec describes what is being requested and how to configure it. The spec is immutable.
"%,
    status
       | ResourceClaimStatus
       | doc "Status describes whether the claim is ready to use and what has been allocated."
       | optional
  },

  ResourceClaimConsumerReference = {
    apiGroup
       | String
       | doc m%"
APIGroup is the group for the resource being referenced. It is empty for the core API. This matches the group in the APIVersion that is used when creating the resources.
"%
       | optional,
    name
       | String
       | doc "Name is the name of resource being referenced.",
    resource
       | String
       | doc "Resource is the type of resource being referenced, for example \"pods\".",
    uid
       | String
       | doc "UID identifies exactly one incarnation of the resource."
  },

  ResourceClaim = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array ResourceClaim
       | doc "Items is the list of resource claims.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | listMeta
       | doc "Standard list metadata"
       | optional
  },

  ResourceClaimSpec = {
    devices
       | DeviceClaim
       | doc "Devices defines how to request devices."
       | optional
  },

  ResourceClaimStatus = {
    allocation
       | AllocationResult
       | doc "Allocation is set once the claim has been allocated successfully."
       | optional,
    devices
       | Array AllocatedDeviceStatus
       | doc m%"
Devices contains the status of each device allocated for this claim, as reported by the driver. This can include driver-specific information. Entries are owned by their respective drivers.
"%
       | optional,
    reservedFor
       | Array ResourceClaimConsumerReference
       | doc m%"
ReservedFor indicates which entities are currently allowed to use the claim. A Pod which references a ResourceClaim which is not reserved for that Pod will not be started. A claim that is in use or might be in use because it has been reserved must not get deallocated.

In a cluster with multiple scheduler instances, two pods might get scheduled concurrently by different schedulers. When they reference the same ResourceClaim which already has reached its maximum number of consumers, only one pod can be scheduled.

Both schedulers try to add their pod to the claim.status.reservedFor field, but only the update that reaches the API server first gets stored. The other one fails with an error and the scheduler which issued it knows that it must put the pod back into the queue, waiting for the ResourceClaim to become usable again.

There can be at most 256 such reservations. This may get increased in the future, but not reduced.
"%
       | optional
  },

  ResourceClaimTemplate = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | objectMeta
       | doc "Standard object metadata"
       | optional,
    spec
       | ResourceClaimTemplateSpec
       | doc m%"
Describes the ResourceClaim that is to be generated.

This field is immutable. A ResourceClaim will get created by the control plane for a Pod when needed and then not get updated anymore.
"%
  },

  ResourceClaimTemplate = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array ResourceClaimTemplate
       | doc "Items is the list of resource claim templates.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | listMeta
       | doc "Standard list metadata"
       | optional
  },

  ResourceClaimTemplateSpec = {
    metadata
       | objectMeta
       | doc m%"
ObjectMeta may contain labels and annotations that will be copied into the ResourceClaim when creating it. No other fields are allowed and will be rejected during validation.
"%
       | optional,
    spec
       | ResourceClaimSpec
       | doc m%"
Spec for the ResourceClaim. The entire content is copied unchanged into the ResourceClaim that gets created from this template. The same fields as in a ResourceClaim are also valid here.
"%
  },

  ResourcePool = {
    generation
       | Number
       | doc m%"
Generation tracks the change in a pool over time. Whenever a driver changes something about one or more of the resources in a pool, it must change the generation in all ResourceSlices which are part of that pool. Consumers of ResourceSlices should only consider resources from the pool with the highest generation number. The generation may be reset by drivers, which should be fine for consumers, assuming that all ResourceSlices in a pool are updated to match or deleted.

Combined with ResourceSliceCount, this mechanism enables consumers to detect pools which are comprised of multiple ResourceSlices and are in an incomplete state.
"%,
    name
       | String
       | doc m%"
Name is used to identify the pool. For node-local devices, this is often the node name, but this is not required.

It must not be longer than 253 characters and must consist of one or more DNS sub-domains separated by slashes. This field is immutable.
"%,
    resourceSliceCount
       | Number
       | doc m%"
ResourceSliceCount is the total number of ResourceSlices in the pool at this generation number. Must be greater than zero.

Consumers can use this to check whether they have seen all ResourceSlices belonging to the same pool.
"%
  },

  ResourceSlice = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | objectMeta
       | doc "Standard object metadata"
       | optional,
    spec
       | ResourceSliceSpec
       | doc m%"
Contains the information published by the driver.

Changing the spec automatically increments the metadata.generation number.
"%
  },

  ResourceSlice = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array ResourceSlice
       | doc "Items is the list of resource ResourceSlices.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | listMeta
       | doc "Standard list metadata"
       | optional
  },

  ResourceSliceSpec = {
    allNodes
       | Bool
       | doc m%"
AllNodes indicates that all nodes have access to the resources in the pool.

Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
"%
       | optional,
    devices
       | Array Device
       | doc m%"
Devices lists some or all of the devices in this pool.

Must not have more than 128 entries.
"%
       | optional,
    driver
       | String
       | doc m%"
Driver identifies the DRA driver providing the capacity information. A field selector can be used to list only ResourceSlice objects with a certain driver name.

Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver. This field is immutable.
"%,
    nodeName
       | String
       | doc m%"
NodeName identifies the node which provides the resources in this pool. A field selector can be used to list only ResourceSlice objects belonging to a certain node.

This field can be used to limit access from nodes to ResourceSlices with the same node name. It also indicates to autoscalers that adding new nodes of the same type as some old node might also make new resources available.

Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set. This field is immutable.
"%
       | optional,
    nodeSelector
       | nodeSelector
       | doc m%"
NodeSelector defines which nodes have access to the resources in the pool, when that pool is not limited to a single node.

Must use exactly one term.

Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
"%
       | optional,
    perDeviceNodeSelection
       | Bool
       | doc m%"
PerDeviceNodeSelection defines whether the access from nodes to resources in the pool is set on the ResourceSlice level or on each device. If it is set to true, every device defined the ResourceSlice must specify this individually.

Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
"%
       | optional,
    pool
       | ResourcePool
       | doc "Pool describes the pool that this ResourceSlice belongs to.",
    sharedCounters
       | Array CounterSet
       | doc m%"
SharedCounters defines a list of counter sets, each of which has a name and a list of counters available.

The names of the SharedCounters must be unique in the ResourceSlice.

The maximum number of SharedCounters is 32.
"%
       | optional
  }
}
