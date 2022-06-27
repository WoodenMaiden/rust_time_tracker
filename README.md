# Rust time tracker

## Presentation
This is a small utility to keep track of your time spent in your projects.

# Usage
Enter TUI
```bash
rtt [-V] [h]
```
## Project management

Add a project
```bash
rtt create [-t "task name"=duration[m|h|d]] unique_project_name
```

Delete a project
```bash
rtt delete [-f] unique_project_name
```

Show time spent on a project
```
rtt show [--pretty|-p] unique_project_name ["task name"] ["task2"]
```

Archive/Unarchive a project
```bash
rtt archive unique_project_name
rtt unarchive unique_project_name
```

## Tasks manipulation
These commands will allow to manipulate time spent on projects' tasks

Add a task to a project
```bash
rtt add [-t|--time 00m|h] unique_project_name "task name"
```

Delete a task from a project
```bash
rtt delete [-f] unique_project_name "task name"
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
