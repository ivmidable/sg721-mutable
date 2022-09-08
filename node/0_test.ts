import { CosmWasmClient, SigningCosmWasmClient, Secp256k1HdWallet, GasPrice, Coin } from "cosmwasm";

import * as fs from 'fs';
import axios from 'axios';
import { ClientRequest } from "http";


const rpcEndpoint = "https://rpc.elgafar-1.stargaze-apis.com/";

const sg721_mutable = fs.readFileSync("../artifacts/sg721_mutable.wasm");

const mnemonic =
    "catch large carpet venue humble yellow ignore miss immune join lock camp";

describe("Cosmwasm Template Tests", () => {
    xit("Generate Wallet", async () => {
        let wallet = await Secp256k1HdWallet.generate(12);
        console.log(wallet.mnemonic);
    });

    xit("Get Testnet Tokens", async () => {
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'stars' });
        console.log(await wallet.getAccounts());
        /*try {
            let res = await axios.post("https://faucet.uni.juno.deuslabs.fi/credit", { "denom": "ujunox", "address": "juno10c3slrqx3369mfsr9670au22zvq082jaej8ve4" });
            console.log(res);
        } catch (e) {
            console.log(e);
        }*/
    }).timeout(10000);

    xit("Send Testnet Tokens", async() => {

    }).timeout(50000);

    //same as
    //junod tx wasm store artifacts/cw1_whitelist.wasm --from wallet --node https://rpc.uni.juno.deuslabs.fi --chain_id=uni-3 --gas-price=0.025ujunox --gas auto
    it("Upload code to testnet", async () => {
        let gas = GasPrice.fromString("0.025ustars");
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'stars' });
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, wallet, {gasPrice:gas});
        let res = await client.searchTx({sentFromOrTo: "stars1fvhcnyddukcqfnt7nlwv3thm5we22lyxyxylr9h77cvgkcn43xfsvgv0pl" });
        //let res = await client.upload("stars1uspey03w3hdvwcaqk2rypwt608eyvw3fkhtpdy", sg721_mutable, "auto");
        console.log(res);
        //calculateFee()
        //console.log(JSON.stringify(res.logs[0].events));
    }).timeout(50000);

    xit("Instantiate code on testnet", async() => {
        let gas = GasPrice.fromString("0.025ujunox");
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'juno' });
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, wallet, {gasPrice:gas});
        let res = await client.instantiate("juno10c3slrqx3369mfsr9670au22zvq082jaej8ve4", 3019, {admins:["juno10c3slrqx3369mfsr9670au22zvq082jaej8ve4"], mutable:false}, "whitelist", "auto");
        console.log(res);
        ///client.execute("juno10c3slrqx3369mfsr9670au22zvq082jaej8ve4", "", )
    }).timeout(20000);

    xit("Instantiate NFT contract", async() => {

    }).timeout(50000);

    xit("Execute a mint on testnet", async() => {
        let gas = GasPrice.fromString("0.025ujunox");
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'juno' });
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, wallet, {gasPrice:gas});
        let res = await client.execute("juno10c3slrqx3369mfsr9670au22zvq082jaej8ve4", "juno10c3slrqx3369mfsr9670au22zvq082jaej8ve4",
            {}, "auto", undefined, []);
    });

    xit("scan mint file and send 111 stars", async () => {
        let gas = GasPrice.fromString("0.025ujunox");
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'juno' });
        let client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, wallet, {gasPrice:gas});
        client.searchTx({sentFromOrTo:"stars10gq4a80n7upxagecwpsn2uzkzqvcy9xg3d2sks"}, {minHeight:1000, maxHeight:100002});
    }).timeout(100000000);


});