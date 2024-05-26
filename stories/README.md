These userstories tell stories we want to make possible in Pioneer.

# 01 - Creating a cluster (`01-creating-a-cluster.md`)
As a user, I want to open the Pioneer cli. It should spawn a Cursive based TUI.

The TUI will open for me, displaying an empty list of nodes and offering to create a new cluster.

Creating a new cluster should show a map with my node in the middle of it. It will initiate a new cluster and add my node to it.

If I click the plus arrow next to my node, it will give me a command that allows me to join another node to my cluster.

This will all be using stubs for now.

# 02 - Exchanging messages (`02-exchanging-messages.md`)
As a user, I want to be able to send a message from one node to another.

I can do this by using the `send` command in the TUI.

The TUI will prompt me for the node I want to send the message to and the message itself.

The message will be sent to the node and the TUI will update the map to show the message has been sent.

The map will update in realtime as each node receives the message. A random colour will be picked based on the hash of the message, and as each node receives that message, it will be displayed in that colour on the map.

This will all be using stubs for now, no real code will be used yet.