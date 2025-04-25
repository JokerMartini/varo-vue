import os
import sys


def main():
    for k, v in sorted(os.environ.items()):
        print('{}={}'.format(k, v))


if __name__ == '__main__':
    main()