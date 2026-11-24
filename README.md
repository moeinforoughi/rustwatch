# rustwatch

A small Rust practice project. It prints basic system information, shows a simulated list of services with different statuses, and counts `ERROR` lines in a log file.

## Run

```sh
cargo run
```

The program expects a `rustwatch.log` file in the project root. If it is missing, it reports the error and still prints the system and service sections.

## What it does

- Prints OS, architecture, and hostname (`COMPUTERNAME`).
- Models services with a `ServiceStatus` enum (`Running`, `Stopped`, `Starting`, `Unknown`) and prints each one.
- Counts how many services are running.
- Reads `rustwatch.log` and counts lines containing `ERROR`.

## Notes

This is a learning repo, not a real monitoring tool. The service list is hard-coded.
