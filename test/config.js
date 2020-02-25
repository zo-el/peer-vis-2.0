const _ = require('lodash')
const path = require('path')
const { Config } = require('@holochain/tryorama')

const dnaPath = path.join(__dirname, "../dist/peer-vis-2.0.dna.json")
const dna = Config.dna(dnaPath, 'app-spec')

const networkType = "sim2h"
const network =
  ( networkType === 'websocket'
  ? Config.network('websocket')

  : networkType === 'memory'
  ? Config.network('memory')

  : networkType === 'sim2h'
  ? {
    type: 'sim2h',
    sim2h_url: 'ws://localhost:9000'
  }

  : (() => {throw new Error(`Unsupported network type: ${networkType}`)})()
)

const logger = {
  type: 'debug',
  rules: {
    rules: [
      {
        exclude: true,
        pattern: '.*parity.*'
      },
      {
        exclude: true,
        pattern: '.*mio.*'
      },
      {
        exclude: true,
        pattern: '.*tokio.*'
      },
      {
        exclude: true,
        pattern: '.*hyper.*'
      },
      {
        exclude: true,
        pattern: '.*rusoto_core.*'
      },
      {
        exclude: true,
        pattern: '.*want.*'
      },
      {
        exclude: true,
        pattern: '.*rpc.*'
      }
    ]
  },
  state_dump: true
}

const tracing = ({playerName}) => ({
  type: 'jaeger',
  service_name: `holochain-${playerName}`
})

const commonConfig = { logger, network, tracing }

module.exports = {
  one: Config.gen({
      app: dna
    },
    commonConfig
  )
}
