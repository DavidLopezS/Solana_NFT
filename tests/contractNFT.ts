import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { ContractNft } from '../target/types/contract_nft';
import * as splToken from '@solana/spl-token';

const {PublicKey, Keypair, Transaction, SystemProgram, SYSVAR_RENT_PUBKEY} = anchor.web3;
const {TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, MINT_SIZE, createInitializeMintInstruction,getAssociatedTokenAddress} = splToken;

describe('contractNFT', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ContractNft as Program<ContractNft>;

  it("Is Initialized!",async () => {
    
    console.log("INF");
    const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
      'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'
    );
    
    const lamports = await provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);
    const mintKeys = Keypair.generate();
    const mint_tx = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: provider.wallet.publicKey,
        newAccountPubkey: mintKeys.publicKey,
        space: MINT_SIZE,
        lamports,
        programId: TOKEN_PROGRAM_ID,
      }), createInitializeMintInstruction(
        mintKeys.publicKey,//mint pubkey
        0,//decimals
        provider.wallet.publicKey,//mint authority
        provider.wallet.publicKey,//freeze authority
      )
    );
    
    //console.log("Create account fee: ", (await mint_tx.getEstimatedFee(provider.connection)).toString());

    const res = await provider.send(mint_tx, [mintKeys]);

    console.log(await provider.connection.getParsedAccountInfo(mintKeys.publicKey));
  
    console.log("ACCOUNT", res);
    console.log("MINTKEY", mintKeys.publicKey.toString());
    console.log("USER", provider.wallet.publicKey.toString());
    
    const associatedToken = await getAssociatedTokenAddress(
      mintKeys.publicKey,
      provider.wallet.publicKey,
    );
    
    console.log("ASSOCIATED TOKEN", associatedToken.toString());

    const [metadataKey, ] = await PublicKey.findProgramAddress(
     [
      Buffer.from('metadata'),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mintKeys.publicKey.toBuffer(),
     ],
     TOKEN_METADATA_PROGRAM_ID 
    );
    
    console.log("METADATA", metadataKey.toString());

    const[masterKey,] = await PublicKey.findProgramAddress(
      [
        Buffer.from('metadata'),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeys.publicKey.toBuffer(),
        Buffer.from('edition'),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    console.log("MA",masterKey.toString());
    
    const tx = await program.rpc.createMetadataToken
    (
      "New NFT",
      "NEW",
      "Pup",//https://arweave.net/o_WP4ouPlRxBSwIg0xs3g4I9tHVsGxtadxCeE6Adr4Q
      1000,
      false,
      {
        accounts: {
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
          payer: provider.wallet.publicKey,
          mplProgram: TOKEN_METADATA_PROGRAM_ID,
          metadata: metadataKey,
          mint: mintKeys.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          signerTokenAccount: associatedToken,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          masterEdition: masterKey,
        }
      }
    );
    
    console.log("TX", tx);

  });
  
});
