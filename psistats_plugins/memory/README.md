# Psistats Memory Plugin

## Configuration

```
# Memory Reporter
#
# Reports the total available memory and total free memory.
#
[[plugin]]
name="memory"
enabled=true
interval=5
```

## Report

```javascript
{
  "reporter": "memory",
  "hostname": "machine-name",
  "value": [24824197, 5455863] // total memory, free memory - in kb
}