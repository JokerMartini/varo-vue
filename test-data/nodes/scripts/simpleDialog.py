################################################################################
# imports
################################################################################
import os
import sys
import inspect
from PySide2 import QtWidgets, QtGui


################################################################################
# widget
################################################################################
class MainWidget(QtWidgets.QWidget):

    def __init__(self, parent=None):
        super(MainWidget, self).__init__(parent=parent)
        # style.setStyle(self)

        # controls
        self.label = QtWidgets.QLabel('Environment Variables')
        self.text = QtWidgets.QTextEdit()
        self.text.setLineWrapMode(QtWidgets.QTextEdit.NoWrap)
        self.text.setReadOnly(True)

        msg = ''
        for k, v in sorted(os.environ.items()):
            msg += '{}={}\n'.format(k, v)
        self.text.setText(msg)

        # layout
        self.mainLayout = QtWidgets.QVBoxLayout()
        self.mainLayout.setContentsMargins(6,6,6,6)
        self.mainLayout.addWidget(self.label)
        self.mainLayout.addWidget(self.text)

        self.setLayout(self.mainLayout)
        

################################################################################
# main
################################################################################
def main():
    app = QtWidgets.QApplication(sys.argv)
    ex = MainWidget()
    ex.resize(600,600)
    ex.show()
    sys.exit(app.exec_())


if __name__ == '__main__':
    pass
    main()