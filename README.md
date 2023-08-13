# natstat

A simple tool to measure behavior of your friendly neighborhood NAT device

## What is this

`natstat` records how NAT behaves in a network. This is done by sending packets
from a `client` with source port between `1-65535` through a NAT device, then
recording the observed reflexive source port as seen by a public `server`.

This is done multiple times per source port in order to surface any patterns
that may emerge.

## Running server

Make sure to run the server without any NAT so that source ports are directly
observed.

```
docker compose build server
docker compose up server
```

## Running client

Run the client to send NAT probes to the server. The client should be behind one
or more NATs to observe the reflexive port allocated.

Ensure you run the client with the same magic integer as you provided the
server.

```
docker compose build server
docker compose up client
```

## Client example command

`natstat client -s 10.0.0.72 -d 100.0.0.92:10080 -m 1234`

## Server example command

`natstat server -l 100.0.0.92:10080 -m 1234`

```

```
