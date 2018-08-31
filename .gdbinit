target remote | openocd -f "interface/stlink-v2.cfg" -f "target/stm32f1x.cfg" -c "gdb_port pipe; log_output openocd.log"

# target remote :3333

monitor arm semihosting enable
load
step