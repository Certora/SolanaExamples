# SolanaExamples

This repository contains examples of Solana applications that can be formally verfied with the Certora Prover.

## Prerequisites

See the [Certora Solana Prover documentation](https://docs.certora.com/en/latest/docs/solana/index.html) 
for instruction about how to install the prerequisites.

Furthermore, you will need to install [just](https://just.systems/man/en/).

## Structure and Usage

Each example has a `certora` subdirectory which contains files to perform the formal verification.
Inside of `certora/conf` there are several `.conf` files which are the config files for `certoraSolanaProver` and 
run the rules in the example.

For instance, to run the verification on the `first_example` run the following:
```bash
cd cvlr_by_example/first_example/certora/conf/`
certoraSolanaProver Default.conf
```
This will build the code and run the verification.