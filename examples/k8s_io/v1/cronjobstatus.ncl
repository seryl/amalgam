# Module: k8s.io.v1.cronjobstatus

let objectreference = import "./objectreference.ncl" in

{
  # CronJobStatus represents the current state of a cron job.
  CronJobStatus = {
    active | optional | Array objectreference.ObjectReference | doc "A list of pointers to currently running jobs.",
    lastScheduleTime | optional | String | doc "Information when was the last time the job was successfully scheduled.",
    lastSuccessfulTime | optional | String | doc "Information when was the last time the job successfully completed.",
  },
}
