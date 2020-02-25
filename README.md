# PeerVis
A way to visually see who is online and connected as a peer in your Holochain app

PeerVis is a "Zome" that you can include in your Holochain app. This means it is a modular unit of code that is standalone, and won't affect the rest of your code.

![PeerVis screenshot](/screenshot.png)

To add it to your app, do the following.

1. Add the following to the Zomes `peer` in your dna


2. Copy the `peerVis` folder INTO the `dna` folder of your app

3. Copy `peervis.html` and `cytoscape.min.js` into the `ui` folder of your app

## How to use it
When you're running your application, navigate to the localhost address of your app, plus the path `/peervis.html`. This will dynamically update and show you who how many other nodes are connected to your node, that the app will be gossiping with.

## How it does this
In order to know who else is online, it attempts to send a node to node message to each peer who has written to the DHT about their presence in the app. Within the peerVis zome, it adds an entry during `genesis` announcing their presence within the app.
So it uses the `send` method of the api, and listens for a response from other nodes. It does this at a regular interval.
