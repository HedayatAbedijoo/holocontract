/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require("path");

const {
  Orchestrator,
  Config,
  combine,
  singleConductor,
  localOnly,
  tapeExecutor
} = require("@holochain/tryorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/DigitalContract.dna.json")

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require("tape")),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly
  )
});
const dna_name = "contract_dna";
const zome_name = "contract";
const dna = Config.dna(dnaPath, dna_name);
const conductorConfig = Config.gen(
  { "contract_dna": dna },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000"
    },
    logger: Config.logger({ type: "error" }),
  }
);


async function show_entry(caller, address, title) {
  console.log("<<<<<<<<<<<<<<<  " + title + "  >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
  const result =
    await caller.call(dna_name, zome_name, "get_entry", {
      address: address

    });

  //console.log(title);
  console.log(result);
  console.log("<End>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
}
orchestrator.registerScenario("Scenario1", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const pub_contract_adrr = await alice.call(
    dna_name,
    zome_name,
    "create_contract",
    {
      title: "First contract",
      contract_body: "the body of contract",
      contractor_address: bob.instance(dna_name).agentAddress,
      timestamp: 123
    }
  );
  console.log("_alice_created_public_contract");
  console.log(pub_contract_adrr);
  t.ok(pub_contract_adrr.Ok);
  await s.consistency();


  //// Bob is going to confirm contract
  const pub_contract_confirm_by_bob = await bob.call(
    dna_name,
    zome_name,
    "confirm_contract",
    {
      title: "First contract",
      body: "the body of contract",
      public_contract_address: pub_contract_adrr.Ok,
      timestamp: 654
    }
  );

  await s.consistency();
  t.ok(pub_contract_confirm_by_bob.Ok);

  await s.consistency();

  await show_entry(alice, pub_contract_adrr.Ok, "_show_public_again");

  await show_entry(alice, pub_contract_confirm_by_bob.Ok[0], "_bob_private_entry");


  // /// Get private entry and check
  // const public_contract = await alice.call(dna_name, zome_name, "get_entry", {
  //   address: pub_contract_adrr.Ok
  // });
  // await s.consistency();
  // console.log("_public_contract");
  // t.ok(public_contract.Ok);
  // console.log(public_contract);
  // await s.consistency();

  // console.log("_alice_private_contract");
  // const contract = JSON.parse(new_contr_pub.Ok.App[1]);
  // console.log(contract);
  // t.deepEqual(contract, {
  //   title: "First contract",
  //   timestamp: 123,
  //   teacher_address: alice.instance("course_dna").agentAddress,
  //   modules: []
  // });


  // //////////////////
  // const get_chain_entries_alice = await alice.call(dna_name, zome_name, "my_contracts", {
  // });
  // console.log("_this_is_private_full_chain_alice");
  // console.log(get_chain_entries_alice.Ok);
  // await s.consistency();
  // //////////////////////
  // const get_chain_entries_bob = await bob.call(dna_name, zome_name, "my_contracts", {
  // });
  // console.log("_this_is_private_full_chain_bob");
  // console.log(get_chain_entries_bob.Ok);
  // await s.consistency();
  // //////////////////////
  // console.log("_final_result");
  // const course = JSON.parse(pub_contract_adrr.Ok.App[1]);
  // console.log(course);

  // const index0 = await alice.call(dna_name, zome_name, "get_entry", {
  //   address: pub_contract_adrr.Ok[0]
  // });
  // console.log("_this_is_me_[0]_index]");
  // console.log(index0);
  // t.ok(index0.Ok);
  // await s.consistency();

  // const index1 = await bob.call(dna_name, zome_name, "get_entry", {
  //   address: pub_contract_adrr.Ok[1]
  // });

  // console.log("_this_is_me_[1]_index]");
  // console.log(index1);
  // t.true(index1.Ok == null);
  await s.consistency();

});


orchestrator.run();
