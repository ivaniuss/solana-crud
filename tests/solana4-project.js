const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { rpc } = require("@project-serum/anchor/dist/cjs/utils");
const { SystemProgram } = anchor.web3;


describe("solana4-project", () => {

  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  // myaccount for the tests.
  const myAccount = anchor.web3.Keypair.generate();

  // Program for the tests.
  const program = anchor.workspace.Solana4Project;


  it("Initialize the program", async () => {

    const init = await program.methods
    .initialize()
    .accounts({
      myAccount: myAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([myAccount])
    .rpc()

    // creating new user
    await program.methods
    .createUser("ivan", new anchor.BN(27))
    .accounts({
      myAccount: myAccount.publicKey,
    }).rpc()

    let userAccount = await program.account.myAccount.fetch(myAccount.publicKey);
    console.log('my account', userAccount);

    // update user
    await program.methods
    .updateUser(new anchor.BN(0), "juan", new anchor.BN(20))
    .accounts({
      myAccount: myAccount.publicKey,
    }).rpc()

    userAccount = await program.account.myAccount.fetch(myAccount.publicKey);
    console.log('my account', userAccount);


    // delete user
    await program.methods
    .deleteUser(new anchor.BN(0))
    .accounts({
      myAccount: myAccount.publicKey,
    }).rpc()

    userAccount = await program.account.myAccount.fetch(myAccount.publicKey);
    console.log('my account', userAccount);

  });

});
