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

const dnaPath = path.join(__dirname, "../dist/sub-contract-prototype.dna.json")

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
const dna_subscription = "subscription_dna";
const zome_contract = "contract";
const zome_provider = "provider";
const dna = Config.dna(dnaPath, dna_subscription);
const conductorConfig = Config.gen(
  { "subscription_dna": dna },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000"
    },
    logger: Config.logger({ type: "error" }),
  }
);

function log(content, title) {
  console.log("<<<<<<<<<<<<<<<<<<<<<<  " + title + "  >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
  console.log(content);
  console.log("<End>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

}

orchestrator.registerScenario("Scenario1", async (s, t) => {
  const { alice, bob, tom } = await s.players(
    { alice: conductorConfig, bob: conductorConfig, tom: conductorConfig },
    true
  );


  //START TEST SECTION///////////////////////////////////////// Alice create some contents: Alice is a Provider
  const alice_content_1_addr = await alice.call(
    dna_subscription,
    zome_provider,
    "create_feed",
    {
      blog: "my blog 1",
      contract_body: "the body of contract",
      comments: ["comment 1-1", "comment1-2"]
    }
  );
  await s.consistency();
  t.ok(alice_content_1_addr.Ok);
  log(alice_content_1_addr.Ok, "alice_content_1_addr");

  const alice_content_2_addr = await alice.call(
    dna_subscription,
    zome_provider,
    "create_feed",
    {
      blog: "my blog 2",
      contract_body: "the body of contract",
      comments: ["comment 2-1", "comment2-2"]
    }
  );
  await s.consistency();
  t.ok(alice_content_2_addr.Ok);
  log(alice_content_2_addr.Ok, "alice_content_2_addr");


  //START TEST SECTION///////////////////////////////////////// Alice gets 2 blogs
  const alice_contents = await alice.call(
    dna_subscription,
    zome_provider,
    "get_my_blogs",
    {
    }
  );
  await s.consistency();
  log(alice_contents.Ok, "Alice Contents");
  t.true(alice_contents.Ok[0] != null);
  t.true(alice_contents.Ok[1] != null);

  await s.consistency();


  //START TEST SECTION///////////////////////////////////////// Bob gets 0 blogs
  const bob_contents = await bob.call(
    dna_subscription,
    zome_provider,
    "get_my_blogs",
    {
    }
  );
  await s.consistency();
  log(bob_contents.Ok, "Bob Contents");
  t.true(bob_contents.Ok[0] == null);


  //START TEST SECTION///////////////////////////////////////// Bob wants to Subscribe to Alice contents as Silver-Membership 
  //// Notice: This implementation of Digital-Contact is just Fake. 
  //   These steps will be replaced by real Digital-Content working version. it should be one zome call, with some internal direct messaging

  // 1- Alice create a contract for Bob
  const bob_sub_contract_addr = await alice.call(
    dna_subscription,
    zome_contract,
    "create_subscribe_contract",
    {
      subscriber: bob.instance(dna_subscription).agentAddress,
      contract_type: "silver-membership",
    }
  );
  await s.consistency();
  log(bob_sub_contract_addr.Ok, "Bob subscription public address");
  t.ok(bob_sub_contract_addr.Ok);

  // 2- Bob will get the signature for the contract
  const bob_signature = await bob.call(
    dna_subscription,
    zome_contract,
    "get_my_signature",
    {
      entry_address: bob_sub_contract_addr.Ok,
    }
  );
  await s.consistency();
  log(bob_signature.Ok, "Bob Signature");
  t.ok(bob_signature.Ok);

  // 3- Bob sign the contract by his signature
  const bob_signed = await bob.call(
    dna_subscription,
    zome_contract,
    "sign_contract_by_subscriber",
    {
      contract_address: bob_sub_contract_addr.Ok,
      signature: bob_signature.Ok
    }
  );
  await s.consistency();
  log(bob_signed.Ok, "Bob Signed the contract");
  t.ok(bob_signed.Ok);

  //START TEST SECTION///////////////////////////////////////// Bob wants to get data from provider(Alice) using his contract(subscription) 
  const bob_subscription_content = await bob.call(
    dna_subscription,
    zome_provider,
    "get_subscription_blogs",
    {
      contract_address: bob_sub_contract_addr.Ok,
      signature: bob_signature.Ok,
    }
  );
  await s.consistency();
  log(bob_subscription_content.Ok, "bob_subscription_content");
  t.ok(bob_subscription_content.Ok);












  //START TEST SECTION///////////////////////////////////////// Tom wants to Subscribe to Alice contents as Gold-Membership 
  // 1- Alice create a contract for Tom
  const tom_sub_contract_addr = await alice.call(
    dna_subscription,
    zome_contract,
    "create_subscribe_contract",
    {
      subscriber: tom.instance(dna_subscription).agentAddress,
      contract_type: "gold-membership",
    }
  );
  await s.consistency();
  log(tom_sub_contract_addr.Ok, "Tom subscription public address");
  t.ok(tom_sub_contract_addr.Ok);

  // 2- Tom will get the signature for the contract
  const tom_signature = await tom.call(
    dna_subscription,
    zome_contract,
    "get_my_signature",
    {
      entry_address: tom_sub_contract_addr.Ok,
    }
  );
  await s.consistency();
  log(tom_signature.Ok, "Tom Signature");
  t.ok(tom_signature.Ok);

  // 3- Tom sign the contract by his signature
  const tom_signed = await tom.call(
    dna_subscription,
    zome_contract,
    "sign_contract_by_subscriber",
    {
      contract_address: tom_sub_contract_addr.Ok,
      signature: tom_signature.Ok
    }
  );
  await s.consistency();
  log(tom_signed.Ok, "Tom Signed the contract");
  t.ok(tom_signed.Ok);

  //START TEST SECTION///////////////////////////////////////// Tom wants to get data from provider(Alice) using his contract(subscription) 
  const tom_subscription_content = await tom.call(
    dna_subscription,
    zome_provider,
    "get_subscription_blogs",
    {
      contract_address: tom_sub_contract_addr.Ok,
      signature: tom_signature.Ok,
    }
  );
  await s.consistency();
  log(tom_subscription_content.Ok, "tom_subscription_content");
  t.ok(tom_subscription_content.Ok);


});




orchestrator.run();
