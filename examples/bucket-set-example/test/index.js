// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

const dnaPath = "./dist/bucket-set-example.dna.json"
const agentAlice = Config.agent("alice")
const dna = Config.dna(dnaPath)
const instanceAlice = Config.instance(agentAlice, dna)
const scenario = new Scenario([instanceAlice], { debugLog: false })

scenario.runTape("can add, retrieve, retrieve by bucket and get all", async (t, { alice }) => {

  const addr = await alice.callSync("main", "create_my_entry", {"entry" : {"content":"sample content"}})
  const result = await alice.callSync("main", "get_my_entry", {"address": addr.Ok})

  // check for equality of the actual and expected results
  t.deepEqual(result, { Ok: {"content":"sample content"} })

  const getBucketResult = await alice.callSync("main", "get_entries_by_bucket", {"bucket_id": "s"})
  t.equal(getBucketResult.Ok.length, 1)

  const getAllResult = await alice.callSync("main", "get_all_entries", {})
  t.equal(getAllResult.Ok.length, 1)

  await alice.callSync("main", "create_my_entry", {"entry" : {"content":"more sample content"}})
  const getBucketResult2 = await alice.callSync("main", "get_entries_by_bucket", {"bucket_id": "m"})
  t.equal(getBucketResult2.Ok.length, 1)

  const getAllResult2 = await alice.callSync("main", "get_all_entries", {})
  t.equal(getAllResult2.Ok.length, 2)

})
