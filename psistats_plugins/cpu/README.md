# Psistats CPU Plugin

## Configuration

```
# CPU Reporter
#
# Reports current cpu usage in percentage.
#
# Configuration:
# config.combined - If set to false, then an array will be reported
#                     with usage percent for each core. Otherwise Reports
#                     total cpu usage.
[[plugin]]
name="cpu"
enabled=true
interval=1
config.combined = true
```

## Report

```javascript
// combined = true
{
  "reporter": "cpu",
  "hostname": "machine-name",
  "value": 34.24
}

// combined = false
{
  "reporter": "cpu",
  "hostname": "machine-name"
  "value:": [
    10.31,
    9.13
    4.42
    55.24
  ]
}