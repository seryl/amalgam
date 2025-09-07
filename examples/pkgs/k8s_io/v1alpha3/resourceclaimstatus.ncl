let allocationResult = import "./AllocationResult.ncl" in
let allocatedDeviceStatus = import "./AllocatedDeviceStatus.ncl" in
let resourceClaimConsumerReference = import "./ResourceClaimConsumerReference.ncl" in

# Module: k8s.io.v1alpha3

# ResourceClaimStatus tracks whether the resource has been allocated and what the result of that was.
{
  allocation
     | allocationResult
     | doc "Allocation is set once the claim has been allocated successfully."
     | optional,
  devices
     | Array allocatedDeviceStatus
     | doc m%"
Devices contains the status of each device allocated for this claim, as reported by the driver. This can include driver-specific information. Entries are owned by their respective drivers.
"%
     | optional,
  reservedFor
     | Array resourceClaimConsumerReference
     | doc m%"
ReservedFor indicates which entities are currently allowed to use the claim. A Pod which references a ResourceClaim which is not reserved for that Pod will not be started. A claim that is in use or might be in use because it has been reserved must not get deallocated.
In a cluster with multiple scheduler instances, two pods might get scheduled concurrently by different schedulers. When they reference the same ResourceClaim which already has reached its maximum number of consumers, only one pod can be scheduled.
Both schedulers try to add their pod to the claim.status.reservedFor field, but only the update that reaches the API server first gets stored. The other one fails with an error and the scheduler which issued it knows that it must put the pod back into the queue, waiting for the ResourceClaim to become usable again.
There can be at most 256 such reservations. This may get increased in the future, but not reduced.
"%
     | optional
}