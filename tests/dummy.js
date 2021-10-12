
const anchor = require('@project-serum/anchor');

describe('money-stream-program', () => {

    const provider = anchor.Provider.env();
    anchor.setProvider(provider);  // Configure the client to use the config cluster.
  
  
    it('Is initialized!', async () => {
        // Add your test here.
        const program = anchor.workspace.MoneyStreamProgram;
        const tx = await program.rpc.dummy();
        console.log("Your transaction signature", tx);
    });
});