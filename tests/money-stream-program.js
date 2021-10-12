const assert = require('assert');
const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

describe('money-stream-program', () => {

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);  // Configure the client to use the config cluster.

  // Program for the tests.
  const program = anchor.workspace.MoneyStreamProgram;

  it('Creates a session', async () => {

    const benefiter = anchor.web3.Keypair.generate()
    await program.rpc.create(provider.wallet.publicKey, benefiter, new anchor.BN(100),  new anchor.BN(10),{
      accounts: {
        sessionAccount: SessionAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [sessionAccount],
    })

    let sessionAccount = await program.account.sessionAccount.fetch(sessionAccount.publicKey)

    assert.ok(sessionAccount.authority.equals(provider.wallet.publicKey))
    assert.ok(sessionAccount.amount.toNumber() === 10)
    
  })
/*
  it('Updates a counter', async () => {
    await program.rpc.increment({
      accounts: {
        counter: counter.publicKey,
        authority: provider.wallet.publicKey,
      },
    })

    const counterAccount = await program.account.counter.fetch(counter.publicKey)

    assert.ok(counterAccount.authority.equals(provider.wallet.publicKey))
    assert.ok(counterAccount.count.toNumber() == 1)
  })
  */
})
