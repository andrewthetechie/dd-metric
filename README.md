# dd-metric

Send a datadog metric via the datadog api.

Builds a CLI client

<p align="center">
    <a href="https://github.com/andrewthetechie/dd-metric" target="_blank">
        <img src="https://img.shields.io/github/last-commit/andrewthetechie/dd-metric" alt="Latest Commit">
    </a>
    <img src="https://img.shields.io/badge/license-MIT-green">
    <img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/andrewthetechie/dd-metric?label=Latest%20Release">
    <br />
    <a href="https://github.com/andrewthetechie/dd-metric/issues"><img src="https://img.shields.io/github/issues/andrewthetechie/dd-metric" /></a>
    <img alt="GitHub Workflow Status Test and Lint (branch)" src="https://img.shields.io/github/workflow/status/andrewthetechie/dd-metric/Tests/main?label=Test and Lint">
    <img alt="GitHub Workflow Status Build and Docker (main)" src="https://img.shields.io/github/workflow/status/andrewthetechie/dd-metric/Release/main?label=Build and Docker">
    <br />
    <img alt="GitHub all releases" src="https://img.shields.io/github/downloads/andrewthetechie/dd-metric/total?color=green">
    <a href='https://hub.docker.com/r/andrewthetechie/dd-metric' target="_blank"><img alt="Docker Pulls" src="https://img.shields.io/docker/pulls/andrewthetechie/dd-metric">
    <img alt="Docker Image Size (latest by date)" src="https://img.shields.io/docker/image-size/andrewthetechie/dd-metric?label=Docker%20Image%20Size"></a>
</p>

# Installation

Download the appropriate binary from a release. Builds are available for Linux, OSX, and Windows in amd64 and arm64 arch.

# Usage

Set your datadog API in your environment

```shell
export DD_API_KEY=yourapikey
```

```shell
dd-metric 0.1.0
Andrew Herrington <andrew.the.techie@gmail.com>
Send a metric to datadog via the api

Uses the datadog API to send either a gauge, counter, or histogram to the datadog api Sends one
metric per invocation Requires DD_API_KEY to be set in the environment to function.

USAGE:
    dd-metric [OPTIONS] --name <NAME>

OPTIONS:
    -h, --help
            Print help information

    -n, --name <NAME>
            Name of the metric to send

    -o, --outputs <OUTPUTS>
            Optional places to output the metric to as a comma separated list of destinations. Valid
            destinations are: stdout, api

            [default: api]

    -t, --type <TYPE>
            Type of the metric to send

            [default: counter]

        --tags <TAGS>
            Optional tags to add to sent metric in the format key:value,key2:value2

            [default: ]

    -v, --value <VALUE>
            Metric value

            [default: 1]

    -V, --version
            Print version information

        --verbose
            Will print verbose output to stdout
```

## Example

Send a counter metric with a value of 7 named test.test_counter

```shell
dd-metric --value 7 --name test.test_counter
```

Tag a histogram metric

```shell
dd-metric --value 22 --type histogram --name test.test_historgram --tags 'tag1:value1,tag2:value2'
```

Output only to stdout, no API calls. Good for testing

```shell
dd-metric --value 7 --name test.test_counter --outputs stdout
{"m":"test.test_counter","v":7,"e":1657943168,"t":[]}
counter test.test_counter sent to Datadog with value 7.0
```
