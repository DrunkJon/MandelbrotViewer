from main_window import *
from PyQt5 import QtCore
from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QApplication, QMainWindow
from PyQt5.QtGui import QPixmap

import sys
from enum import Enum, auto

import rust_interface


class Modes(Enum):
    Mandelbrot = auto()
    Julia = auto()


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.ui = Ui_MainWindow()
        self.ui.setupUi(self)

        # params
        self.scale = 120    # 120 = (1920, 1080), 80 = (1280, 720) ...
        self.tries = 100
        self.julia_cords = (0.25, 0.0)

        # render Items
        self.mandel_scene = None
        self.mandel_valid = False
        self.julia_scene = None
        self.julia_valid = False

        # only used for cord translation
        self.plot_window = rust_interface.PlotWindow((16 * self.scale, 9 * self.scale))
        
        # render first image
        self.mode = Modes.Mandelbrot
        self.mandel_path = None
        self.julia_path = None
        self.load_image()

    def resizeEvent(self, a0: QtGui.QResizeEvent) -> None:
        self.resize_image()
        return super().resizeEvent(a0)

    def load_image(self):
        if self.mode == Modes.Mandelbrot:
            self.load_mandelbrot()
        elif self.mode == Modes.Julia:
            self.load_julia()

    def load_mandelbrot(self):
        if not self.mandel_valid:
            # generate mandel_scene
            self.mandel_path = rust_interface.load_mandelbrot(self.scale, self.tries)
            pixmap = QPixmap(self.mandel_path)

            self.mandel_item = QtWidgets.QGraphicsPixmapItem(pixmap)
            self.mandel_item.setTransformationMode(Qt.SmoothTransformation)
            
            self.mandel_scene = ResponsiveScene(self)
            self.mandel_scene.addItem(self.mandel_item)

            self.mandel_valid = True
        # render mandel_scene
        self.set_scene(self.mandel_scene)
        self.resize_image()

    def load_julia(self):
        if not self.julia_valid:
            # generate julia_scene
            self.julia_path = rust_interface.load_julia(self.julia_cords, self.scale, self.tries)
            pixmap = QPixmap(self.julia_path)

            self.julia_item = QtWidgets.QGraphicsPixmapItem(pixmap)
            self.julia_item.setTransformationMode(Qt.SmoothTransformation)
            
            self.julia_scene = ResponsiveScene(self)
            self.julia_scene.addItem(self.julia_item)

            self.julia_valid = True
        # render julia_scene
        self.set_scene(self.julia_scene)
        self.resize_image()

    def set_scene(self, scene):
        self.ui.graphicsView.setScene(scene)

    def resize_image(self):
        item = self.mandel_item if self.mode == Modes.Mandelbrot else self.julia_item
        self.ui.graphicsView.fitInView(item, QtCore.Qt.KeepAspectRatio)

    def change_mode(self):
        if self.mode == Modes.Mandelbrot:
            self.mode = Modes.Julia
            self.load_julia()
        elif self.mode == Modes.Julia:
            self.mode = Modes.Mandelbrot
            self.load_mandelbrot()
        

class ResponsiveScene(QtWidgets.QGraphicsScene):
    def __init__(self, window: MainWindow):
        self.window = window
        super().__init__()

    def mouseDoubleClickEvent(self, event: 'QGraphicsSceneMouseEvent') -> None:
        self.mousePressEvent(event)
        return super().mouseDoubleClickEvent(event)

    def mousePressEvent(self, event: 'QGraphicsSceneMouseEvent'): 
        super().mousePressEvent(event)
        button = event.button()
        if button == Qt.RightButton:
            point = event.buttonDownPos(button)
            self.right_click_event(point)
        elif button == Qt.LeftButton:
            self.left_click_event()

    def right_click_event(self, point):
        self.window.change_mode()
        print("Mode", self.window.mode)
        x = point.x()
        y = point.y()
        print("Scene", x, y)
        cords = self.window.plot_window.pix_to_cords(x,y)
        print("Cords", cords)
        if self.window.julia_cords != cords:
            self.window.julia_valid = False
            self.window.julia_cords = cords
        self.window.load_image()

    def left_click_event(self):
        # maybe implement a drag function
        pass


def main():
    app = QApplication(sys.argv)
    main_window = MainWindow()
    main_window.showMaximized()
    sys.exit(app.exec_())


if __name__ == '__main__':
    main()