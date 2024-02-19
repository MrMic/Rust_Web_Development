enum TaskPriority {
  "user-blocking",
  "user-visible",
  "background"
};

dictionary SchedulerPostTaskOptions {
  AbortSignal signal;
  TaskPriority priority;
  [EnforceRange] unsigned long long delay = 0;
};

callback SchedulerPostTaskCallback = any ();

[Exposed=(Window, Worker)]
interface Scheduler {
  Promise<any> postTask(SchedulerPostTaskCallback callback,
                        optional SchedulerPostTaskOptions options = {});
};

[Exposed=(Window, Worker)]
interface TaskPriorityChangeEvent : Event {
  constructor(DOMString type, TaskPriorityChangeEventInit priorityChangeEventInitDict);

  readonly attribute TaskPriority previousPriority;
};

dictionary TaskPriorityChangeEventInit : EventInit {
  required TaskPriority previousPriority;
};

dictionary TaskControllerInit {
  TaskPriority priority = "user-visible";
};

[Exposed=(Window,Worker)]
interface TaskController : AbortController {
  constructor(optional TaskControllerInit init = {});

  [Throws]
  undefined setPriority(TaskPriority priority);
};

dictionary TaskSignalAnyInit {
  (TaskPriority or TaskSignal) priority = "user-visible";
};

[Exposed=(Window, Worker)]
interface TaskSignal : AbortSignal {
  [NewObject] static TaskSignal _any(sequence<AbortSignal> signals, optional TaskSignalAnyInit init = {});

  readonly attribute TaskPriority priority;

  attribute EventHandler onprioritychange;
};

partial interface mixin WindowOrWorkerGlobalScope {
  [Replaceable] readonly attribute Scheduler scheduler;
};
