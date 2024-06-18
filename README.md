# Historical Gas Price Estimation

The rust code extimates the Gas price on Arbitrum One mainnet.

It does this by effective gas price of each transaction in a block fetched by the historical timestamp and then averaging it across all transactions in that block.

## To run

Clone the repository and navigate to the root folder.

Install dependencies using the command `cargo add`.

Make sure you have Rust installed in your machine. You can check this by running the cmd `rustc --version`.

Create a `.env` file and add 2 environment variables
- ARB_RPC_URL (alchemy or any other node provider rpc url)
- ARB_API_KEY (From etherscan or arbiscan)

Execute the code by running `cargo run`