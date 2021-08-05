## Iterating over strings
This iterates over Unicode Scalar values:

``
for c in "whatever".chars()
``

This iterates over raw bytes (u8):
Remember that scalar values can be composed of more than one byte.

``
for b in "whatever".bytes()
``


# Error handling
Rust unwinds and cleans up variables when hitting a panic!() during normal circumstances.
We can change this behavior to simply aborting and letting OS do the cleanup (minimizing executable size)
by adding 
````
[profile]
panic = 'abort'

OR

[profile.release]
panic = 'abort'
````
in the Cargo.toml file.