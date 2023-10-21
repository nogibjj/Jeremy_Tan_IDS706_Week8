[![Python CI/CD Pipeline](https://github.com/nogibjj/Jeremy_Tan_IDS706_Week8/actions/workflows/pythonCI.yml/badge.svg)](https://github.com/nogibjj/Jeremy_Tan_IDS706_Week8/actions/workflows/pythonCI.yml)
[![Rust CI/CD Pipeline](https://github.com/nogibjj/Jeremy_Tan_IDS706_Week8/actions/workflows/rustCI.yml/badge.svg)](https://github.com/nogibjj/Jeremy_Tan_IDS706_Week8/actions/workflows/rustCI.yml)
# Jeremy Tan IDS706 Week 8 
The project is a command-line interface (CLI) tool implemented in both Rust and Python for encrypting and decrypting messages using the AES-256 CBC PKCS#7 encryption algorithm. It supports two modes: encryption and decryption.

## Rust Implementation:
The Rust version uses the aes_cbc crate for encryption and decryption. It employs the clap library for parsing command-line arguments. The CLI takes input for mode (encrypt or decrypt), the message to be encrypted or decrypted, and optional arguments for the encryption key and initialization vector (IV). It measures and logs the elapsed time for the operation and appends this information to a Markdown file.

### Preparation and Dependency Installation: 
1. open codespaces 
2. wait for codespaces to be built 
3. build: `cargo build` for dependencies installation
4. run: `cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt` or use your own string

### Check Format and Test Erros: 
1. Format code `make format`
2. Lint code `make lint`
3. Test coce `make test`

## Python Implementation:
The Python version features a similar CLI, utilizing the argparse module for argument parsing. It also offers options for mode, input text, and optional key and IV in base64 format. The CLI measures execution time, logs it, and displays the encrypted or decrypted message along with the elapsed time.


### Preparation: 
1. git clone the repo
2. install: `make python_install`
3. run: `python main.py encrypt "Hello World"` or use your own string   

### Check Format and Test Errors: 
1. Format code `make python_format`
2. Lint code `make python_lint`
3. Test coce `make python_test`

## Speed and Resource Usage:
[Link to Rust runtime Markdown File](https://github.com/nogibjj/Jeremy_Tan_IDS706_Week8/blob/main/rust_times.md)
[Link to Python runtime Markdown File](https://github.com/nogibjj/Jeremy_Tan_IDS706_Week8/blob/main/python_times.md)

You can view how long it takes to encrypt and decrypt the same messages above. Based on the speed, it's obvious Rust run on average 400 times faster than Python and we can infer why the resource usage is vastly smaller than Python. Rust outperforms Python in speed primarily due to its static typing, zero-cost abstractions, and absence of a Global Interpreter Lock (GIL). Rust's strict typing allows for more efficient compilation, while its ownership system enables high-performance abstractions without sacrificing safety. Additionally, Rust manages memory directly, avoiding the overhead of Python's garbage collector. The language also offers fine-grained control over memory, enabling low-level optimizations. These factors, combined with an optimized compiler and a performance-centric standard library, contribute to Rust's reputation for speed.

## References
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* https://github.com/DaGenix/rust-crypto/
* https://github.com/nogibjj/rust-data-engineering