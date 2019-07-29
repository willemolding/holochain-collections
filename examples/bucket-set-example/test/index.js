const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/bucket-set-example.dna.json")
const dna = Diorama.dna(dnaPath, 'bucket-set-example')

const diorama = new Diorama({
  instances: {
    alice: dna
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

diorama.registerScenario("can add, retrieve, retrieve by bucket and get all", async (s, t, {alice}) => {
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

diorama.run()
