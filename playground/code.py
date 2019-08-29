import board
import displayio
from time import sleep
from adafruit_st7735 import ST7735

D_PINS = [board.D0,board.D1,board.D2,board.D3,board.D4,board.D5,board.D7,board.D9,board.D10,board.D11,board.D12,board.D13]
A_PINS = [board.A0,board.A1,board.A2, board.A3,board.A4,board.A5]

spi = board.SPI()
init_sequence = (b"\xe1\x0f\x00\x0E\x14\x03\x11\x07\x31\xC1\x48\x08\x0F\x0C\x31\x36\x0F" # Set Gamma
                 b"\x11\x80\x78"# Exit Sleep then delay 0x78 (120ms)
                 b"\x29\x80\x78"# Display on then delay 0x78 (120ms)
                )
def disp(tft_cs, tft_dc):
    displayio.release_displays()
    display_bus = displayio.FourWire(spi, command=init_sequence, chip_select=tft_cs, reset=board.A3)

    display = ST7735(display_bus, width=128, height=128)

    # Make the display context
    splash = displayio.Group(max_size=10)

    color_bitmap = displayio.Bitmap(128, 128, 1)
    color_palette = displayio.Palette(1)
    color_palette[0] = 0xFF0000

    bg_sprite = displayio.TileGrid(color_bitmap,
                                   pixel_shader=color_palette,
                                   x=0, y=0)
    splash.append(bg_sprite)
    display.show(splash)
#for A in A_PINS:
#    for D in D_PINS:
#        print("a:",A)
#        print("d:",D)
#        disp(A, D)
#        sleep(.5)
disp(board.A5, board.D9)
while True:
    pass