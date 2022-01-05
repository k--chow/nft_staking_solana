import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { NftStake } from '../target/types/nft_stake';

// initialize

// try to stake with different
// try to stake twice

// try to release with different
// try to release with manager before staked¡

describe('nft_stake', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.NftStake as Program<NftStake>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
