# Blastmud

A component of a Rust-based mud (multi-user text-based game). Unlike many muds, it is designed to
be completely Free software (other than an age-verification file - see below), with the game
core written in Rust rather than in any form of softcode. Only user data forms part of the database.
Even the map is programmed in a normal text editor, and can be
tested locally before being deployed to the game.

# Age verification file

The Blastmud game is for adults only (18 years of age or older). In order to make a complete game, three
components are required:

* This game server codebase - which is publicly available and shareable under a permissive (3-clause BSD-style license). It isn't playable as a game by itself.
* A client. Openly available software such as telnet, tintin++, or mudlet can be used as this component.
* A closed-source age verification file, to be placed as `age-verification.yml` alongside the `gameserver.conf` file. This file is copyrighted with all rights reserved (except for the use by the person it is intended for to run the game) and cannot legally be given to anyone else. The initial author of Blastmud intends to provide an `age-verification.yml` file to anyone I am satisfied is not a minor.

## Why does a Free/Open Source project deliberately include a requirement for a non-Open Source file?

In the jurisdiction where the initial author is based, it is illegal to distribute unclassified or R18+ classified games (defined as playable software / data / some combination of it) to people under 18. Restricting access to all components of the game would be an impediment for easy collaboration on the game.

So a decision was made to only distribute a non-playable Free / Open Source component without restrictions (and to ensure this non-playable component doesn't, by itself, meet the definition of either a computer game or a submittable publication).

## I obtained an `age-verification.yml` from the initial author - can I share it / publish it?

No, this file is licensed solely to you and it is a breach of copyright law to publish it without consent from the initial author. A takedown request for the material might be sent, the shared `age-verification.yml` might be revoked in future versions of the server codebase, and you could even be sued for copyright infringement.

Depending on your jurisdiction, publishing a complete game (including `age-verification.yml`) to people who are under 18 could also be a crime.

If you attempt to use the official Blastmud GitHub project (or any other resources) to share `age-verification.yml` (e.g. through issues or pull requests), the material will be deleted and you will be blocked from further interaction with the project (unless we are satisfied it was accidental).

You are allowed to put it on a computer system / server where it is only accessible to a limited number of people known to you, as long as you have verified all those people are 18 or over, and know not to further distribute the file.

## Can I change / remove the code so it doesn't need `age-verification.yml`?

The license for Blastmud allows you to change the code and redistribute your changes. If you are forking Blastmud to create your own game engine, you could change
the age verification keypair or entirely remove the code. You may not call such a modified game Blastmud. Please be aware that if
you modify the code to create a complete computer game, in some jurisdictions you might have to get your fork classified, and might
have legal obligations not to distribute it to anyone under a certain age.

Regarding the use of official Blastmud resources such as our GitHub project and game server instance: to ensure minors are protected, you must not post versions of Blastmud that disable the checking of `age-verification.yml` (or post any other complete unclassified game or game that is unsuitable for minors of any age), nor post patches, pull requests, or instructions for doing the same. You may be blocked from further interaction with the project if you do this (unless we are satisfied it was accidental).

# Architecture

Blastmud consists of the following main components:
* blastmud_listener is intended to be a long-running component that accepts connections from users. It can't be changed without disconnecting all users.
* blastmud_interfaces is a shared library defining the interface between the listener and the gameserver. It can't be changed without disconnecting all users.
* blastmud_game implements the gameserver. It does not hold any long-lived state, and so can be safely redeployed and replaced without disconnecting all users.
* A PostgreSQL database is used to store the entire state (user data) of the game. It is used with synchronous_commit turned off so that recently changed data is only stored in memory.

# Status

Blastmud is not yet playable, it is under development.

# Schema management
We only keep the latest version in version control, and use migra (pip3 install migra) to identify changes between
the production schema and the latest in code.

The latest schema is under `schema`.

Create a user with a secret password, and username `blast`. Create a production database called `blast`.

To get to the latest schema:
* Run `psql <schema/schema.sql` to create the temporary `blast_schemaonly` database.
* Run `migra "postgres:///blast" "postgres:///blast_schemaonly" > /tmp/update.sql`
* Check `/tmp/update.sql` and if it looks good, apply it with `psql -d blast </tmp/update.sql`
