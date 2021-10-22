import * as anchor from "@project-serum/anchor";
import { strict as assert } from "assert";

describe("sol_cron", () => {
  // Read the provider from the configured environmnet.
  const provider = anchor.Provider.env();

  // Configure the client to use the provider.
  anchor.setProvider(provider);

  const solCron = anchor.workspace.Solcron;
  const exampleTask = anchor.workspace.ExampleTask;

  const counter = anchor.web3.Keypair.generate();
  const counterAuthority = anchor.web3.Keypair.generate();
  const user = provider.wallet.publicKey;
  let tx;

  it("all", async () => {
    tx = await exampleTask.rpc.create(counterAuthority.publicKey, {
      accounts: {
        counter: counter.publicKey,
        user: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [counter],
    });
    console.log("Create tx: ", tx);

    tx = await solCron.rpc.runTask({
      accounts: {
        taskProgram: exampleTask.programId,
      },
      remainingAccounts: [
        { pubkey: counter.publicKey, isWritable: true, isSigner: false },
      ],
    });
    console.log("Run task tx: ", tx);


    const counterAccount = await exampleTask.account.counter.fetch(counter.publicKey)
    assert.ok(counterAccount.authority.equals(counterAuthority.publicKey))
    assert.ok(counterAccount.count.toNumber() == 1)
  });
});
