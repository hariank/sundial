import * as anchor from "@project-serum/anchor";
import { strict as assert } from "assert";

describe("sol_cron", () => {
  // Read the provider from the configured environmnet.
  const provider = anchor.Provider.env();

  // Configure the client to use the provider.
  anchor.setProvider(provider);

  const solCron = anchor.workspace.SolCron;
  const exampleTask = anchor.workspace.ExampleTask;

  const counter = anchor.web3.Keypair.generate();
  const counterAuthority = anchor.web3.Keypair.generate();
  const user = provider.wallet.publicKey;

  it("all", async () => {
    await exampleTask.rpc.create({
      accounts: {
        counter: counter,
        user: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [counterAuthority],
    });

    await solCron.rpc.run_task({
      accounts: {
        task_program: exampleTask.programId,
      },
    });

    let counterAccount = await exampleTask.account.counter.data.fetch(
      counterAuthority.publicKey
    );
    assert.ok(counterAccount.authority.equals(user))
    assert.ok(counterAccount.count.toNumber() == 1)
  });
});
