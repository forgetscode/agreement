import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Agreement } from '../target/types/agreement';

describe('agreement', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Agreement as Program<Agreement>;

  const contractor = program.provider.wallet;

  const contract = anchor.web3.Keypair.generate();


  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize(16, 15,{
      accounts: {
        contract: contract.publicKey,
        contractor: contractor.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers:[contract],
    });
    console.log("Your transaction signature", tx);

    let _myAccountDataNew = await program.account.contract.fetch(contract.publicKey);
    console.log(_myAccountDataNew.contractor.toBase58());
    console.log(_myAccountDataNew.contractee);
  });

});
