# CronJobStatus represents the current state of a cron job.
{
  active | optional | Array ObjectReference | doc "A list of pointers to currently running jobs.",
  lastScheduleTime | optional | String | doc "Information when was the last time the job was successfully scheduled.",
  lastSuccessfulTime | optional | String | doc "Information when was the last time the job successfully completed.",
}
