# EscrowPlayersUi
This is a proof of concept to ensure that a set of rules are being fulfilled. This set of rules are attached to a different project of a different nature. The idea is to escrow the users' NFTs so users cannot violate those rules while they are the owners of their NFT.

For that, a PDA will take possession of the NFT

Inside the root dir, you can find 2 different apps:
1. The front-end (./scrow-for-players-ui)
2. The back-end (./scrow-for-players-on-chain)

## Run the front-end

It was generated with [Angular CLI](https://github.com/angular/angular-cli) version 12.0.5.

Inside the front-end dir, run `npm run start` for a dev server. Navigate to `http://localhost:4200/`. The app will automatically reload if you change any of the source files.

## Run the back-end

Ensure you have your local solana-test-validator running, there is a dev wallet attached to your local configuration and it has more than 1 SOL.

Inside the back-end dir, run `cargo build-bpf` to generate the compiled version of the Rust program.

Deploy the program to your local cluster with `solana program deploy /target/deploy/bpf_program_template.so`

_Note: Only Instruction 0 is implemented at present: Deposit the NFT. The next instruction will handle the withdrawal of the NFT previously deposited._