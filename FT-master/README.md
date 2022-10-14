Fungible Token (FT)
===================

Example implementation of a [Fungible Token] contract which uses [near-contract-standards] and [simulation] tests. This is a contract-only example.

  [Fungible Token]: https://nomicon.io/Standards/FungibleToken/Core
  [near-contract-standards]: https://github.com/near/near-sdk-rs/tree/master/near-contract-standards
  [simulation]: https://github.com/near/near-sdk-rs/tree/master/near-sdk-sim


## Building

To build run:
```bash
./scripts/build.sh
```

Using this contract
===================

### Quickest deploy

You can build and deploy this smart contract to a development account. [Dev Accounts](https://docs.near.org/concepts/basics/account#dev-accounts) are auto-generated accounts to assist in developing and testing smart contracts. Please see the [Standard deploy](#standard-deploy) section for creating a more personalized account to deploy to.

```bash
near dev-deploy --wasmFile res/fungible_token.wasm --helperUrl https://near-contract-helper.onrender.com
```

Behind the scenes, this is creating an account and deploying a contract to it. On the console, notice a message like:

>Done deploying to dev-1234567890123

In this instance, the account is `dev-1234567890123`. A file has been created containing a key pair to
the account, located at `neardev/dev-account`. To make the next few steps easier, we're going to set an
environment variable containing this development account id and use that when copy/pasting commands.
Run this command to the environment variable:

export NEAR_ENV=testnet
export OWNER_ID=hapi-test.testnet
export CONTRACT_ID=token.$OWNER_ID
export AML_ID=contract.$OWNER_ID

```bash
source neardev/dev-account.env
```

You can tell if the environment variable is set correctly if your command line prints the account name after this command:
```bash
echo $CONTRACT_NAME
```

The next command will initialize the contract using the `new` method:

```bash
near call $CONTRACT_ID new '{"owner_id": "'$OWNER_ID'", "total_supply": "1000000000000000000000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Hapi powered token", "symbol": "HAPI", "decimals": 18 }, "aml_account_id": "'$AML_ID'"}' --accountId $CONTRACT_ID
```

To get the fungible token metadata:

```bash
near view $CONTRACT_NAME ft_metadata
```

### Standard deploy

This smart contract will get deployed to your NEAR account. For this example, please create a new NEAR account. Because NEAR allows the ability to upgrade contracts on the same account, initialization functions must be cleared. If you'd like to run this example on a NEAR account that has had prior contracts deployed, please use the `near-cli` command `near delete`, and then recreate it in Wallet. To create (or recreate) an account, please follow the directions on [NEAR Wallet](https://wallet.near.org/).

Switch to `mainnet`. You can skip this step to use `testnet` as a default network.

    export NEAR_ENV=mainnet

In the project root, log in to your newly created account  with `near-cli` by following the instructions after this command:

    near login

To make this tutorial easier to copy/paste, we're going to set an environment variable for your account id. In the below command, replace `MY_ACCOUNT_NAME` with the account name you just logged in with, including the `.near`:

    ID=MY_ACCOUNT_NAME

You can tell if the environment variable is set correctly if your command line prints the account name after this command:

    echo $ID

Now we can deploy the compiled contract in this example to your account:

    near deploy --wasmFile res/fungible_token.wasm --accountId $ID

FT contract should be initialized before usage. You can read more about metadata at ['nomicon.io'](https://nomicon.io/Standards/FungibleToken/Metadata.html#reference-level-explanation). Modify the parameters and create a token:

    near call $ID new '{"owner_id": "'$ID'", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Example Token Name", "symbol": "EXLT", "decimals": 8 }}' --accountId $ID

Get metadata:

    near view $ID ft_metadata


Transfer Example
---------------

Let's set up an account to transfer some tokens to. These account will be a sub-account of the NEAR account you logged in with.

    near create-account bob.$ID --masterAccount $ID --initialBalance 1

Add storage deposit for Bob's account:

    near call $ID storage_deposit '' --accountId bob.$ID --amount 0.00125


Check balance of Bob's account, it should be `0` for now:

    near view $ID ft_balance_of '{"account_id": "'bob.$ID'"}'

Transfer tokens to Bob from the contract that minted these fungible tokens, exactly 1 yoctoNEAR of deposit should be attached:

    near call $ID ft_transfer '{"receiver_id": "'bob.$ID'", "amount": "19"}' --accountId $ID --amount 0.000000000000000000000001


near deploy token.hapi-test.testnet --wasmFile=res/hapi_token_release.wasm
