# Greet Program

This is a simple Rust CLI program that greets a person a specified number of times. It uses the `clap` crate to parse command-line arguments.


## Usage

1. Build the program:
    ```sh
    cargo build --release
    ```

2. Run the program:
    ```sh
    ./target/release/<your-program-name> --name <name> --count <number>
    ```

    Replace `<your-program-name>`, `<name>`, and `<number>` with appropriate values. For example:
    ```sh
    ./target/release/greet --name Alice --count 3
    ```

    This will output:
    ```
    Hello Alice!
    Hello Alice!
    Hello Alice!
    ```

## Command Line Arguments

- `--name` or `-n`: The name of the person to greet (required).
- `--count` or `-c`: The number of times to greet (optional, default is 1).

## Example

```sh
./target/release/greet --name Bob --count 2
