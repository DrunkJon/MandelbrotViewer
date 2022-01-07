import mandelbrot_module as mandelbrot

dims = (1920, 1080)

plot = mandelbrot.PlotWindow(dims)
print(plot)
plot = plot.zoom((dims[0] / 2, dims[1] / 2),0.5)
print(plot)
plot.load_mandelbrot(150)
plot.load_julia((0.25, 0.0), 150)
