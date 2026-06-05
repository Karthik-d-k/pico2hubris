## pico2hubris

Hubris OS experiments on the Raspberry Pi Pico 2 (RP2350, ARM Cortex-M33) — exploring embedded peripherals (UART, SPI, GPIO, ...) on top of the Hubris kernel.

## Applications

| App | Peripheral | Description |
|-----|-----------|-------------|
| `gpio-blinky` | GPIO | Blink onboard LED (GPIO25) with busy-wait delays |
| `systick-heartbeat` | SysTick | Heartbeat LED pattern + UART uptime reporter using kernel timers |
| `uart-echo` | UART | UART0 echo at 115200 8N1 — polled TX/RX |
| `ipc-pingpong` | IPC | Ping/pong counter over `sys_send` with fault recovery |

## Prerequisites

- [**`Rust toolchain`**](https://rust-lang.org/tools/install/) — pinned via `rust-toolchain.toml`
- [**`just`**](https://github.com/casey/just) — command runner
- [**`hubake`**](https://github.com/cbiffle/exhubris-demo/) - Hubris build tool
- [**`humility-pico`**](https://github.com/thenewwazoo/humility/tree/bmatt/update-probe-rs) — Hubris debug tool
- [**`openocd-pico`**](https://github.com/raspberrypi/pico-sdk-tools/releases) — on-chip debugger
- **`USB-UART serial adapter`** — Pico Debug Probe (recommended) or FTDI-based adapter

## Build & Flash

This project uses [just](https://github.com/casey/just) as a command runner. See the `Justfile` for all available recipes.

```sh
# Build an app (default: gpio-blinky)
just build

# Build a specific app
just app=uart-echo build

# Flash to the board
just app=uart-echo flash

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
