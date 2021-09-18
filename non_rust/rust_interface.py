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
        self.x_min, self.x_max = x_range
        self.x_dif = self.x_max - self.x_min
        self.y_min, self.y_max = y_range
        self.y_dif = self.y_max - self.y_min

    def __repr__(self) -> str:
        return f"{self.pixel_dimensions}: x=({self.x_min}, {self.x_max}) y=({self.y_min}, {self.y_max})"

    def pix_to_cords(self, pix_x, pix_y):
        x = self.x_min + self.x_dif * pix_x / self.pixel_dimensions[0]
        y = self.y_min + self.y_dif * (self.pixel_dimensions[1] - pix_y) / self.pixel_dimensions[1]
        return x, y

    def zoom(self, p: tuple, factor=0.8) -> "PlotWindow":
        new_x_dif = self.x_dif * factor
        new_y_dif = self.y_dif * factor

        new_x_min = p[0] - 0.5 * new_x_dif
        new_x_max = p[0] + 0.5 * new_x_dif

        new_y_min = p[1] - 0.5 * new_y_dif
        new_y_max = p[1] + 0.5 * new_y_dif
        if factor >= 1.0:
            pass
        else:
            # new window should be contained in old window
            # fit x
            if new_x_min < self.x_min:
                new_x_min = self.x_min
                new_x_max = new_x_min + new_x_dif
            elif new_x_max > self.x_max:
                new_x_max = self.x_max
                new_x_min = self.x_max - new_x_dif
            # fit y
            if new_y_min < self.y_min:
                new_y_min = self.y_min
                new_y_max = new_y_min + new_y_dif
            elif new_y_max > self.y_max:
                new_y_max = self.y_max
                new_y_min = self.y_max - new_y_dif
        
        return PlotWindow(self.pixel_dimensions, (new_x_min, new_x_max), (new_y_min, new_y_max))

    def load_mandelbrot(self, scale: int, tries: int):
        return mandelbrot.fine_mandelbrot(self.x_min, self.x_max, self.y_max, self.y_min, scale, MANDEL_FILE, tries)

    def load_julia(self, c_cords: tuple, scale: int, tries: int):
        return mandelbrot.fine_julia(c_cords[0], c_cords[1], self.x_min, self.x_max, self.y_max, self.y_min, scale, JULIA_FILE, tries)
