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