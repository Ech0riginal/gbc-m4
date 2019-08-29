import board
import displayio
import digitalio
from adafruit_st7735r import ST7735R
from adafruit_rgb_display import color565
import adafruit_rgb_display.st7735 as st7735

tft_cs = board.A2 #digitalio.DigitalInOut(board.A2)
tft_dc = board.A3 #digitalio.DigitalInOut(board.A3)
tft_rc = board.A4 #digitalio.DigitalInOut(board.A4)

spi = board.SPI()
displayio.release_displays()
dbus = displayio.FourWire(spi,command=tft_dc,chip_select=tft_cs, reset=tft_rc)
display = ST7735R(dbus, width=128, height=160, colstart=0, rowstart=0, bgr=True)
f = open("/fku.bmp", "rb")
odb = displayio.OnDiskBitmap(f)
face = displayio.TileGrid(odb,pixel_shader=displayio.ColorConverter())
splash = displayio.Group(max_size=10)
splash.append(face)
display.show(splash)
while True:
    pass