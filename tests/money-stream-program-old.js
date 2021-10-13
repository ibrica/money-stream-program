const assert = require('assert');
const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;
/*
describe('money-stream-program', () => {

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);  // Configure the client to use the config cluster.

  // Program for the tests.
  const program = anchor.workspace.MoneyStreamProgram;
  const sessionAccount = anchor.web3.Keypair.generate();
  const benefiterAccount = anchor.web3.Keypair.generate();

  it('Creates a session', async () => {
    // https://www.notion.so/Debugging-Custom-Anchor-Errors-b8540dd418c44a4e939ab17c56a3fd3b

    await program.rpc.create(provider.wallet.publicKey, benefiterAccount.publicKey, new anchor.BN(100),  new anchor.BN(10),{
      accounts: {
        sessionAccount: sessionAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [sessionAccount],
    });

    let account = await program.account.sessionAccount.fetch(sessionAccount.publicKey)

    assert.ok(account.authority.equals(provider.wallet.publicKey))
    assert.ok(account.limit.toNumber() === 100)
    assert.ok(account.amount.toNumber() === 10)
    
  }).timeout(5000); // node timeouting after 1s???

  it('Updates amount for a step', async () => {
    await program.rpc.tick({
      accounts: {
        sessionAccount: sessionAccount.publicKey,
        authority: provider.wallet.publicKey,
      },
    })

    let account = await program.account.sessionAccount.fetch(sessionAccount.publicKey)

    assert.ok(account.authority.equals(provider.wallet.publicKey))
    assert.ok(account.amount.toNumber() === 20)
  }).timeout(5000);

  it('Checks a balance for benefiter', async () => {
    await program.rpc.balance({
      accounts: {
        sessionAccount: sessionAccount.publicKey,
        authority: benefiterAccount.publicKey,
      },
    })

    let account = await program.account.sessionAccount.fetch(sessionAccount.publicKey)

    assert.ok(account.amount.toNumber() === 20)
  }).timeout(5000);



  it('Withdraws a money as benefiter', async () => {
    await program.rpc.withdraw({
      accounts: {
        sessionAccount: sessionAccount.publicKey,
      }
    })

    let account = await program.account.sessionAccount.fetch(sessionAccount.publicKey)

    assert.ok(account.amount.toNumber() === 20)
  }).timeout(5000);
  
})
*/
