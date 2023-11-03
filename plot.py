import matplotlib.pyplot as plt

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

    # Velocity
    vel_x = [float(line[2].split(',')[0]) for line in csv[1:]]
    vel_y = [float(line[2].split(',')[1]) for line in csv[1:]]
    vel_z = [float(line[2].split(',')[2]) for line in csv[1:]]

    # Acceleration
    acc_x = [float(line[3].split(',')[0]) for line in csv[1:]]
    acc_y = [float(line[3].split(',')[1]) for line in csv[1:]]
    acc_z = [float(line[3].split(',')[2]) for line in csv[1:]]

    # Rotation
    rot_x = [float(line[4].split(',')[0]) for line in csv[1:]]
    rot_y = [float(line[4].split(',')[1]) for line in csv[1:]]
    rot_z = [float(line[4].split(',')[2]) for line in csv[1:]]

    # Sun
    sun_x = [float(line[5].split(',')[0]) for line in csv[1:]]
    sun_y = [float(line[5].split(',')[1]) for line in csv[1:]]
    sun_z = [float(line[5].split(',')[2]) for line in csv[1:]]

    # Charge
    charge = [float(line[6]) for line in csv[1:]]


    # Create plots
    fig, axs = plt.subplots(2,3)

    # Plot position
    axs[0, 0].plot(time, pos_x,'r')
    axs[0, 0].plot(time, pos_y,'g')
    axs[0, 0].plot(time, pos_z,'b')
    axs[0, 0].set_title('Position')
    axs[0, 0].set(xlabel='Time [s]', ylabel='[m]')
    axs[0, 0].legend(['x','y','z'])
    axs[0, 0].grid()

    # Plot velocity
    axs[0, 1].plot(time, vel_x,'r')
    axs[0, 1].plot(time, vel_y,'g')
    axs[0, 1].plot(time, vel_z,'b')
    axs[0, 1].set_title('Velocity')
    axs[0, 1].set(xlabel='Time [s]', ylabel='[m/s]')
    axs[0, 1].legend(['x','y','z'])
    axs[0, 1].grid()

    # Plot acceleration
    axs[0, 2].plot(time, acc_x,'r')
    axs[0, 2].plot(time, acc_y,'g')
    axs[0, 2].plot(time, acc_z,'b')
    axs[0, 2].set_title('Acceleration')
    axs[0, 2].set(xlabel='Time [s]', ylabel='[m/s²]')
    axs[0, 2].legend(['x','y','z'])
    axs[0, 2].grid()

    # Plot rotation
    axs[1, 0].plot(time, rot_x,'r')
    axs[1, 0].plot(time, rot_y,'g')
    axs[1, 0].plot(time, rot_z,'b')
    axs[1, 0].set_title('Rotation')
    axs[1, 0].set(xlabel='Time [s]', ylabel='[deg]')
    axs[1, 0].legend(['x','y','z'])
    axs[1, 0].grid()

    # Plot sun
    axs[1, 1].plot(time, sun_x,'r')
    axs[1, 1].plot(time, sun_y,'g')
    axs[1, 1].plot(time, sun_z,'b')
    axs[1, 1].set_title('Sun vector')
    axs[1, 1].set(xlabel='Time [s]')
    axs[1, 1].legend(['x','y','z'])
    axs[1, 1].grid()

    # Plot charge
    axs[1, 2].plot(time, charge)
    axs[1, 2].set_title('Charge')
    axs[1, 2].set(xlabel='Time [s]', ylabel='[Wh]')
    axs[1, 2].grid()

    # Plot
    plt.show()


if __name__ == '__main__':
    main()
