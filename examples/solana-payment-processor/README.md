# Sol Payments

SolPayments is a smart contract program build for the [Solana blockchain](https://solana.com/) that allows merchants to receive crypto-currency payments at their online shops.

More information: [SolPayments Official Website](https://solpayments.com/)

## Benefits of using SolPayments

- **Low fees** - SolPayments charges merchants just 0.3% of the transaction value.  Additionally, the fees charged to buyers for sending the crypto-currency payments are almost free (just a few cents)
- **Fast** - payments made through SolPayments are completed in a few seconds
- **Non-custodial** - SolPayments never takes custody of payments made to any merchants that use it.  You are always in full control of your money.

## Program API

All the instructions supported by the Sol Payments program are documented [here](src/instruction.rs).

## Contributing

### Environment Setup

1. Install Rust from [https://rustup.rs/](https://rustup.rs/)
2. Install Solana v1.5.0 or later from [https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool](https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool)

### Build and test for program compiled natively

```sh
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF

```sh
$ cargo build-bpf
$ cargo test-bpf
```
