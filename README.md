## pico2hubris

Hubris OS experiments on the Raspberry Pi Pico 2 (RP2350) using the ARM Cortex-M33 core.

This repository is a learning playground for exploring embedded communication peripherals such as
UART, SPI, GPIO, and other hardware interfaces while developing applications on top of the Hubris OS kernel.

## Applications

- **blinky** - Blinks the on-board LED (GPIO25) in a simple on/off loop using busy-wait delays. A minimal single-task app that demonstrates bare-metal GPIO configuration via the RP2350 PAC.

- **heartbeat** - Two-task app combining a heartbeat LED pattern (two quick blinks followed by a 1s pause) with an uptime reporter that prints elapsed seconds over UART0 every 2 seconds. Demonstrates kernel timer syscalls (`sys_set_timer` / `sys_recv_notification`) for deadline-based scheduling instead of busy-waiting.

- **pingpong** - Two-task IPC demo. The *ping* task sends an incrementing counter (1-10) to the *pong* task via `sys_send`. The *pong* task receives the message, blinks the LED that many times, and replies. Demonstrates Hubris inter-task messaging, task slots, and generation-aware fault recovery.

- **uart** - Single-task UART echo. Configures UART0 (PL011) at 115200 8N1 on GPIO0/GPIO1, sends a greeting, then echoes every received byte back to the host. Demonstrates bare-metal UART peripheral setup and polled TX/RX.

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
