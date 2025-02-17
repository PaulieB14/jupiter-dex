# Substreams CLI Reference

The Substreams command line interface (CLI) is the primary tool for sending requests and receiving data from Substreams endpoints.

## Package Specification

When a package is specified in commands, you can use any of the following:
- Local `substreams.yaml` configuration files
- Local `.spkg` package files
- Remote `.spkg` package URLs
- Local directory containing a `substreams.yaml` file

If no package is specified, the CLI will look for a `substreams.yaml` file in the current directory.

## Commands

### init

Initializes a new Substreams project through an interactive prompt.

```bash
substreams init
```

The command will ask several questions to customize your project setup for your target blockchain.

### build

Performs the following actions:
1. Generates Protobufs specified in `substreams.yaml`
2. Compiles Rust code
3. Creates a Substreams package file (`.spkg`)

```bash
substreams build
```

### run

Connects to a Substreams endpoint and processes data.

```bash
substreams run -e mainnet.eth.streamingfast.io:443 \
   -t +1 \
   ./substreams.yaml \
   module_name
```

Parameters:
- `-e`: Endpoint URL of the Substreams provider
- `-t` or `--stop-block`: Specifies the stop block (e.g., `+1` for a single block)
- `-s` or `--start-block`: Specifies the start block
- Last argument: Module name to run (must be defined in manifest)

#### Headers

Use the `-H` option to pass custom headers with the gRPC request:

```bash
substreams run -e mainnet.eth.streamingfast.io:443 \
   -t +1 \
   -H "X-Sf-Substreams-Parallel-Jobs: 20" \
   ./substreams.yaml \
   module_name
```

Common headers:
- `X-Sf-Substreams-Parallel-Jobs`: Sets number of parallel jobs (default: 10)

#### Output Formats

Use `-o` or `--output` to specify the output format:
- `ui`: Terminal UI with progress information and logs
- `json`: Indented JSON stream of data
- `jsonl`: Line-delimited JSON stream

Example with JSON output:
```bash
substreams run -e mainnet.eth.streamingfast.io:443 \
    package.spkg \
    module_name -o json
```

### gui

Launches a terminal-based graphical interface for viewing Substreams data.

#### GUI Navigation Shortcuts

| Function | Keys |
|----------|------|
| Switch screen (Request/Progress/Output) | `tab` |
| Restart | `r` |
| Quit | `q` |
| Navigate Blocks Forward | `p` |
| Navigate Blocks Backward | `o` |
| Go To Block | `= + block number + enter` |
| Navigate Modules Forward | `i` |
| Navigate Modules Backward | `u` |
| Search | `/ + text + enter` |
| Help | `?` |

#### Replay Mode

The GUI saves streamed data to `replay.log`. Use `--replay` to reload previous data without connecting to the server.

### info

Displays package information from a yaml or spkg file.

```bash
substreams info ./substreams.yaml
```

Shows:
- Package name and version
- Documentation
- Module definitions
- Initial blocks
- Module types and hashes

### graph

Generates a visual representation of the package modules in mermaid-js format.

```bash
substreams graph ./substreams.yaml
```

The output can be visualized using the Mermaid Live Editor.

### codegen

Generates code for specific sinks using Substreams modules as input.

Available generators:

1. Subgraph Generator:
```bash
substreams codegen subgraph
```
Generates a Substreams-powered Subgraph project.

2. SQL Generator:
```bash
substreams codegen sql
```
Generates a SQL-based Substreams project.

### inspect

Provides detailed internal information about a package's file structure.

```bash
substreams inspect ./substreams.yaml
```

Useful for debugging and examining package internals.

### Help

Access CLI help documentation:

```bash
substreams -h
```

This displays all available commands and their brief descriptions.

## Note

The `pack` command is deprecated. Use `build` instead for creating deployable packages.
