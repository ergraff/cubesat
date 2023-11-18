import matplotlib.pyplot as plt
import numpy as np

def read_csv(file: str):
    with open(file, 'r') as f:
        # Read and separate to lines
        csv = f.read().split('\n')
        # Split by delimiter '|'
        csv = [line.split('|') for line in csv]
        # Delete empty last time
        del csv[-1]

        return csv


def main():
    csv = read_csv("history.csv")

    # Time
    time = [float(line[0]) for line in csv[1:]]

    # Position
    pos_x = [float(line[1].split(',')[0]) for line in csv[1:]]
    pos_y = [float(line[1].split(',')[1]) for line in csv[1:]]
    pos_z = [float(line[1].split(',')[2]) for line in csv[1:]]

    # Charge
    charge = [float(line[8]) for line in csv[1:]]

    # Earth
    r = 6.3781*10**6
    u = np.linspace(0, 2 * np.pi, 20)
    v = np.linspace(0, np.pi, 20)
    x = r * np.outer(np.cos(u), np.sin(v))
    y = r * np.outer(np.sin(u), np.sin(v))
    z = r * np.outer(np.ones(np.size(u)), np.cos(v))

    # Create plots
    ax = plt.axes(projection="3d")
    ax.plot_surface(x,y,z, zorder=0)
    ax.plot(pos_x, pos_y, pos_z, zorder=5)
    ax.set(xlabel='x [m]', ylabel='y [m]', zlabel='z [m]')
    ax.legend(['Earth','Position'])
    ax.set_aspect('equal')
    plt.show()


if __name__ == '__main__':
    main()
