# Rust time tracker

## Presentation
This is a small utility to keep track of your time spent in your projects.

# Usage
## Tasks and projects management
Enter TUI
```bash
rtt [-V] [h]
```

Add a project or a task to it. If the project does not exists it will be created
```bash
rtt create unique_project_name [-t "task name"=duration[m|h|d]]
```

Delete a project or a task. if a task is provided the project itself won't be deleted
```bash
rtt delete [-f] unique_project_name "task name"
```

Show time spent on a project or on tasks
```
rtt show [--pretty|-p] unique_project_name ["task name"] ["task2"]
```

Archive/Unarchive a project
```bash
rtt archive unique_project_name
rtt unarchive unique_project_name
```

Amend time spent on a project
```bash
rtt amend +-00m|h unique_project_name "task name"
```
## :movie_camera: Recording task time 
Did you ever started a task but forgot to write down where you started ? Or do you slice your work time with many breaks? You can record the time you are spending, just like you would do with a camera!

:warning: Only one record at a time can take place


Start the recording
```bash
rtt start unique_project_name "task name"
```

Pause/resume the recording
```bash
rtt pause
```

Stop the recording
```bash
rtt stop
```
