# kuberoutes

Tool to visualise network policies in Kubernetes with kubectl. Inspired by [ChrisCooney/kube-routes](https://github.com/ChrisCooney/kube-routes).

- [kuberoutes](#kuberoutes)
- [Usage](#usage)
- [Commands](#commands)

## Installation

```sh-session
brew install atkinchris/tools/kuberoutes
```

The tool depends on `kubectl`, which will be installed as a dependency by `brew`.

## Usage

```sh-session
Tool to visualise network policies in Kubernetes with kubectl

USAGE:
    kuberoutes [FLAGS] [OPTIONS]

FLAGS:
    -d, --default-deny    pods are isolated by default
    -h, --help            Prints help information
    -n, --namespace       namespace to query the network policies of
    -V, --version         Prints version information

OPTIONS:
    -l, --labels <LABELS>    labels to match against network policy selector
```
