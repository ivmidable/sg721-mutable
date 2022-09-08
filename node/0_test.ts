import { CosmWasmClient, SigningCosmWasmClient, Secp256k1HdWallet, GasPrice, Coin } from "cosmwasm";

import * as fs from 'fs';
import axios from 'axios';
import { ClientRequest } from "http";


const rpcEndpoint = "https://rpc.elgafar-1.stargaze-apis.com/";

const sg721_mutable = fs.readFileSync("../artifacts/sg721_mutable.wasm");

const mnemonic =
    "catch large carpet venue humble yellow ignore miss immune join lock camp";

const sender = "stars1uspey03w3hdvwcaqk2rypwt608eyvw3fkhtpdy";

async function setupClient(): Promise<SigningCosmWasmClient> {
    let gas = GasPrice.fromString("0.025ustars");
    let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'stars' });
    let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, wallet, { gasPrice: gas });
    return client;
}

describe("Cosmwasm Template Tests", () => {
    xit("Generate Wallet", async () => {
        let wallet = await Secp256k1HdWallet.generate(12);
        console.log(wallet.mnemonic);
    });

    //same as
    //junod tx wasm store artifacts/cw1_whitelist.wasm --from wallet --node https://rpc.uni.juno.deuslabs.fi --chain_id=uni-3 --gas-price=0.025ujunox --gas auto
    xit("Upload code to testnet", async () => {
        let client = await setupClient();
        let res = await client.upload(sender, sg721_mutable, "auto");
        console.log(res);

    }).timeout(50000);

    xit("Migrate NFT contract on testnet", async () => {
        //using the contract address migrate the NFT contract to the second code_id
        //then verify the minted NFT still exists.
        let code_id = 0;
        let client = await setupClient();
        let res = await client.migrate(sender, "contract_address", code_id, {}, "auto");
        console.log(res);
    }).timeout(100000);



});