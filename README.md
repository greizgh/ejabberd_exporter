# ejabberd_exporter

[Prometheus](https://prometheus.io/) exporter for ejabberd metrics.
Tested on ejabberd 18.01 on debian.

## Project status

**WIP**
This project should not be considered stable.
Until it hits version 1.0.0, metrics names may change in upcoming release without warning.

This is **not** a *production ready* tool, use at your own risks.

## Building

    cargo build --release

To create a .deb package, install [cargo-deb](https://github.com/mmstick/cargo-deb#readme), then:

    cargo deb

## Running

    ejabberd-exporter

Make sure you have sufficient priviledges to run `ejabberdctl`.

## Caveats

* This exporter is synchronous, as per recommended by prometheus documentation.
It means this will takes some time to gather all exported values.
You may need to increase the timeout around 10s.
* For now, data is retrieved from ejabberdctl command.
At some point it will be retrieved from the Rest API.
* This has not been tested on a large ejabberd instance.
Feel free to open an issue or a merge request to improve support.

## Prometheus configuration

```json
scrape_configs:
  - job_name: ejabberd
    scrape_interval: 1m
    scrape_timeout: 10s
    static_configs:
      - targets:
        - '192.168.1.2:9334'
```
