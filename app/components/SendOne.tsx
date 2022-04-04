import * as anchor from '@project-serum/anchor';
import { useWallet } from '@solana/wallet-adapter-react';
import {  clusterApiUrl, Connection, PublicKey } from '@solana/web3.js';
import React, { FC } from 'react';
import  idl from '../target/idl/agreement.json';
import { Agreement } from '../target/types/agreement';


export const SendOne: FC = () => {
    let amount_total = new anchor.BN(1* (1000000000));
    let amount_gurantee = new anchor.BN(0.5* (1000000000));

    console.log()

    const programID = new PublicKey(idl.metadata.address);


    const network = clusterApiUrl('devnet');
    const connection = new Connection(network, "processed");
    const wallet = useWallet();

    
    const provider = new anchor.Provider(connection, wallet, "processed");


    const program = new anchor.Program<Agreement>(idl, programID, provider);

    async function initialize() {
        const buffer = anchor.web3.Keypair.generate();

        if(wallet.publicKey !=null){
            const [contractPDA, _ ] = await PublicKey
            .findProgramAddress(
              [
                anchor.utils.bytes.utf8.encode("contract_acc"),
                wallet.publicKey.toBuffer(),
                buffer.publicKey.toBuffer(),
              ],
              programID
            );
            
            const tx = await program.rpc.initialize(buffer.publicKey, amount_gurantee, amount_total,  {
            accounts:{
                contract: contractPDA,
                contractor: wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                }
            });

            console.log("tx: ",tx);
            console.log("buffer: ", buffer);

            const tx2 = await program.rpc.cancel({
                accounts:{
                    contract: contractPDA,
                    destination:wallet.publicKey,
                }
            })

            console.log("tx2: ",tx2);
        }
    }

    
    return (
        <>
            <button onClick={initialize}>
                init!
            </button>
        </>

    );
};