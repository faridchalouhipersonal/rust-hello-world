# Rust Hello World

This project is a personal test of a Rust application within a Docker container, complete with a CI pipeline.

## Overview

This repository demonstrates a basic setup for developing, testing, and deploying a Rust application in Docker. GitHub Actions is configured to automatically test changes to ensure stability and avoid regressions.
w
## Local Setup

To get started with local testing:

1. **Clone the repository**  
   Clone this repository to your local machine:
   ```bash
   git clone https://github.com/faridchalouhipersonal/rust-hello-world.git
    ```
2. **Install Rust**

    Ensure that Rust is installed and up-to-date. You can install Rust via [rustup](https://rustup.rs/)
   
3. **Build the project**

The below command will build the code
```bash
cargo build
```

4. **Test the project**

The below command will test the code
```bash
cargo test
```
5. **Run the project**

The below command will run the code

```bash
cargo run
```
