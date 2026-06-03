# proto-terraform

A [proto](https://moonrepo.dev/proto) WASM plugin for managing [Terraform](https://www.terraform.io/) versions.

## Why WASM?

While Terraform distributes pre-built binaries, a WASM plugin is necessary because:

- Version resolution uses HashiCorp's custom release API (`releases.hashicorp.com/terraform/index.json`), not GitHub tags
- Download URLs follow a non-standard pattern (`releases.hashicorp.com`, not GitHub releases)
- Supports detection of `.terraform-version` files (used by tfenv and similar tools)

## Installation

Add to your `.prototools` file:

```toml
[plugins]
terraform = "github://ageha734/proto-terraform"
```

Or install directly:

```bash
proto plugin add terraform "github://ageha734/proto-terraform"
```

## Usage

```bash
# Install a specific version
proto install terraform 1.9.0

# Install latest
proto install terraform latest

# Use .terraform-version file
echo "1.9.0" > .terraform-version
proto install terraform
```

## Supported Platforms

| OS      | Architectures     |
|---------|-------------------|
| Linux   | x64, arm64, x86, arm |
| macOS   | x64, arm64        |
| Windows | x64, x86          |

## Version Detection

The plugin detects versions from:

- `.terraform-version` (tfenv compatible)
- `.prototools`

## Development

### Prerequisites

- Rust toolchain with `wasm32-wasip1` target
- proto (for testing)

### Build

```bash
cargo build --target wasm32-wasip1 --release
```

### Test

```bash
cargo test
```

### Local Testing

```bash
cargo build --target wasm32-wasip1 --release
proto plugin add terraform source:./target/wasm32-wasip1/release/proto_terraform.wasm
proto install terraform latest
terraform --version
```

## License

MIT
