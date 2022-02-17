# Interpreter Dispatch Research

A collections of different instruction dispatching techniques
that can be used in Rust code to drive interpreters.

You can "benchmark" the techniques that have been implemented so far using
the following command:

```bash
time cargo test --release {name}::counter_loop
```

Where `{name}` is one of

- `switch`
- `switch_tail`
- `closure_loop`
- `closure_tree`
- `closure_tail`

# Architectures

All benchmark results are performed on my personal machine.

## `switch` Technique

This is the usual `switch` based instruction dispatch that is probably
most commonly used amonst simple interpreters.
It defines an `enum` with all the bytecodes and their parameters.
For execution it simply loops over the instructions and matches on the
kind of the next instruction that is about to be executed.

Benchmark result: `497.41ms`

## `switch_tail` Technique

This defines an `enum` for all bytecodes just like the `switch` technique.
For the execution every instruction calls the next instruction instead of
using a central loop. This way every single instruction contains the switch
over the instruction kind which may result in bloated compiled source code.

Benchmark result: `657.92ms`

## `closure_loop` Technique

Similar to the `switch` technique it uses a central loop to dispatch the
next instruction, however, it replaces the central match with a indirect
call to boxed closures that contain everything that is required for the
instruction execution.

Benchmark result: `933.29ms`

## `closure_tree` Technique

This one has a very interesting instruction structure.
Similar to the `closure_loop` technique it uses closures to model instructions.
However, for every instruction that may be followed by other instructions
we allow the internal closure to directly call the next instruction indirectly.

This way we can model the control flow according to basic blocks whereas
all basic blocks of the source input have exactly one root closure that calls
the rest via tail calls.

Benchmark result: `1019.01ms`

## `closure_tail` Technique

This is very similar to the `switch_tail` technique.
The only notable difference is that instructions are now organized as
closures as in `closure_loop` and `closure_tree` techniques.

Benchmark result: `867.14ms`
