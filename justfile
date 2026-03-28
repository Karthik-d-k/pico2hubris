# Default app name (override with: just app=myapp <recipe>)
app := "blinky"

# Common aliases
alias b := build
alias r := reboot
alias f := flash
alias g := gdb
alias o := openocd
alias e := entry-points
alias c := clean

default: help

# Show available recipes and usage info
help:
    @echo "Hubris Build System"
    @echo "==================="
    @echo ""
    @echo "Current app: {{app}}"
    @echo ""
    @echo "Usage: just [app=<name>] <recipe>"
    @echo ""
    @echo "Override the default app (blinky) for any recipe:"
    @echo "  just app=myapp build"
    @echo "  just app=myapp flash"
    @echo ""
    @just --list --unsorted

build:
    hubake build apps/{{app}}-app.kdl
    hubake pack-hex .work/{{app}}/final/ {{app}}-output.hex -g {{app}}-gdbconfig

reboot:
    picotool reboot -u -c arm

flash:
    humility-pico -a {{app}}-build.zip flash -F
    humility-pico -a {{app}}-build.zip tasks

entry-points:
    @for elf in .work/{{app}}/final/*; do \
        name=$(basename "$elf"); \
        ep=$(arm-none-eabi-readelf -h "$elf" | grep 'Entry point address' | awk '{print $$4}'); \
        printf "%-15s Entry Point: %s\n" "$name" "$ep"; \
    done

gdb:
    arm-none-eabi-gdb -x {{app}}-gdbconfig

openocd:
    openocd-pico -f openocd.cfg -c "program {{app}}-output.hex verify"

clean:
    cargo clean
    rm -rf .work/{{app}}/
    rm -rf {{app}}-build.zip 
    rm -f {{app}}-output.hex {{app}}-gdbconfig

clean-all:
    cargo clean
    rm -rf .work/
    rm -rf *-build.zip 
    rm -f *-output.hex *-gdbconfig
