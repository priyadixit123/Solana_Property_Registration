import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
// Nimport * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaPropertyRegistry } from "../target/types/solana_property_registry";
import { assert } from "chai";
import type { SolanaPropertyRegistry } from "../target/types/solana_property_registry";

  

describe("solana_property_registry", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaPropertyRegistry as anchor.Program<SolanaPropertyRegistry>;
  

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaPropertyRegistry as Program <SolanaPropertyRegistry>;
  let adminAccount : anchor.web3.keypair;
  let propertyAccount: anchor.web3.keypair;

  const owner = provider.wallet;
  const nominee1 = anchor.web3.keypair.generate();
  const nominee2 = anchor.web3.keypair.generate();

  before(async () => {
    const sig = await provider.connection.requestAirdrop(owner.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(sig);
  });

  it("initialize admin", async () => {

    adminAccount = anchor.web3.Keypair.generate();

    await program.methods
    .initializeAdmin()
    .accounts ({
      admin:adminAccount.pubkey,
      initializer:  owner.publicKey,
      systemProgram:anchor.web3.SystemProgram.programId,
    })

    .signers([adminAccount])
    .rpc();   
     

    
    const admin = await program.account.admin.fetch(
      adminAccount.publicKey
    );     
    
    assert.ok(admin.owner.equals(owner.publicKey));
  });


   it("Registers a property", async () => {

    propertyAccount = anchor.web3.Keypair.generate();

    await program.methods
    .registerProperty("PROP123", "New Delhi", new anchor.BN(2000))
    .accounts ({
      property:propertyAccount.pubkey,
      owner:  owner.publicKey,
      systemProgram:anchor.web3.SystemProgram.programId,
    })

    .signers([propertyAccount])
    .rpc();   
     

    
    const property = await program.account.property.fetch(
      propertyAccount.publicKey
    );  

    assert.equal(property.propertyId,"PROP123" );  
    assert.equal(property.location,"New Delhi" );  
    assert.equal(property. area.toNumber(),2000 );   
    
    assert.ok(property.owner.equals(owner.publicKey));
    assert.deepEqual(property.history,[owner.publicKey] );  
  });

  it("Adds nominees", async () =>{

    await program.methods
    .addNominee(nominee1.publicKey,60)
    .accounts ({
      property: propertyAccount.publickey,
      owner:owner.publicKey,
    })
    .rpc();

    await program.methods
    .addNominee(nominee2.publicKey,40)
    .accounts ({
      property:propertyAccount.publicKey,
      owner:owner.publicKey,

    })
    .rpc();

    const property = await program.account.property.fetch(
      propertyAccount.publicKey
    );
    assert.lengthOf(property.nominees,2);
    assert.ok(property.nominees[0].nominee.equals(nominee1.publicKey));
    assert.equal(property.nominees[0].share,60);
    assert.ok(property.nominees[1].nominee.equals(nominee2.publickey));
    assert.equal(property.nominees[1].share,40);
  });

  it("Transfer property",async()=>{
    const newOwner = anchor.web3.Keypair.generate();

    await program.methods
    .transferProperty(newOwner.publicKey)
    .accounts({
      owner:owner.publicKey,
    })
    .rpc(); 

    const property = await program.account.property.fetch(
      propertyAccount.publicKey
    );

    assert.ok(property.owner.equals(newOwner.publicKey));
    assert.deepEqual(property.history.slice(-1)[0], newOwner.publicKey);

    
    });

    it("Freezes property (Admin)", async()=>{
      await program.methods
      .updateDisputeStatus(true)
      .accounts({
        property:propertyAccount.publicKey,
        admin:adminAccount.publicKey,
        authority:owner.publicKey,
      })
      .rpc();

      const property = await program.account.property.fetch(
        propertyAccount.publicKey
      );
      assert.isTrue(property.freezeStatus);
    });

    it("Nomniee Claims property share", async()=>{
      await program.methods
      .updateDisputeStatus(false)
      .accounts({
        property:propertyAccount.publicKey,
        admin:adminAccount.publicKey,
        authority:owner.publicKey,
      })
      .rpc();

      await program.methods
      .claimProperty()
      .accounts({
        property :propertyAccount.publicKey,
        claimant: nominee1.publicKey,
      })
      .signers([nominee1])
      .rpc();

      const property = await program.account.property.fetch(
        propertyAccount.publicKey
      );
      const nominee = property.nominees.find(n=>n.nominee.equals(nominee1.publicKey));
      assert.isTrue(nominee.claimed);
    });




});
