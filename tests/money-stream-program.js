const anchor = require("@project-serum/anchor");
const { PublicKey, Transaction, SystemProgram } = anchor.web3;
const { TOKEN_PROGRAM_ID, Token } = require("@solana/spl-token");
const assert = require("assert");
const provider = anchor.Provider.env();

describe("money-stream-program", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MoneyStreamProgram;

  let mint = null;
  let initializerTokenAccount = null;
  let takerTokenAccount = null;

  let vault_account_pda = null;
  let vault_account_bump = null;
  let vault_authority_pda = null;
 
  const streamAccount = anchor.web3.Keypair.generate();
  const payer = anchor.web3.Keypair.generate();
  const mintAuthority = anchor.web3.Keypair.generate();
  const initializerMainAccount = anchor.web3.Keypair.generate();
  const takerMainAccount = anchor.web3.Keypair.generate();
  let mintAmount = 100; // mint new tokens

  it("Initialise stream state", async () => {
    // Airdropping tokens to a payer.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, 5000000000), //Devnet limit
      "confirmed"
    );

    // Fund Main Accounts
    await provider.send(
      (() => {
        const tx = new Transaction();
        tx.add(
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: initializerMainAccount.publicKey,
            lamports: 1000000000,
          }),
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: takerMainAccount.publicKey,
            lamports: 1000000000,
          })
        );
        return tx;
      })(),
      [payer]
    );

    mint = await Token.createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

 
    initializerTokenAccount = await mint.createAccount(initializerMainAccount.publicKey);
    takerTokenAccount = await mint.createAccount(takerMainAccount.publicKey);


    await mint.mintTo(
      initializerTokenAccount,
      mintAuthority.publicKey,
      [mintAuthority],
      mintAmount
    );

    let _initializerTokenAccount = await mint.getAccountInfo(initializerTokenAccount);
    let _takerTokenAccount = await mint.getAccountInfo(takerTokenAccount);

    assert.ok(_initializerTokenAccount.amount.toNumber() == mintAmount);
    assert.ok(_takerTokenAccount.amount.toNumber() == 0);
  }).timeout(20000);

  it("Initialize stream", async () => {
    const [_vault_account_pda, _vault_account_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("token-seed"))],
      program.programId
    );
    vault_account_pda = _vault_account_pda;
    vault_account_bump = _vault_account_bump;

    const [_vault_authority_pda, _vault_authority_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("streaming"))],
      program.programId
    );
    vault_authority_pda = _vault_authority_pda;

    await program.rpc.initializeStream(
      vault_account_bump,
      new anchor.BN(60),
      new anchor.BN(1),
      new anchor.BN(10),
      {
        accounts: {
          initializer: initializerMainAccount.publicKey,
          vaultAccount: vault_account_pda,
          mint: mint.publicKey,
          initializerTokenAccount: initializerTokenAccount,
          streamAccount: streamAccount.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        instructions: [
          await program.account.streamAccount.createInstruction(streamAccount),
        ],
        signers: [streamAccount, initializerMainAccount],
      }
    );

    let _vault = await mint.getAccountInfo(vault_account_pda);

    let _streamAccount = await program.account.streamAccount.fetch(
      streamAccount.publicKey
    );

    // Check that the new owner is the PDA.
    assert.ok(_vault.owner.equals(vault_authority_pda));

    // Check that the values in the stream account match what we expect.
    assert.ok(_streamAccount.initializerKey.equals(initializerMainAccount.publicKey));
    assert.ok(_streamAccount.limit.toNumber() == 60);
    assert.ok(_streamAccount.step.toNumber() == 1);
    assert.ok(_streamAccount.rate.toNumber() == 10);

    assert.ok(
      _streamAccount.initializerTokenAccount.equals(initializerTokenAccount)
    );
    
   assert(true)

  }).timeout(5000);

  it("Time tick", async () => {
    await program.rpc.tick({
      accounts: {
        initializer: initializerMainAccount.publicKey,
        streamAccount: streamAccount.publicKey,
      },
      signers: [initializerMainAccount]
    });

    let _streamAccount = await program.account.streamAccount.fetch(
      streamAccount.publicKey
    );

    // Check that the values in the stream account match after tick
    assert.ok(_streamAccount.initializerKey.equals(initializerMainAccount.publicKey));
    assert.ok(_streamAccount.limit.toNumber() == 60);
    assert.ok(_streamAccount.step.toNumber() == 2);
    assert.ok(_streamAccount.rate.toNumber() == 10);
  }).timeout(5000);

  it("Check balance", async () => {
    await program.rpc.balance({
      accounts: {
        taker: takerMainAccount.publicKey,
        initializer: initializerMainAccount.publicKey,
        takerTokenAccount: takerTokenAccount,
        streamAccount: streamAccount.publicKey,
      },
      signers: [takerMainAccount]
    });

    let _streamAccount = await program.account.streamAccount.fetch(
      streamAccount.publicKey
    );

    // Check that the values in the stream account match what taker expects.
    assert.ok(_streamAccount.initializerKey.equals(initializerMainAccount.publicKey));
    assert.ok(_streamAccount.limit.toNumber() == 60);
    assert.ok(_streamAccount.step.toNumber() == 2);
    assert.ok(_streamAccount.rate.toNumber() == 10);
  }).timeout(5000);


  it("Withdraw stream", async () => {
    await program.rpc.withdraw({
      accounts: {
        taker: takerMainAccount.publicKey,
        initializer: initializerMainAccount.publicKey,
        takerTokenAccount: takerTokenAccount,
        initializerTokenAccount: initializerTokenAccount,
        streamAccount: streamAccount.publicKey,
        vaultAccount: vault_account_pda,
        vaultAuthority: vault_authority_pda,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [takerMainAccount]
    });

    let _takerTokenAccount = await mint.getAccountInfo(takerTokenAccount);
    let _initializerTokenAccount = await mint.getAccountInfo(initializerTokenAccount);

    // TODO: Assert if the PDA token account is closed

    assert.ok(_takerTokenAccount.amount.toNumber() == 20);
    assert.ok(_initializerTokenAccount.amount.toNumber() == 80);

  }).timeout(5000);


  it("Initialize stream and cancel stream", async () => {
    // Init

    await program.rpc.initializeStream(
      vault_account_bump,
      new anchor.BN(60),
      new anchor.BN(1),
      new anchor.BN(10),
      {
        accounts: {
          initializer: initializerMainAccount.publicKey,
          vaultAccount: vault_account_pda,
          mint: mint.publicKey,
          initializerTokenAccount: initializerTokenAccount,
          streamAccount: streamAccount.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        instructions: [
          await program.account.streamAccount.createInstruction(streamAccount),
        ],
        signers: [streamAccount, initializerMainAccount],
      }
    );
   
    // Cancel the stream.
    await program.rpc.cancelStream({
      accounts: {
        initializer: initializerMainAccount.publicKey,
        takerTokenAccount: takerTokenAccount,
        initializerTokenAccount: initializerTokenAccount,
        vaultAccount: vault_account_pda,
        vaultAuthority: vault_authority_pda,
        streamAccount: streamAccount.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [initializerMainAccount]
    });

    // TODO: Assert if the PDA token account is closed

    // Check the final owner should be the provider public key.
    _initializerTokenAccount = await mint.getAccountInfo(initializerTokenAccount);
    _takerTokenAccount = await mint.getAccountInfo(takerTokenAccount);
    assert.ok(_initializerTokenAccount.owner.equals(initializerMainAccount.publicKey));

    // Check cancel funds.
    assert.ok(_takerTokenAccount.amount.toNumber() == 30);
    assert.ok(_initializerTokenAccount.amount.toNumber() == 70);
  }).timeout(10000);
});
