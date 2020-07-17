[![Build Status](https://github.com/dakom/todomvc-shipyard/workflows/Build/badge.svg)](https://github.com/dakom/todomvc-shipyard/actions)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Demo](https://img.shields.io/badge/demo-launch-yellow)](https://dakom.github.io/todomvc-shipyard)

# Work in progress...


### Rendering 
Most systems are executed together via a workload, even if they aren't necessary
This is efficient enough since the "hot path" systems are gated with dirty flags 

Performance, if it ever becomes a problem, can be solved by
Either splitting the renderings up to be more targeted (not part of the workload)
Or maintaining more state in the world (e.g. track more dirties)

I'd say this is actually one of the nice things of this approach
We get to choose where to optimize
Since the systems are split very granularly, it's much simpler to benchmark and fix

In fact, one split which is there is between the workloads - tree updates vs css updates
Since they are split this way, the CSS updates can one day run in parallel!!!


