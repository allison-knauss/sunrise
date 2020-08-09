# telemetry

The telemetry server is a system-wide network-attached logging library. All other services call this service's `error`, `warning`, `info`, and `trace` endpoints rather than any other logging mechanism. This enables unified project-wide tracing.
