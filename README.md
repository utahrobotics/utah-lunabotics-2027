# Lunabot

## Guide to start up the robot is in [FIRST_SETUP.md](FIRST_SETUP.md)

## Architecture Overview
<img width="7916" height="1812" alt="image" src="https://github.com/user-attachments/assets/2a677af1-867d-446d-bd8f-a53078787d23" />

##### Check copperconfig.ron to see the definitions of all the tasks running and the datatypes passed between tasks.

## Dependencies

### Production env (Linux only)

1. ```
   apt/dnf/yum/etc install \
    pkg-config \
    libssl-dev \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav \
    libacl1-dev \
    libgstrtspserver-1.0-dev \
    libges-1.0-dev \
    libv4l-dev \
    libunwind-dev \
    libudev-dev \
   ```
2. [Bazelisk](http://github.com/bazelbuild/bazelisk/releases/)
3. [Realsense SDK](https://github.com/IntelRealSense/librealsense/blob/master/doc/installation.md#building-librealsense2-sdk)
4. [Apriltag Library](https://github.com/AprilRobotics/apriltag)


### Simulation Environment 
See [mujoco-sim/README.md](mujoco-sim/README.md) for dependencies and instructions.

### Optional Dependencies

1. **cubuild** - Enhanced error messages for Copper macros
   <https://github.com/copper-project/copper-rs/tree/master/support/cargo_cubuild>
2. iox2 cli tool for seeing active iceoryx2 nodes and services.
3. cargo flamegraph + perf for profiling
4. gdb
5. lz4 for compressing logs


## Building and Running

See [ONBOARDING.md](ONBOARDING.md) for more details on how to get your local development environment and log replay set up. 

*NOTE: On some machines you have to increase the stack size for it to compile ```export RUST_MIN_STACK=107108864```.*

### Production env (Linux only)
1. Install dependencies listed in the above ```Dependencies``` section for the production environment.
2. run ```make sync``` to build/sync deps for the Unitree L2 publisher.
3. run ```make prod``` to build and run the project.

## Camera discovery

- monitors udev events and allows for easy discovery of which cameras are on which ports.

```bash
make discover-cameras
```

## Profiling

1. Install [samply](https://crates.io/crates/cargo-samply) 
2. run ```make sim PERF=true``` or ```make prod PERF=true``` to profile. 

# Trouble Shooting
List of common problems and how to fix them can be found [here](https://github.com/utahrobotics/utah-lunabotics-2026/blob/main/TROUBLESHOOTING.md)

# Using the Web Panel
1. Connect to the same wifi network as the lunabot.
2. Navigate to 192.168.0.103
3. Click "Start Lunabot"
4. Ensure that the lunabase software is running on your computer, and the ip in the top left corner is correct, then press connect.

# Crate Layout

## Entry points
### lunabot-cu/src/main.rs
* Launches exernal processes.
* Sets up rerun
* Serializes the robot chain. 
* Builds and runs the lunabot application.

### lunabot-cu/src/resim.rs
* Reads in copperlist logs from lunabot-cu/logs. These logs contain messages passed between copper tasks.
* Runs the lunabot in simulation mode which allows you to selectively decide which task's process functions are simulated "Mocked", and which tasks process functions are not.
* Allows you to set a task with a mocked process function to output to whatever was read in from the logs, effectively allowing for deterministic replay of whatever was captured.

### lunabot-cu/src/sim.rs
* Launches mujoco cpp-viewer, and starts the lunabot.
* Simulates some sensor inputs, and controls the motor nodes defined in the simulation

### external-tasks/realsense
* Launched by lunabot-cu. 
* Detects when a realsense device is plugged in, automatically opens device and publishes point clouds.
* Not compiled or launched in log replay mode.

### unilidar_iceoryx_publisher (deprecated)
* Launched by lunabot-cu
* Connects to L2 and publishes imu data and pointclouds from it.
* Not compiled or executed in log replay mode.

## misc/ 
Contains libraries for kinematics, network protocols, GPU utilites/Shader pipelines, camera auto discovery, and interaction with VESC boards.

## lunabot-cu/src/bridges/
Contains task for communicating with the lunabase.

## lunabot-cu/src/tasks/sources
* L2 imu rx (deprecated)
* l2 pointcloud rx (deprecated)
* realsense subscriber - gets frames from any d4xx series
* t265 subscriber - gets images, 6dof, and imu messages from the plugged in t265/1 devices
* udev monitor - feeds v4l2 usb add events to gstreamer piplines

## lunabot-cu/src/tasks/sinks
* Sink tasks for interacting with the vescs
* Null sink for images we dont care about.

## lunabot-cu/src/tasks/ai
* Takes in readings from the lunabase. 
* Keeps track of what the robot is currently doing (see LunabotAction enum)
* Keeps track of state (the blackboard) associated with the lunabot: robot chain, last messages seen from lunabase, latest obstacle map, (and more to come)
* Decides how to control the motors and actuators based on all that information.

## lunabot-cu/src/tasks (not the ones in sources/ or sinks/)
Tasks that lie between the sources and sinks for:
* Image processing for apriltag detection.
* Automatically opening camera devices as they become available.
* Point cloud processing (KISS-ICP)
* localizer
* gstreamer pipelines for rgb cameras
* occupancy grid generation
* interface for the rpi pico.

## lunabot-cu/src/utils
Helper functions for:
* Framed codec for talking with rp2040
* Udev polling
* Linear interpolation
* Converting between units/types.

## lunabot-cu/src/comms
* Structures and helpers used for connecting to the base station.

## common/ 
- contains types shared between the base station and lunabot

## embedded_common
A no_std crate containing structures used by the embedded code as well as our tasks. 

## embedded-legacy 
- embedded code for last years robot TERI, kept for backwards compatibility

## Embedded
- embedded code for this years robot (2026)

## Others

### lunabot-cu/src/rerun_viz.rs
Utilities for connecting to rerun.


### lunabot-cu/src/motors.rs
Legacy code for controlling motors via VESC. <br>
The enumerate_motors() function (used by the motor_ctrl task) returns a structure that you can use to command the motors.

### lunabot-cu/src/simple_monitor.rs
* Hooks into the copper runtime and prints messages when a task's process, preprocess, etc return Err.
* Sets the list of errored tasks, which are then read by the lunabase and sent to the base station.

Example output:
```
=== ERRORED TASKS ===
Task 10: lunabase (State: Process) - lunabase not connected
   context:lunabase disconnected
Task 17: cam_side (State: Process) - no frames received
   context:no frames received
Task 14: cam_back (State: Process) - no frames received
   context:no frames received
Task 4: realsense_pointcloud (State: Process) - No points seen in 600 ms
   context:No points seen in 600 ms
Task 5: realsense_occupancy (State: Process) - No occupancy grid seen in 600 ms
   context:No occupancy grid seen in 600 ms
Task 2: l2_pointcloud (State: Process) - No points seen in 600 ms
   context:No points seen in 600 ms
=====================
```
