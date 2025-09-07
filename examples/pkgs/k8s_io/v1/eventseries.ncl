# Module: k8s.io.v1

# EventSeries contain information on series of events, i.e. thing that was/is happening continuously for some time.
{
  count
     | Number
     | doc "Number of occurrences in this series up to the last heartbeat time"
     | optional,
  lastObservedTime
     | String
     | doc "Time of the last occurrence observed"
     | optional
}