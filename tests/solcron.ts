import * as anchor from "@project-serum/anchor";
import { strict as assert } from "assert";

describe("sol_cron", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const solcronProgram = anchor.workspace.Solcron;
  const counterProgram = anchor.workspace.ExampleTask;

  const counter = anchor.web3.Keypair.generate();
  const counterAuthority = anchor.web3.Keypair.generate();
  const user = provider.wallet.payer;
  const userKey = provider.wallet.publicKey;

  let specAccountKey: anchor.web3.PublicKey, specAccountBump: number;

  it("setup", async () => {
    await counterProgram.rpc.create(counterAuthority.publicKey, {
      accounts: {
        counter: counter.publicKey,
        user: userKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [counter],
    });

    const counterAccount = await counterProgram.account.counter.fetch(
      counter.publicKey
    );
    assert.ok(counterAccount.authority.equals(counterAuthority.publicKey));
    assert.ok(counterAccount.count.toNumber() == 0);

    [specAccountKey, specAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [counterProgram.programId.toBuffer()],
        solcronProgram.programId
      );
    console.log("Spec account: ", specAccountKey, specAccountBump);
  });

  it("register", async () => {
    const startTs = new anchor.BN(Date.now() / 1000 - 5);
    const intervalTs = new anchor.BN(startTs.toNumber() + 10);

    await solcronProgram.rpc.registerTask(
      new anchor.BN(specAccountBump),
      counterProgram.programId,
      startTs,
      intervalTs,
      {
        accounts: {
          taskProgram: counterProgram.programId,
          taskSpecification: specAccountKey,
          registrar: userKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [user],
      }
    );

    const specAccount = await solcronProgram.account.taskSpecification.fetch(
      specAccountKey
    );
    console.log("Spec account data:\n", specAccount);
    assert.ok(specAccount.taskProgram.equals(counterProgram.programId));
    assert.ok(specAccount.startTs.eq(startTs));
    assert.ok(specAccount.intervalTs.eq(intervalTs));
    assert.ok(specAccount.lastExecutedTs.toNumber() == 0);
  });

  it("run", async () => {
    await solcronProgram.rpc.runTask({
      accounts: {
        taskProgram: counterProgram.programId,
        taskSpecification: specAccountKey,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      },
      remainingAccounts: [
        { pubkey: counter.publicKey, isWritable: true, isSigner: false },
      ],
    });

    const counterAccount = await counterProgram.account.counter.fetch(
      counter.publicKey
    );
    assert.ok(counterAccount.count.toNumber() == 1);

    const specAccount = await solcronProgram.account.taskSpecification.fetch(
      specAccountKey
    );
    console.log("Spec account data:\n", specAccount);
    assert.ok(specAccount.lastExecutedTs.toNumber() != 0);
  });

  it("run invalid - program and spec mismatch", async () => {
    await assert.rejects(
      async () => {
        await solcronProgram.rpc.runTask({
          accounts: {
            taskProgram: solcronProgram.programId,
            taskSpecification: specAccountKey,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          },
          remainingAccounts: [
            { pubkey: counter.publicKey, isWritable: true, isSigner: false },
          ],
        });
      },
      (err: any) => {
        assert.equal(err.code, 146);
        return true;
      }
    );
  });

  it("run early", async () => {});

  it("run off schedule", async () => {});
});
