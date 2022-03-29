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

  const contractee = anchor.web3.Keypair.generate();

  let amount_total = new anchor.BN(20* (1000000000));
  let amount_gurantee = new anchor.BN(10* (1000000000));

  it('Is initialized!', async () => {

    const fromAirdropSignature = await program.provider.connection.requestAirdrop(
          contractee.publicKey,
          anchor.web3.LAMPORTS_PER_SOL,
      )
    await program.provider.connection.confirmTransaction(fromAirdropSignature);

      const [contractPDA, _ ] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("contract_acc"),
          contractor.publicKey.toBuffer()
        ],
        program.programId
      );
    let balancebefore = await program.provider.connection.getBalance(contractee.publicKey);
    console.log("contractee balance: ",balancebefore* (10**-9));

    console.log(contractPDA.toBase58());
    // Add your test here.
    const tx = await program.rpc.initialize(amount_gurantee, amount_total,{
      accounts: {
        contract: contractPDA,
        contractor: contractor.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }
    });
    console.log("Your transaction signature", tx);

    const accounts = await program.provider.connection.getProgramAccounts(program.programId);
    console.log("program accounts after init:", accounts[0].account.owner.toBase58());

    let balanceafter = await program.provider.connection.getBalance(contractPDA);
    console.log(balanceafter* (10**-9));

    /*
    const tx2 = await program.rpc.updateAmount(amount_gurantee, amount_total,{
      accounts:{
        contract: contractPDA,
        contractor: contractor.publicKey,
      }
    });
    console.log("Your transaction signature", tx2);
    */

    const tx3 = await program.rpc.open({
      accounts: {
        contract: contractPDA,
        contractor: contractor.publicKey,
      }
    });

    
    const tx4 = await program.rpc.openTo( contractee.publicKey , {
      accounts: {
        contract: contractPDA,
        contractor: contractor.publicKey,
      }
    });
    

    let _myAccountDataNew = await program.account.contract.fetch(contractPDA);
    console.log("account data after opento: ", _myAccountDataNew);

    const tx5 = await program.rpc.accept({
      accounts: {
        contract: contractPDA,
        contractee: contractee.publicKey,
      },
      signers:[contractee]
    });

    

    const tx6 = await program.rpc.dispute({
      accounts: {
        contract: contractPDA,
        contractee: contractee.publicKey,
        destination: contractor.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    let balanceafter2 = await program.provider.connection.getBalance(contractee.publicKey);
    console.log("contractee: ",balanceafter2* (10**-9));

    
    let balanceafter3 = await program.provider.connection.getBalance(contractor.publicKey);
    console.log("contractor: ",balanceafter3* (10**-9));

  });
});
