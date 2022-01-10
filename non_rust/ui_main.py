from main_window import *
from PyQt5 import QtCore
from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QApplication, QMainWindow
from PyQt5.QtGui import QPixmap

import sys
from enum import Enum, auto

from mandelbrot_module import PlotWindow


class Modes(Enum):
    Mandelbrot = auto()
    Julia = auto()


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.ui = Ui_MainWindow()
        self.ui.setupUi(self)

        self.grabKeyboard()

        # params
        self.scale = 120    # 120 = (1920, 1080), 80 = (1280, 720) ...
        self.tries = 10000
        self.power = 1
        self.zoom_factor = 2

        # render Items
        self.mandel_scene = None
        self.mandel_item = None
        self.mandel_valid = False

        self.julia_scene = None
        self.julia_item = None
        self.julia_valid = False

        # maps pixels to 2D coordinates
        self.plot_window = self.get_new_plot_window()
        
        # render first image
        self.mode = Modes.Mandelbrot
        self.mandel_path = None
        self.julia_path = None
        self.load_image()

    def get_new_plot_window(self):
        return PlotWindow((16 * self.scale, 9 * self.scale))

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
            self.mandel_path = self.plot_window.load_mandelbrot(self.tries, self.power)
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
            self.julia_path = self.plot_window.load_julia(self.tries, self.power)
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
        elif self.mode == Modes.Julia:
            self.mode = Modes.Mandelbrot
        self.load_image()

    def invalidate_all(self):
        self.mandel_valid = False
        self.julia_valid = False

    def invalidate(self):
        if self.mode == Modes.Mandelbrot:
            self.mandel_valid = False
        elif self.mode == Modes.Julia:
            self.julia_valid = False


    def keyPressEvent(self, a0: QtGui.QKeyEvent) -> None:
        super_result = super().keyPressEvent(a0)
        # reset view
        self.plot_window.reset_view()
        self.invalidate_all()
        self.load_image() 
        return super_result
        

class ResponsiveScene(QtWidgets.QGraphicsScene):
    def __init__(self, window: MainWindow):
        self.window = window
        super().__init__()

    def wheelEvent(self, event: 'QGraphicsSceneWheelEvent') -> None:
        super().wheelEvent(event)
        rotation = event.delta()
        point = event.scenePos()
        p = (point.x(), point.y())
        print("p", p)
        if rotation > 0:
            print("wheel forward")
            self.window.plot_window.zoom(p, 1 / self.window.zoom_factor)
        elif rotation < 0:
            print("wheel backward")
            self.window.plot_window.zoom(p, self.window.zoom_factor)
        print(self.window.plot_window)
        self.window.invalidate_all()
        self.window.load_image()

    def mouseDoubleClickEvent(self, event: 'QGraphicsSceneMouseEvent') -> None:
        super().mouseDoubleClickEvent(event)
        self.mousePressEvent(event)

    def mousePressEvent(self, event: 'QGraphicsSceneMouseEvent'): 
        super_result = super().mousePressEvent(event)
        button = event.button()
        point = event.buttonDownPos(button)
        if button == Qt.RightButton:
            self.right_click_event(point)
        elif button == Qt.LeftButton:
            self.left_click_event(point)
        return super_result

    def right_click_event(self, point):
        x = point.x()
        y = point.y()
        print("Scene", x, y)
        cords = (x,y)
        self.window.julia_valid = False
        self.window.plot_window.set_julia(cords)
        self.window.change_mode()
        print("Mode", self.window.mode)

    def left_click_event(self, point):
        x = point.x()
        y = point.y()
        print("Scene", x, y)
        cords = (x,y)
        self.window.plot_window.move_view(cords)
        self.window.invalidate()
        self.window.load_image()


def main():
    app = QApplication(sys.argv)
    main_window = MainWindow()
    main_window.showMaximized()
    sys.exit(app.exec_())


if __name__ == '__main__':
    main()