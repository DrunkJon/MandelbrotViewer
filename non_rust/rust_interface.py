import mandelbrot_module as mandelbrot


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
