const { one } = require('./config')

module.exports = scenario => {
  scenario('get_memo_returns_none', async (s, t) => {
    const {alice, bob, jack} = await s.players({alice: one, bob: one, jack: one}, true)

    await s.consistency()

    const get_bob = await bob.call("app", "peer_vis", "get_peers", {})
    // console.log("BOB----->", get_bob);
    t.deepEqual(get_bob.Ok.length, 3)

    await s.consistency()

    const get_alice = await alice.call("app", "peer_vis", "get_peers", {})
    // console.log("Alice---->", get_alice);
    t.deepEqual(get_alice.Ok.length, 3)

    await s.consistency()

    const get_jack = await jack.call("app", "peer_vis", "get_peers", {})
    // console.log("Jack---->", get_jack);
    t.deepEqual(get_jack.Ok.length, 3)
  })

}
