import matplotlib.pyplot as plt
import sys

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
    if len(sys.argv) == 1:
        print("No name was given!")
        return
    csv = read_csv(f"./output/{sys.argv[1]}.csv")

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

    # Rotational velocity
    rot_vel_x = [float(line[5].split(',')[0]) for line in csv[1:]]
    rot_vel_y = [float(line[5].split(',')[1]) for line in csv[1:]]
    rot_vel_z = [float(line[5].split(',')[2]) for line in csv[1:]]

    # Sun
    sun_x = [float(line[6].split(',')[0]) for line in csv[1:]]
    sun_y = [float(line[6].split(',')[1]) for line in csv[1:]]
    sun_z = [float(line[6].split(',')[2]) for line in csv[1:]]

    # Charge
    charge = [float(line[7]) for line in csv[1:]]


    # Create plots
    fig, axs = plt.subplots(4,2)
    fig.suptitle(f"{sys.argv[1]}")

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
    axs[1, 0].plot(time, acc_x,'r')
    axs[1, 0].plot(time, acc_y,'g')
    axs[1, 0].plot(time, acc_z,'b')
    axs[1, 0].set_title('Acceleration')
    axs[1, 0].set(xlabel='Time [s]', ylabel='[m/s²]')
    axs[1, 0].legend(['x','y','z'])
    axs[1, 0].grid()

    # Plot rotation
    axs[1, 1].plot(time, rot_x,'r')
    axs[1, 1].plot(time, rot_y,'g')
    axs[1, 1].plot(time, rot_z,'b')
    axs[1, 1].set_title('Rotation')
    axs[1, 1].set(xlabel='Time [s]', ylabel='[deg]')
    axs[1, 1].legend(['x','y','z'])
    axs[1, 1].grid()

    # Plot rotational velocity
    axs[2, 0].plot(time, rot_vel_x,'r')
    axs[2, 0].plot(time, rot_vel_y,'g')
    axs[2, 0].plot(time, rot_vel_z,'b')
    axs[2, 0].set_title('Rot. vel.')
    axs[2, 0].set(xlabel='Time [s]', ylabel='[deg/s]')
    axs[2, 0].legend(['x','y','z'])
    axs[2, 0].grid()

    # Plot sun
    axs[2, 1].plot(time, sun_x,'r')
    axs[2, 1].plot(time, sun_y,'g')
    axs[2, 1].plot(time, sun_z,'b')
    axs[2, 1].set_title('Sun vector')
    axs[2, 1].set(xlabel='Time [s]')
    axs[2, 1].legend(['x','y','z'])
    axs[2, 1].grid()

    # Plot charge
    axs[3, 0].plot(time, charge)
    axs[3, 0].set_title('Charge')
    axs[3, 0].set(xlabel='Time [s]', ylabel='[Wh]')
    axs[3, 0].grid()

    # Plot
    plt.show()


if __name__ == '__main__':
    main()
