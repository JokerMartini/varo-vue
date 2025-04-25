import os
import sys


def main(args):
    # writes file to desktop with name of python file
    folder = os.path.join(os.path.join(os.path.expanduser('~')), 'Desktop')
    folder = os.path.expanduser('~/Documents')
    output = os.path.join(folder, 'python_sample.txt')

    print('OUTPUT >>> {}'.format(output))

    lines = ['python', os.path.basename(__file__)]
    lines.extend(args)

    with open(output, 'w') as f:
        for line in lines:
            f.write('{}\n'.format(line))

    if os.path.isfile(output):
        os.startfile(output)


if __name__ == '__main__':
    args = sys.argv[1:]
    print('ARGS >>> {}'.format(args))
    if len(args) >= 1:
        main(args)
    else:
        argsAlt = ['Manual test']
        main(argsAlt)
