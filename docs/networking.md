# Networking

Local P2P is reserved for v0.5.

The current `dust node start` command is a safe local stub that reports node state but does not expose public-network behavior.

Planned localnet rules:

- reject oversized frames;
- reject malformed binary payloads;
- track invalid message counts;
- disconnect noisy local peers;
- never panic on malformed input.
