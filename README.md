# Blastmud

A Rust-based mud (multi-user text-based game). Unlike many muds, it is designed to be completely
Free software, with the game core written in Rust rather than in any form of softcode. Only user
data forms part of the database. Even the map is programmed in a normal text editor, and can be
tested locally before being deployed to the game.

# Architecture

Blastmud consists of the following main components:
* blastmud_listener is intended to be a long-running component that accepts connections from users. It can't be changed without disconnecting all users.
* blastmud_interfaces is a shared library defining the interface between the listener and the gameserver. It can't be changed without disconnecting all users.
* blastmud_game implements the gameserver. It does not hold any long-lived state, and so can be safely redeployed and replaced without disconnecting all users.
* A PostgreSQL database is used to store the entire state (user data) of the game. It is used with synchronous_commit turned off so that recently changed data is only stored in memory.

# Status

Blastmud is not yet playable, it is under development.
