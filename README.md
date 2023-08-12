A simple tool to measure behavior of your friendly neighborhood NAT device

## What is this?

`natstat` records how NAT behaves in a network. This is done by sending packets
from a `client` with source port between `1-65535` through a NAT device, then
recording the observed reflexive source port as seen by a public `server`.

This is done multiple times per source port in order to surface any patterns
that may emerge.

## Running

```
docker compose build
docker compose up
```

## Client example command
`natstat client -s 10.0.0.72 -d 100.0.0.92:8080 -m 1234`

## Server example command
`natstat server -l 100.0.0.92:8080 -m 1234`
