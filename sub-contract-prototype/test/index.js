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


  //START TEST SECTION///////////////////////////////////////// Alice gets 2 blogs. She is a provider 
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


  //START TEST SECTION///////////////////////////////////////// Bob gets 0 blogs, because he is not a provider
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

  const bob_subscribtion = await fake_contract(alice, bob, "silver-membership", s, t);
  const bob_subscription_content = await bob.call(
    dna_subscription,
    zome_provider,
    "get_subscription_blogs",
    {
      contract_address: bob_subscribtion
    }
  );
  await s.consistency();
  log(bob_subscription_content.Ok, "bob_subscription_content");
  t.ok(bob_subscription_content.Ok);
  const result_bob_contents = JSON.parse(bob_subscription_content.Ok);
  // Bob has Siver-Membership so, he can not accesst to Comments 
  t.deepEqual(result_bob_contents,
    { Ok: [{ blog: 'my blog 2', comments: [] }, { blog: 'my blog 1', comments: [] }] }
  );

  //START TEST SECTION///////////////////////////////////////// Tom subscribe himself with gold-membership permission 
  const tom_subscribtion = await fake_contract(alice, tom, "gold-membership", s, t);
  const tom_subscription_content = await tom.call(
    dna_subscription,
    zome_provider,
    "get_subscription_blogs",
    {
      contract_address: tom_subscribtion
    }
  );
  await s.consistency();
  log(tom_subscription_content.Ok, "tom_subscription_content");
  t.ok(tom_subscription_content.Ok);

  const result_tom_contents = JSON.parse(tom_subscription_content.Ok);
  // Bob has Gold-Membership so, he can accesst to all data 
  t.deepEqual(result_tom_contents,
    { Ok: [{ blog: 'my blog 2', comments: ['comment 2-1', 'comment2-2'] }, { blog: 'my blog 1', comments: ['comment 1-1', 'comment1-2'] }] }
  );

});

//// Notice: This implementation of Digital-Contact is just Fake. 
//   These steps will be replaced by real Digital-Content working version. it should be one zome call, with some internal direct messaging
async function fake_contract(provider_caller, subscriber_caller, subscription_type, s, t) {

  //START TEST SECTION///////////////////////////////////////// X wants to Subscribe to Alice contents as Gold-Membership 
  // 1- Alice create a contract for X
  const sub_contract_addr = await provider_caller.call(
    dna_subscription,
    zome_contract,
    "create_subscribe_contract",
    {
      subscriber: subscriber_caller.instance(dna_subscription).agentAddress,
      contract_type: subscription_type,
    }
  );
  await s.consistency();
  log(sub_contract_addr.Ok, "X subscription public address");
  t.ok(sub_contract_addr.Ok);

  // 2- Tom will get the signature for the contract
  const signature = await subscriber_caller.call(
    dna_subscription,
    zome_contract,
    "get_my_signature",
    {
      entry_address: sub_contract_addr.Ok,
    }
  );
  await s.consistency();
  log(sub_contract_addr.Ok, "X Signature");
  t.ok(sub_contract_addr.Ok);

  // 3- Tom sign the contract by his signature
  const signed = await subscriber_caller.call(
    dna_subscription,
    zome_contract,
    "sign_contract_by_subscriber",
    {
      contract_address: sub_contract_addr.Ok,
      signature: signature.Ok
    }
  );
  await s.consistency();
  log(signed.Ok, "X Signed the contract");
  t.ok(signed.Ok);
  log(sub_contract_addr.Ok, "X subscription contract");

  return sub_contract_addr.Ok;
}

orchestrator.run();
