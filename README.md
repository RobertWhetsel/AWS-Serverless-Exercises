# Rust Lambda Function

A serverless function written in Rust, deployed using AWS SAM.

## Prerequisites
- Rust installed
- AWS SAM CLI installed
- Docker (for local testing)
- x86_64-linux-musl-gcc (for cross-compilation)

## Project Structure
```
.
├── Cargo.toml          # Rust dependencies and project configuration
├── handler.rs          # Lambda function implementation
├── Makefile           # Build automation
└── template.yaml      # AWS SAM template
```

## Dependencies
- lambda_runtime (0.8)
- serde (1.0)
- serde_json (1.0)
- tokio (1.0)

## Building
```bash
make build
```
This will:
1. Build the release binary targeting Linux
2. Copy the binary to a file named 'bootstrap'

## Cleaning
```bash
make clean
```
This removes the build artifacts and bootstrap file.

## Deployment Configuration
The function is configured with:
- Runtime: provided.al2
- Architecture: x86_64
- Memory: 128MB
- Timeout: 30 seconds

## Deploying
1. Build the project:
```bash
make build
```

2. Deploy using SAM:
```bash
sam deploy --guided
```

## Local Testing
You can test the function locally using SAM:
```bash
sam local invoke RustFunction
```

## Cleaning Up
To remove build artifacts:
```bash
make clean
```

To delete the deployed stack:
```bash
sam delete
