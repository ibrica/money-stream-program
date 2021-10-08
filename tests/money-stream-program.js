const anchor = require('@project-serum/anchor');

describe('money-stream-program', () => {

  // Configure the client to use the config cluster.
  anchor.setProvider(anchor.Provider.env());
  anchor

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.MoneyStreamProgram;
    const tx = await program.rpc.initialize();
    console.log("Your transaction signature", tx);
  });
});
