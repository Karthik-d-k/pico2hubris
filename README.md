## pico2hubris

Hubris OS experiments on the Raspberry Pi Pico 2 (RP2350) using the ARM Cortex-M33 core.

This repository is a learning playground for exploring embedded communication peripherals such as
UART, SPI, GPIO, and other hardware interfaces while developing applications on top of the Hubris OS kernel.

## Applications

- **blinky** - LED blink demo
- **heartbeat** - Heartbeat echo app
- **pingpong** - Inter-task ping-pong messaging
- **uart** - UART communication

## Build & Flash

This project uses [just](https://github.com/casey/just) as a command runner. See the `Justfile` for all available recipes.

```sh
# Build an app (default: blinky)
just build

# Build a specific app
just app=uart build

# Flash to the board
just app=uart flash

# See all recipes
just help
```

## UART Output (Windows)

To view UART output on Windows, use PuTTY's `plink`:

```sh
plink.exe -serial COM3 -sercfg 115200,8,n,1,N
```

## Status

Work in progress.
