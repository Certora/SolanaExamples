# IntegrityMint Example

## Based On The Blog Post
- [spl-token-2022](https://medium.com/certora/formal-verification-of-the-confidentiality-extension-of-spl-token-2022-5a75807f674e)

## Source Code
Located at: `program/src`

## Specs
Located at: `certora/spec`

## Compilation (Need to be done only if changed the source code or spec)
```bash
cd IntegrityOfMint
cargo build-sbf --arch=sbfv1 --features no-entrypoint certora
```

## Usage
```bash
cd certora

# Run the IntegrityOfProcessMintToFalse Rule
certoraRun IntegrityOfProcessMintToFalse.conf

# Run the IntegrityOfProcessMintTo Rule
certoraRun IntegrityOfProcessMintTo.conf
```

## Links
- [Integrity of Process Mint To](https://vaas-stg.certora.com/output/1512/6e6a20092bf34e8f930116c11b2404c5?anonymousKey=6412262a34999121d2ad4695a22a33c0d217ae52)
- [Integrity of Process Mint To False](https://vaas-stg.certora.com/output/1512/e3d64bbc065b4f3f8c788ce0a2ca954b?anonymousKey=a045cc5e62f7718666366c86e54d2231400eaf0a)