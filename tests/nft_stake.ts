const anchor = require("@project-serum/anchor");
import * as spl from '@solana/spl-token';
import { Program } from '@project-serum/anchor';
import { NftStake } from '../target/types/nft_stake';

// Setup
// make Mint
// make associated token account
// mintTo


// workflow
// create new account for contract (player)
// initialize (progam.rpc.initialize)
  // creates the accounts for us! vs. vanilla rust 




// initialize

// try to stake with different
// try to stake twice

// try to release with different
// try to release with manager before staked¡

describe('nft_stake', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.NftStake as Program<NftStake>;

  let nftMint = spl.Token;

  const staker = anchor.web3.Keypair.generate();

  const manager = anchor.web3.Keypair.generate();

  it('Is initialized!', async () => {
    // get PDA and bump
    const stakeProgram = anchor.web3.Keypair.generate();

    const [stakeProgramToken, stakeProgramTokenBump] = await anchor.web3.PublicKey.findProgramAddress(
      [stakeProgram.publicKey.toBuffer()],
      program.programId
    );

    // Initialize transaction
    const tx = await program.rpc.initialize(
      staker.publicKey,
      manager.publicKey,
      stakeProgramTokenBump,
      {
        accounts: {
          myAccount: stakeProgram.publicKey,
          user: staker.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      signers: [staker]
      }
    );
    console.log("Your transaction signature", tx);
  });

});
