import * as anchor from "@coral-xyz/anchor";
import { BorshCoder, EventParser, Program } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { config } from 'dotenv';
import { expect } from "chai";
import BN from "bn.js";
import { ZeroaiProgram } from "../target/types/zeroai_program";


config();

// Price Feed IDs
// https://pyth.network/developers/price-feed-ids#solana-devnet

const BTC_ToUSD = "HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J";
const ETH_ToUSD = "EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw";
const SOL_ToUSD = "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix";
const BNB_ToUSD = "GwzBgrXb4PG59zjce24SF2b9JXbLEjJJTBkmytuEZj1b";
const DOGE_ToUSD = "4L6YhY8VvUgmqG5MvJkUJATtzB2rFqdrJwQCmFLv4Jzy";
const JUP_ToUSD = "Gh9hscza9YaCzr84tNV1NZQfpqoL3csYnWDygDkQmBU2";

// Configure the client to use the env cluster.
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.ZeroaiProgram as Program<ZeroaiProgram>;

// Configure the walltes
const userWallet = anchor.workspace.ZeroaiProgram.provider.wallet;

/**
 * In a development or testnet environment, it's recommended to use a persistent keypair
 * to avoid repeated account initialization and rent-exemption delays.
 *
 * If the VAULT_KEY environment variable is set, it will be used to derive the keypair.
 * Otherwise, a new keypair will be generated and logged to the console.
 *
 * For production deployments, it's crucial to use a secure keypair management solution
 * and never store private keys in plaintext or commit them to version control.
 */
const secretKey = new Uint8Array(JSON.parse(process.env.VAULT_KEY || ''));
const vault = Keypair.fromSecretKey(secretKey);

/* Uncomment the line below to generate a new keypair for development/testing */
// const vault = anchor.web3.Keypair.generate();

/**
 * Retrieves the Program Derived Address (PDA) for the "games" account.
 * The PDA is a unique address derived from the program's ID, a seed phrase, and an optional signer.
 * It is used to deterministically locate and reference accounts owned by the program.
 *
 * @param {Buffer} seed - The seed phrase used to derive the PDA, consisting of the string "games" and the user's wallet public key.
 * @param {web3.PublicKey} programId - The public key of the Solana program that owns the "games" account.
 * @returns {[web3.PublicKey, number]} An array containing the derived PDA public key and its corresponding bump seed.
 */
const [gamesPDA] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("games"), userWallet.publicKey.toBuffer()],
  program.programId
);

/**
 * Asynchronous utility function that introduces a deliberate delay.
 * Primarily employed by the EventParser to facilitate the retrieval of transaction signatures.
 *
 * @param ms - The desired delay duration in milliseconds.
 * @returns A Promise that resolves after the specified delay.
 */
function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

describe("initVault", () => {
  // Initialize the Vault account
  it("Vault account initialized!", async () => {
    try {
      const tx = await program.methods
        .initvault({
          admin: userWallet.publicKey,
          btcFeed: new anchor.web3.PublicKey(BTC_ToUSD),
          ethFeed: new anchor.web3.PublicKey(ETH_ToUSD),
          solFeed: new anchor.web3.PublicKey(SOL_ToUSD),
          bnbFeed: new anchor.web3.PublicKey(BNB_ToUSD),
          dogeFeed: new anchor.web3.PublicKey(DOGE_ToUSD),
          jupFeed: new anchor.web3.PublicKey(JUP_ToUSD),
        })
        .accounts({
          vault: vault.publicKey,
          payer: userWallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([vault])
        .rpc();
      const account = await program.account.vault.fetch(vault.publicKey);
      expect(ETH_ToUSD === account.ethFeed.toString());
      console.log("Admin: ", account.admin.toString());
    } catch (err) {
      expect(err);
      console.log(err);
    }
  });
});
describe("initGames", () => {
  // Initialize the Games account
  it("Games initialized!", async () => {
    try {
      const tx = await program.methods
        .initgames()
        .accounts({
          vault: vault.publicKey,
          games: gamesPDA,
          admin: userWallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      const account = await program.account.instantWinGame.fetch(gamesPDA);
      expect(account.count.toNumber() === 0);
      console.log(account.symbol.toString());
    } catch (err) {
      expect(err);
      console.log(err);
    }
  });
});
describe("addGames", () => {
  // Add Game to the Games account
  it("Add Game!", async () => {
    let symbol = ["BTC", "ETH", "SOL", "BNB", "DOGE", "JUP"];
    let price = [
      new BN(645391234),
      new BN(43218765),
      new BN(1901827),
      new BN(6201780),
      new BN(1780),
      new BN(9230),
    ];
    try {
      const tx = await program.methods
        .addgame(symbol, price)
        .accounts({
          vault: vault.publicKey,
          game: gamesPDA,
          pythBtc: new anchor.web3.PublicKey(BTC_ToUSD),
          pythEth: new anchor.web3.PublicKey(ETH_ToUSD),
          pythSol: new anchor.web3.PublicKey(SOL_ToUSD),
          pythBnb: new anchor.web3.PublicKey(BNB_ToUSD),
          pythDoge: new anchor.web3.PublicKey(DOGE_ToUSD),
          pythJup: new anchor.web3.PublicKey(JUP_ToUSD),
          admin: userWallet.publicKey,
        })
        .rpc();
      const account = await program.account.instantWinGame.fetch(gamesPDA);
      expect(account.targetprice[1].toNumber() === 2000);
      console.log("BTC Price: ", account.closeprice[0].toString());
    } catch (err) {
      expect(err);
      console.log(err);
    }
  });
});
describe("eventParser", () => {
  // Get Signature and Parse Event
  it("Last Signatures!", async () => {
    await sleep(1000);
    const signatures = await anchor
      .getProvider()
      .connection.getConfirmedSignaturesForAddress2(
        vault.publicKey,
        { limit: 1 },
        "confirmed"
      );
    console.log("Signature: ", signatures[0].signature);
    const transaction = await anchor
      .getProvider()
      .connection.getParsedTransaction(signatures[0].signature, {
        commitment: "confirmed",
      });
    const eventParser = new EventParser(
      program.programId,
      new BorshCoder(program.idl)
    );
    const events = eventParser.parseLogs(transaction.meta.logMessages);
    for (let event of events) {
      for (let i = 0; i < (event.data.closeprice as Array<BN>).length; i++) {
        console.log(
          event.data.symbol[i],
          Number(event.data.closeprice[i]) / 10000
        );
      }
    }
  });
});
describe("closeGames", () => {
  // Close the Games account
  it("Close Game!", async () => {
    try {
      const tx = await program.methods
        .closegame()
        .accounts({
          vault: vault.publicKey,
          game: gamesPDA,
          admin: userWallet.publicKey,
        })
        .rpc();
    } catch (err) {
      expect(err);
      console.log(err);
    }
  });
});