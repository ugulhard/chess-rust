# chess-rust
A chess program, and hopefully eventually a chess AI and simple frontend as well.
Here is some diagram I have created that guides my design: https://app.diagrams.net/#G18vfKKMe8j7b8O6J_M1d448EbN0gUzCpQ

# Testing
My goal is to create tests for all piece movement, several checkmate positions, pawn promoton, en passant, castling and several positions involving checks or pins.

For piece movement, unit tests for 
 - Normal movement patterns.
 - Blocked movement.
 - Captures.
Are considered a minimum, with potentially some special cases for the knight as it jumps.
