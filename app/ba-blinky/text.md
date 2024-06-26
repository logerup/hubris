name = "stm32h743-nucleo-ba"
target = "thumbv7em-none-eabihf"
board = "nucleo-h743zi2"
chip = "../../chips/stm32h7"
stacksize = 896

[kernel]
name = "stm32h7-nucleo-ba"
requires = {flash = 24000, ram = 2300} #changed ram allocation from 5120
features = ["h743"] #commented out dump , "dump"

[tasks.jefe]
name = "task-jefe"
priority = 0
max-sizes = {flash = 16384, ram = 2048}
start = true
#features = ["dump"] 
stacksize = 1536
notifications = ["fault", "timer"]
extern-regions = [ "sram2", "sram3", "sram4" ]

[tasks.jefe.config.allowed-callers]
set_reset_reason = ["sys"]
request_reset = ["hiffy"]

[tasks.sys]
name = "drv-stm32xx-sys"
features = ["h743"]
priority = 1
max-sizes = {flash = 2048, ram = 1024}
uses = ["rcc", "gpios", "system_flash"]
start = true
task-slots = ["jefe"]

[tasks.user_leds]
name = "drv-user-leds"
features = ["stm32h7"]
priority = 2
max-sizes = {flash = 2048, ram = 1024}
start = true
task-slots = ["sys"]
notifications = ["timer"]

[tasks.hiffy]
name = "task-hiffy"
features = ["h743", "stm32h7"]#commented out [, "i2c", "gpio", "spi", "rng"]
priority = 4
max-sizes = {flash = 32768, ram = 32768 }
stacksize = 2048
start = true
task-slots = ["sys"] #commented out , "i2c_driver", "rng_driver"

[tasks.idle]
name = "task-idle"
priority = 5 # changed prio from 6 ot 5, because no prio 5 task
max-sizes = {flash = 128, ram = 256}
stacksize = 256
start = true

#own task oriented on provided pong

[tasks.blinky]
name = "task-blinky"
priority = 3
max-sizes = {flash = 1024, ram = 1024}
start = true
task-slots = ["user_leds"]
notifications = ["timer"]

# pasting commented out sections below, for more readability above, pastes are from top to bottom

#[tasks.i2c_driver]
#name = "drv-stm32xx-i2c-server"
#features = ["h743"]
#priority = 2
#max-sizes = {flash = 16384, ram = 4096}
#uses = ["i2c1", "i2c2", "i2c3", "i2c4"]
#start = true
#task-slots = ["sys"]
#notifications = ["i2c1-irq", "i2c2-irq", "i2c3-irq", "i2c4-irq"]

#[tasks.i2c_driver.interrupts]
#"i2c1.event" = "i2c1-irq"
#"i2c1.error" = "i2c1-irq"
#"i2c2.event" = "i2c2-irq"
#"i2c2.error" = "i2c2-irq"
#"i2c3.event" = "i2c3-irq"
#"i2c3.error" = "i2c3-irq"
#"i2c4.event" = "i2c4-irq"
#"i2c4.error" = "i2c4-irq"

#[tasks.spi_driver]
#name = "drv-stm32h7-spi-server"
#priority = 2
#max-sizes = {flash = 16384, ram = 4096}
#features = ["spi1", "h743"]
#uses = ["spi1"]
#start = true
#notifications = ["spi-irq"]
#interrupts = {"spi1.irq" = "spi-irq"}
#stacksize = 880
#task-slots = ["sys"]

#[tasks.net]
#name = "task-net"
#stacksize = 3000
#priority = 2
#max-sizes = {flash = 65536, ram = 8192, sram1 = 32768}
#features = ["h743"]
#sections = {eth_bulk = "sram1"}
#uses = ["eth", "eth_dma", "tim16"]
#start = true
#notifications = ["eth-irq", "mdio-timer-irq", "wake-timer"]
#interrupts = {"eth.irq" = "eth-irq", "tim16.irq" = "mdio-timer-irq"}
#task-slots = ["sys", "jefe"]

#[tasks.ping]
#name = "task-ping"
#features = []
#priority = 5
#max-sizes = {flash = 8192, ram = 1024}
#start = true
#task-slots = [{peer = "pong"}]

#[tasks.pong]
#name = "task-pong"
#priority = 3
#max-sizes = {flash = 1024, ram = 1024}
#start = true
#task-slots = ["user_leds"]
#notifications = ["timer"]

#[tasks.udpecho]
#name = "task-udpecho"
#priority = 3
#max-sizes = {flash = 16384, ram = 8192}
#stacksize = 4096
#start = true
#task-slots = ["net"]
#notifications = ["socket"]

#[tasks.rng_driver]
#features = ["h743"]
#priority = 3
#name = "drv-stm32h7-rng"
#max-sizes = {flash = 8192, ram = 512}
#stacksize = 256
#start = true
#task-slots = ["sys", "user_leds"]
#uses = ["rng"]

#[tasks.dump_agent]
#name = "task-dump-agent"
#features = ["no-rot"]
#priority = 4
#max-sizes = {flash = 32768, ram = 2048 }
#start = true
#task-slots = ["jefe"]
#stacksize = 1200
#extern-regions = [ "sram2", "sram3", "sram4" ]

#[config]

#[[config.i2c.controllers]]
#controller = 2

#[config.i2c.controllers.ports.F]
#scl.pin = 1
#sda.pin = 0
#af = 4

#
# To use the Nucleo board as an SPD initiator, uncomment the following:
#
# [[config.i2c.controllers.ports.F.muxes]]
# driver = "ltc4306"
# address = 0b1001_010

#[config.spi.spi1]
#controller = 1
#fifo_depth = 16

#[config.spi.spi1.mux_options.cn7_arduino]
#outputs = [
#    {port = "A", pins = [5], af = 5}, # sck
#    {port = "B", pins = [5], af = 5}, # mosi
#]
#input = {port = "A", pin = 6, af = 5} # miso

#[config.spi.spi1.devices.pins]
#mux = "cn7_arduino"
#cs = [{port = "D", pin = 14}]
#clock_divider = "DIV16"


#[config.net]
# UDP ports in sockets below are assigned in oxidecomputer/oana

#[config.net.sockets.echo]
#kind = "udp"
#owner = {name = "udpecho", notification = "socket"}
#port = 7
#tx = { packets = 3, bytes = 1024 }
#rx = { packets = 3, bytes = 1024 }