[![Build Status](https://github.com/dakom/todomvc-shipyard/workflows/Build/badge.svg)](https://github.com/dakom/todomvc-shipyard/actions)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Demo](https://img.shields.io/badge/demo-launch-yellow)](https://dakom.github.io/todomvc-shipyard)


# TodoMVC - Shipyard

#### A TodoMVC written in pure Rust on top of the [Shipyard entity component system](https://github.com/leudz/shipyard).

It interfaces with the DOM directly via [web-sys](https://github.com/rustwasm/wasm-bindgen/tree/master/crates/web-sys), so in order to make that easier a few other dependencies are used:

* [awsm_web](https://github.com/dakom/awsm-web): dom helpers and utilities
* [simple-html-template](https://github.com/dakom/simple-html-template): very lightweight templates
* [gloo-events](https://github.com/rustwasm/gloo/tree/master/crates/events): event binding


### Rendering 

The dom updates themselves are split into 3 different workloads: tree updates, property updates (including styling), and event binding.

This means that when we get parallelism in wasm, all of the property updates and styling should be able to execute in parallel! We're not there yet, but the infrastructure is just waiting :D

### Updates

Right now, all the systems are executed at each update, even if they aren't necessary.

This is efficient enough since the "hot path" systems are gated with dirty flags, and if performance ever becomes a problem it can be solved by a number of techniques (more targetted system/workload runs, tracking more dirty flags, using update storages, etc.)

I'd say this is actually one of the best things of this approach.
We get to choose where to optimize ;)
Since the systems are split very granularly, it's much simpler to benchmark and fix.

### Techniques

Most of the techniques used here are shipyard-specific, for example it makes heavy use of [Tag Components](https://leudz.github.io/shipyard/guide/going-further/other-component-storage.html?highlight=tag#tag-components) which work wonderfully in shipyard, but probably wouldn't work as well in an archetype-based ECS like Unity's or Legion.

Another angle is thinking in a "ECS" or, more generally, "data-oriented" way. Not so much in terms of keeping the cache happy (though that too), but more like splitting things up and iterating over the storages across different systems. This takes some getting used to, but it's a fun and effective way to program.

There's actually a _ton_ of freedom in this approach overall to build things in different ways. Please consider this demo _a_ way to build on the DOM with an ECS, not _the_ way, even if we're talking shipyard-specific. For example, this demo doesn't even use [Update Packs](https://leudz.github.io/shipyard/guide/going-further/packs.html#update) at all, which would have been a way to make it more "reactive" out of the box.