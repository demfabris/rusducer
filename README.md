# Rusducer

## About

General purpose reducer-like application for your common system tasks

## Basic usage

```yaml
---
- action:
    id: MOVE_MY_FILES
    description: Put files from place A to place B
    files:
      - "~/file"
      - "~/another_file"
    move:
      destination:
        - "~/Documents"
        - "~/Pictures"
      replace: true
    command: "sh -c custom_script.sh"
    side_effect: "..."
```
