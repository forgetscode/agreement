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

  const bad = anchor.web3.Keypair.generate();

  let amount_total = new anchor.BN(20* (1000000000));
  let amount_gurantee = new anchor.BN(10* (1000000000));

  it('Is initialized!', async () => {
      const [contractPDA, _] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("contract-acc"),
          contractor.publicKey.toBuffer()
        ],
        program.programId
      );
    let balancebefore = await program.provider.connection.getBalance(contractPDA);
    console.log(balancebefore* (10**-9));

    // Add your test here.
    const tx = await program.rpc.initialize(amount_gurantee, amount_total,{
      accounts: {
        contract: contractPDA,
        contractor: contractor.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }
    });
    console.log("Your transaction signature", tx);

    let _myAccountDataNew = await program.account.contract.fetch(contractPDA);
    console.log(_myAccountDataNew);
    let balanceafter = await program.provider.connection.getBalance(contractPDA);
    console.log(balanceafter* (10**-9));

    const tx2 = await program.rpc.updateAmount(amount_gurantee, amount_total,{
      accounts:{
        contract: contractPDA,
        contractor: contractor.publicKey,
      }
    });

    const tx3 = await program.rpc.cancel({
      accounts:{
        contract: contractPDA,
        destination: contractor.publicKey,
      }
    });
    let balanceafterclose = await program.provider.connection.getBalance(contractPDA);
    console.log(balanceafterclose* (10**-9));

  });
});
