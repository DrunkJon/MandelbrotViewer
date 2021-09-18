import mandelbrot


MANDEL_FILE = "./renders/mandel.png"
JULIA_FILE = "./renders/julia.png"


def load_mandelbrot(scale: int, tries: int):
    assert scale >= 0
    assert tries >= 0

    return mandelbrot.mandelbrot(scale, MANDEL_FILE, tries)

def load_julia(c_cords: tuple, scale: int, tries: int):
    assert scale >= 0
    assert tries >= 0

    jx, jy = c_cords

    return mandelbrot.julia(jx, jy, scale, JULIA_FILE, tries)


X_DIF = 2.1333
Y_DIF = 1.2


class PlotWindow:
    def __init__(self, pixel_dimensions: tuple, x_range: tuple = (-X_DIF, X_DIF), y_range: tuple = (-Y_DIF, Y_DIF)) -> None:
        self.pixel_dimensions = pixel_dimensions
        self.x_min, x_max = x_range
        self.x_dif = x_max - self.x_min
        self.y_min, y_max = y_range
        self.y_dif = y_max - self.y_min

    def pix_to_cords(self, pix_x, pix_y):
        x = self.x_min + self.x_dif * pix_x / self.pixel_dimensions[0]
        y = self.y_min + self.y_dif * pix_y / self.pixel_dimensions[1]
        return x, y

    def zoom(self, factor=0.8) -> "PlotWindow":
        pass
