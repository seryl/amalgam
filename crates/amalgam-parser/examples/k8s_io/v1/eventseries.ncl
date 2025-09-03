# EventSeries contain information on series of events, i.e. thing that was/is happening continuously for some time.
{
  count | optional | Number | doc "Number of occurrences in this series up to the last heartbeat time",
  lastObservedTime | optional | String | doc "Time of the last occurrence observed",
}
