[![Build Status](https://travis-ci.com/allison-knauss/sunrise.svg?branch=master)](https://travis-ci.com/allison-knauss/sunrise)

# sunrise
Epic-scale game built in rust on amethyst

## High Concept

Sunrise is a huge idea, meant to be something of a magnum opus. It is a game created from several different systems, each system big enough and interesting enough to be the core of its own game.

## Running

All server applications will be runnable via `docker-compose -f stack.yml up -d`. Client applications will be built as executables as appropriate. Currently the docker-compose stack only spins up a postgresql database to use. Connection information is set in the config in the util/ sub-project.

## Database

This project currently uses a single postgresql database. Interface for this database is implemented in `util::sunrise_db`. Configuration is the config file at `src/util/config.toml`.

## Roadmap

To achieve the high concept, a series of proof-of-concept games will be built. First, games specifically about each system (defined below) will be created. Then games exploring the intersection between two or more systems will be created. Only then will Sunrise itself, with the weight of multiple titles and many hours behind it, be created. These games shall also include building tooling appropriate for the system(s) being used which can then be reused in the future. The purpose of creating these games is to help ensure that each system is fun and feels good on its own.

## Game Systems

1. Economic system
2. Faction / relationship / sentiment system
3. Smart, Machine Learning-driven, needs-based NPCs
4. Rich PC definition and character growth
5. Permanent changes to a shared world

### Economic System

Lots of resources, resource production and consumption, dynamic supply & demand. This should be fun as the core model for both a Railroad Tycoon-esque game and an RTS base-building game.

### Faction / Relationship / Sentiment System

One system that models both individual character and group relationships and sentiments. Sentiment, relationships, and group membership shall all change over time based on actions taken. Inspirations are MMORPG-esque reputations (but more fun), Bioware-esque dialog wheels, and Sims-esque character relationships.

### Smart, Machine Learning-driven, Needs-based NPCs

The core NPC behaviour system needs to feel really good. A negative example is how Civilization-esque game NPCs handle trade: I probably don't want to trade my newly-acquired relic to you, even if you ask every year, and I probably won't accept your awkwardly one-sided trade. This system is the most loosely defined at this point, because I don't really know what fun for this looks like without playing with it.

### Rich PC definition and character growth

I don't enjoy when games lock you into a "class", such as with WoW. I also don't enjoy when games give no constraints at all to what you can do, such as FFXIV's weapon-tied class mechanic. I want my character to grow based on their experiences and learn from those they spend time with in a way that feels slightly closer to real. I want limitations and hard choices but also ample freedom to explore and try different things.

### Permanent changes to a shared world

Games where you run through identical respawned content feel bad. Phasing is an improvement, but it isolates and separates people. I want player actions (as well as NPC actions) to permanently change the world. I want to build things everyone can enjoy. I want to destroy things to prevent others from enjoying them. Inspirations are sandbox-esque construction and Eve Online's null space.

## Technical Systems

Sunrise's concept games shall all be built on a shared platform including a microservice-structured server and unique, individual clients. The server shall be shared across all concept games and Sunrise itself. The technical systems and expectations needed shall include those listed below.

1. Authentication / Authorization / Accounting (AAA)
2. Unified logging and telemetry
3. Sufficiently performant routing client<->client, client<->server, and server<->server 
4. High-quality unit tests and documentation always. Preferrably TDD or BDD
5. A baseline client application template used for all the different games
6. Everything-is-an-ECS design. Amethyst as the core of client applications, Specs as the core of server applications.

## References

- Amethyst game engine - https://github.com/amethyst/amethyst
- Specs Parallel ECS - https://github.com/slide-rs/specs
