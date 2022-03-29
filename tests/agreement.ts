import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { Agreement } from '../target/types/agreement';
import { expect } from 'chai';

describe('agreement', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Agreement as Program<Agreement>;

  const contractor = program.provider.wallet;



  it('Is initialized!', async () => {
      const [contractPDA, _] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("contract-acc"),
          contractor.publicKey.toBuffer()
        ],
        program.programId
      );

    // Add your test here.
    const tx = await program.rpc.initialize(15, 15,{
      accounts: {
        contract: contractPDA,
        contractor: contractor.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }
    });
    console.log("Your transaction signature", tx);

    let _myAccountDataNew = await program.account.contract.fetch(contractPDA);
    console.log(_myAccountDataNew);
    console.log(_myAccountDataNew.contractee);

  });

});
