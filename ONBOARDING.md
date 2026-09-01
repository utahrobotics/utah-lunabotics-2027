# Onboarding Guide for New USR Members

Welcome to Utah Student Robotics! This guide will take you from zero to your first pull request. It consolidates every piece of setup information from across the project so you have a single source of truth.

> **How to read this guide**: Start at the top and work through each section in order. Not every section will apply to you—if you're a software-only contributor on macOS you can skip the hardware and Linux production sections. Use the table of contents to jump to what you need.

---

## Table of Contents

1. [Prerequisites & Background Knowledge](#1-prerequisites--background-knowledge)
2. [Choose Your Development Path](#2-choose-your-development-path)
3. [Environment Setup](#3-environment-setup)
   - [Windows](#windows)
   - [macOS](#macos)
   - [Linux](#linux)
4. [Clone & Build the Project](#4-clone--build-the-project)
5. [Run Your First Log Replay](#5-run-your-first-log-replay)
6. [Run the MuJoCo Simulation](#6-run-the-mujoco-simulation)
7. [Run the Lunabase (Godot)](#7-run-the-lunabase-godot)
8. [Understand the Architecture](#8-understand-the-architecture)
9. [Key Configuration Files](#9-key-configuration-files)
10. [IDE & Tooling Setup](#10-ide--tooling-setup)
11. [Git Workflow & Your First PR](#11-git-workflow--your-first-pr)
12. [Make Targets Cheatsheet](#12-make-targets-cheatsheet)
13. [Production Environment (Linux Only)](#13-production-environment-linux-only)
14. [Troubleshooting](#14-troubleshooting)
15. [Further Reading & Resources](#15-further-reading--resources)

---

## 1. Prerequisites & Background Knowledge

Before diving into the codebase, make sure you're comfortable with:

| Skill | Why You Need It | Where to Learn |
|---|---|---|
| **Git basics** | All code changes go through Git/GitHub | [Git Handbook](https://guides.github.com/introduction/git-handbook/) |
| **Terminal / command line** | Building, running, and debugging all happen here | [Linux Command Line Basics](https://ubuntu.com/tutorials/command-line-for-beginners) |
| **Rust fundamentals** | The robot software is written in Rust | [The Rust Book](https://doc.rust-lang.org/book/) |

**Optional but helpful:**
- Familiarity with [Copper (cu29)](https://github.com/copper-project/copper-rs) — the real-time robotics framework we use
- Basic understanding of robotics concepts (sensors, actuators, point clouds)
- GDScript / Godot experience if you plan to work on the lunabase UI

> [!TIP]
> You do **not** need to be a Rust expert to contribute. Many tasks involve configuration, Godot UI work, MuJoCo simulation models, or Python calibration scripts.

---

## 2. Choose Your Development Path

Not everyone needs to install everything. Pick the path that matches what you'll be working on:

| Path | OS Support | What You'll Do | Setup Sections |
|---|---|---|---|
| **Log Replay** | Windows, macOS, Linux | Replay recorded sensor data and visualize it in Rerun | [§3](#3-environment-setup), [§4](#4-clone--build-the-project), [§5](#5-run-your-first-log-replay) |
| **MuJoCo Simulation** | Windows, macOS, Linux | Run the robot in a physics simulator | [§3](#3-environment-setup), [§4](#4-clone--build-the-project), [§6](#6-run-the-mujoco-simulation) |
| **Lunabase (Godot UI)** | Windows, macOS, Linux | Work on the base station control panel | [§3](#3-environment-setup), [§7](#7-run-the-lunabase-godot) |
| **Production / Hardware** | Linux only | Deploy to real hardware | [§3](#3-environment-setup), [§13](#13-production-environment-linux-only) |
| **Embedded Firmware** | Any (with debug probe) | Program the Raspberry Pi Picos | See [embedded/README.md](embedded/README.md) |

---

## 3. Environment Setup

Follow the instructions for **your operating system** below. Each section installs everything needed for log replay (the recommended starting point).

---

### Windows

#### 3.1 — Git

Download the standalone installer from [git-scm.com](https://git-scm.com/downloads/win). After installation, configure Git with your GitHub credentials using either a **personal access token** or **SSH keys**.

```powershell
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

#### 3.2 — Package Manager: Chocolatey

Install [Chocolatey](https://chocolatey.org/install) using the individual installation instructions (a single PowerShell command). This makes installing other tools much easier.

#### 3.3 — Rust

Install from [rustup.rs](https://rustup.rs). Open a **new** PowerShell window after installation and verify:

```powershell
rustc --version
```

> [!NOTE]
> The project pins a specific nightly toolchain via `rust-toolchain.toml` (`nightly-2026-01-18`). When you build inside the repo, `rustup` will automatically download and use the correct version. You do **not** need to manually run `rustup default nightly`.

#### 3.4 — `make`

```powershell
# Run in an admin PowerShell
choco install make
```

Open a new PowerShell and verify: `make --version`

#### 3.5 — LLVM / Clang

```powershell
# Run in an admin PowerShell
choco install llvm
```

Open a new PowerShell and verify: `clang --version`

#### 3.6 — C++ Build Tools

1. Download the installer from [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/?q=build+tools).
2. Scroll to the bottom → expand **Tools for Visual Studio** → download **Build Tools for Visual Studio**.
3. In the installer window, select:
   - **Desktop development with C++**
   - **macOS/Linux Development with C++**
4. Click **Install** and wait for it to finish.

> [!NOTE]
> You do **not** need Visual Studio itself—only these build tools.

#### 3.7 — Rerun (Visualization)

1. Download the latest stable Windows binary from [Rerun's GitHub Releases](https://github.com/rerun-io/rerun/releases) (e.g. `rerun-cli-0.25.1-x86_64-pc-windows-msvc.exe`).
2. Rename it to `rerun.exe`.
3. Place it in a permanent directory (e.g. `C:\Program Files\Rerun`).
4. Add that directory to your system `PATH`:
   - Press **Win + X** → **System** → **Advanced system settings** → **Environment Variables**
   - Under **System variables**, select **Path** → **Edit** → **New** → add the directory
5. Open a new PowerShell and verify: `rerun --version`

#### 3.8 — 7-Zip ZS (for log files)

Standard 7zip will **not** work with `.tar.lz4` log files. Download the installer from [7-Zip-zstd releases](https://github.com/mcmilk/7-Zip-zstd/releases).

---

### macOS

#### 3.1 — Xcode Command Line Tools

This installs `git`, `make`, `clang`, and other essentials in one go:

```bash
xcode-select --install
```

#### 3.2 — Homebrew

Install from [brew.sh](https://brew.sh). Homebrew is the de facto macOS package manager and will be helpful for installing other tools.

#### 3.3 — Git Configuration

Git is pre-installed on macOS. Set it up with your GitHub credentials:

```bash
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

Configure either a **[personal access token](https://github.com/settings/tokens)** or **[SSH keys](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account)** for GitHub authentication.

#### 3.4 — Rust

Install from [rustup.rs](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

> [!NOTE]
> The project pins a specific nightly toolchain via `rust-toolchain.toml`. When you build inside the repo, `rustup` will automatically download and use the correct version. If you run into issues, you may need to manually run `rustup default nightly`.

#### 3.5 — Rerun (Visualization)

```bash
# Download the correct binary for your Mac
# Apple Silicon:
curl -L -o rerun https://github.com/rerun-io/rerun/releases/download/0.24.1/rerun-cli-0.24.1-aarch64-apple-darwin
# Intel:
# curl -L -o rerun https://github.com/rerun-io/rerun/releases/download/0.24.1/rerun-cli-0.24.1-x86_64-apple-darwin

chmod +x rerun
mkdir -p ~/.local/bin
mv rerun ~/.local/bin/

# Add to PATH (zsh)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Verify
which rerun
```

> [!TIP]
> Check [Rerun's releases page](https://github.com/rerun-io/rerun/releases) for the latest version—the URLs above may be outdated.

---

### Linux

#### 3.1 — Git

Pre-installed on most distros. Configure it:

```bash
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

Set up either a **[personal access token](https://github.com/settings/tokens)** or **[SSH keys](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account)** for GitHub.

#### 3.2 — `make` and Build Essentials

```bash
# Ubuntu / Debian
sudo apt install build-essential

# Fedora
sudo dnf groupinstall "Development Tools"
```

#### 3.3 — clang

```bash
# Ubuntu / Debian
sudo apt install libclang-dev

# Fedora
sudo dnf install clang-devel
```

#### 3.4 — Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

> [!NOTE]
> The project pins a specific nightly toolchain via `rust-toolchain.toml` (`nightly-2026-01-18`). When you build inside the repo, `rustup` will automatically download and use the correct version. You do **not** need to manually run `rustup default nightly`.

#### 3.5 — Rerun (Visualization)

Download the latest Linux binary from [Rerun's GitHub Releases](https://github.com/rerun-io/rerun/releases). Make it executable and add it to your PATH:

```bash
chmod +x rerun-cli-*
sudo mv rerun-cli-* /usr/local/bin/rerun
rerun --version
```

---

## 4. Clone & Build the Project

```bash
# Clone via SSH (recommended)
git clone git@github.com:utahrobotics/utah-lunabotics-2027.git
cd utah-lunabotics-2027

# Or clone via HTTPS
git clone https://github.com/utahrobotics/utah-lunabotics-2027.git
cd utah-lunabotics-2027
```

Run an initial check build to make sure your environment is working:

```bash
make check-resim
```

> [!NOTE]
> **Stack size**: On some machines, the Rust compiler needs a larger stack. If you see a stack overflow during compilation, run:
> ```bash
> export RUST_MIN_STACK=107108864
> ```
> Then retry the build.

The first build will take several minutes as it downloads and compiles all dependencies. Subsequent builds are incremental and much faster.

---

## 5. Run Your First Log Replay

Log replay is the **easiest way to start contributing** — it lets you replay recorded sensor data from the real robot and visualize it, without needing any hardware or simulation setup.

### 5.1 — Download a Log File

1. Go to the `#lunabot-logs` channel in Discord.
2. Download a log file (`.tar.lz4` format).
3. Decompress it:

**Linux / macOS:**
```bash
lz4 -d <filename>.tar.lz4
tar xvf <filename>.tar
```

**Windows:**
Use 7-Zip ZS to extract (you'll need to extract twice — once for `.lz4`, once for `.tar`).

4. Place the extracted log files in `lunabot-cu/logs/`.

> [!IMPORTANT]
> Make sure there are **no nested directories** inside `lunabot-cu/logs/` — the log files should be placed directly in that folder.

> [!WARNING]
> Logs must be compatible with your current code. Each log post in Discord specifies the commit hash it was recorded with. If you see strange deserialization errors, make sure you're on the matching commit: `git checkout <commit-hash>`.

### 5.2 — Run the Replay

```bash
make resim
```

This builds the project and launches the replay in Rerun's GUI. The first build takes a few minutes.

🎉 **Congratulations!** You should now see sensor data playing back in the Rerun visualizer.

---

## 6. Run the MuJoCo Simulation

The MuJoCo simulation lets you run the full robot software in a physics-simulated environment. This is invaluable for testing autonomy, motor control, and actuator logic without the physical robot.

### 6.1 — Install MuJoCo for Rust (Static Linking)

Follow the installation instructions at [mujoco-rs static linking guide](https://mujoco-rs.readthedocs.io/en/v2.0.x/installation.html#static-linking) to build MuJoCo for static linking.

> [!TIP]
> You may need to leave off the `--release` flag on the `make` step. You may also need to use [this fork](https://github.com/matthewashton-k/mujoco-rs) of mujoco-rs.

### 6.2 — Set Environment Variables

Point the build system to your MuJoCo installation:

```bash
export MUJOCO_STATIC_LINK_DIR=/path/to/mujoco-rs/mujoco/build/lib
```

### 6.3 — Run the Simulation

```bash
# Run with UCF arena
make sim SIM_ARENA=ucf

# Or with Artemis arena
make sim SIM_ARENA=artemis
```

> [!TIP]
> If you disable task logging the sim will run smoother.

### 6.4 — Standalone Viewer (No Robot Code)

If you just want to preview the simulation scene and iterate on the MuJoCo XML models without running the robot code:

1. Download MuJoCo from [github.com/google-deepmind/mujoco/releases](https://github.com/google-deepmind/mujoco/releases).
2. Run the viewer:
   ```bash
   /path/to/mujoco/bin/simulate mujoco-sim/artemis_arena.xml
   ```

This supports hot-reloading — edit XML files and click **Reload** in the viewer UI.

For more details on adding meshes, tooling (obj2mjcf), and learning resources, see [mujoco-sim/README.md](mujoco-sim/README.md).

---

## 7. Run the Lunabase (Godot)

The lunabase is the base station GUI used to control and monitor the robot. It's built with Godot and uses a Rust GDExtension library.

### 7.1 — Install Godot

Download and install **Godot 4.6** (stable) from [godotengine.org](https://godotengine.org/).

### 7.2 — Build the GDExtension Library

```bash
cd lunabase-lib
cargo build
cd ..
```

> [!IMPORTANT]
> You need to rebuild this library (`cargo build` in `lunabase-lib/`) every time the Rust code in `lunabase-lib/src/lib.rs` changes.

### 7.3 — Open and Run the Project

```bash
# Or use the make shortcut (builds + opens):
make edit-lunabase
```

In the Godot editor, click one of the play buttons in the top-right corner to launch the `MainControl.tscn` scene.

### 7.4 — Connecting to the Robot (or Simulation)

1. Type the IP address of the robot (or `127.0.0.1` for local simulation) in the upper-left corner of the lunabase.
2. Press **Connect**.
3. Select your control mode (Manual, Autonomy, etc.).

For Godot contribution guidelines, see [godot/README_GODOT.md](godot/README_GODOT.md).

---

## 8. Understand the Architecture

Before writing code, take some time to understand how the system fits together.

### High-Level Architecture

The robot software is built on [Copper (cu29)](https://github.com/copper-project/copper-rs), a real-time robotics framework. Copper organizes the system as a **directed acyclic graph (DAG) of tasks** that pass typed messages between each other.

> Check `copperconfig.ron` to see the definitions of all tasks and the datatypes passed between them.

### System Entry Points

| Entry Point | File | Purpose |
|---|---|---|
| **Production** | `lunabot-cu/src/main.rs` | Launches external processes, sets up Rerun, builds and runs the robot |
| **Log Replay** | `lunabot-cu/src/resim.rs` | Replays recorded copper logs with selective task mocking |
| **Simulation** | `lunabot-cu/src/sim.rs` | Launches MuJoCo viewer, simulates sensor inputs |

### Crate Layout

| Crate / Directory | What It Contains |
|---|---|
| `lunabot-cu/` | Main robot application — tasks, bridges, utilities |
| `lunabot-cu/src/tasks/sources/` | Sensor input tasks (RealSense, T265, udev monitor) |
| `lunabot-cu/src/tasks/sinks/` | Output tasks (VESC motor control, null sinks) |
| `lunabot-cu/src/tasks/ai/` | Behavior tree-based autonomy ([details](lunabot-cu/src/tasks/ai/README.md)) |
| `lunabot-cu/src/tasks/` | Processing tasks (AprilTag detection, KISS-ICP, occupancy grid, etc.) |
| `lunabot-cu/src/bridges/` | Communication with the lunabase |
| `lunabot-cu/src/comms/` | Network connection helpers |
| `lunabot-cu/src/utils/` | Helpers (framed codec, udev polling, unit conversion) |
| `common/` | Types shared between lunabase and lunabot |
| `embedded_common/` | `no_std` types shared with embedded firmware |
| `embedded/` | Firmware for the 2026 robot's Picos |
| `embedded-legacy/` | Firmware for the previous robot (TERI) |
| `external-tasks/realsense/` | External process for RealSense depth cameras |
| `lunabase-lib/` | Rust GDExtension library for the Godot lunabase |
| `godot/` | Godot project for the base station UI |
| `mujoco-sim/` | MuJoCo simulation scene files and meshes |
| `misc/` | Utility libraries — kinematics, networking, GPU shaders, VESC, camera discovery, calibration |
| `robot-layout/` | Physical robot sensor layout definitions (`lunabot.ron`) |

### AI / Behavior Tree

The autonomy system uses a behavior tree with three top-level modes:

1. **Manual** — Motors/actuators mirror lunabase commands
2. **Autonomy** — Navigate, dig, and dump behaviors
3. **Software Stop** — Zero all motor speeds (safe fallback)

For a deep dive, see [lunabot-cu/src/tasks/ai/README.md](lunabot-cu/src/tasks/ai/README.md).

---

## 9. Key Configuration Files

| File | Purpose |
|---|---|
| `copperconfig.ron` | Defines all Copper tasks, their configs, and how they're wired together |
| `robot-layout/lunabot.ron` | Physical positions of sensors relative to the robot center |
| `rust-toolchain.toml` | Pins the exact Rust nightly version (`nightly-2026-01-18`) |
| `Cargo.toml` (workspace) | Workspace member definitions and shared dependencies |
| `.cargo/config.toml` | Cargo build settings (linker flags, etc.) |
| `Makefile` | Build/run shortcuts — run `make help` for the full list |
| `Dockerfile` | Docker container for development (fallback option) |
| `.devcontainer/devcontainer.json` | VS Code Dev Container configuration |

> [!TIP]
> Run `make validate-config` to check your `copperconfig.ron` for errors with friendlier messages than the compile-time panics. Run `make visualize-config` to generate a graph of the task DAG as `graph.svg`.

---

## 10. IDE & Tooling Setup

### Recommended IDE: VS Code

Download from [code.visualstudio.com](https://code.visualstudio.com).

**Recommended Extensions:**
| Extension | Purpose |
|---|---|
| `rust-lang.rust-analyzer` | Rust language support (essential) |
| `tamasfe.even-better-toml` | TOML file support |

**Rust Analyzer Configuration:**

To get proper IDE support for log replay code, set the feature flag in your VS Code settings (`.vscode/settings.json`):

```json
{
    "rust-analyzer.cargo.features": ["resim"]
}
```

> [!NOTE]
> You may want to change this to `["sim"]` or `["production"]` depending on what you're currently working on, since different entry points are gated behind different feature flags.

### Alternative: Dev Container

The project includes a Dev Container configuration (`.devcontainer/devcontainer.json`). If you use VS Code with the Remote - Containers extension, you can develop inside a pre-configured Docker container with all dependencies installed.

### Optional Tools

| Tool | Purpose | Install |
|---|---|---|
| **cubuild** | Enhanced error messages for Copper macros | [copper-rs/support/cargo_cubuild](https://github.com/copper-project/copper-rs/tree/master/support/cargo_cubuild) |
| **cargo samply** | CPU profiling via flamegraphs | `cargo install cargo-samply` |
| **lz4** | Compress / decompress log files | Package manager (`apt`, `brew`, `choco`) |
| **gdb** | Debugger (Linux) | Package manager |
| **obj2mjcf** | Convert .obj meshes to MuJoCo format | See [mujoco-sim/README.md](mujoco-sim/README.md) |

---

## 11. Git Workflow & Your First PR

> [!NOTE]
> For detailed contribution guidelines (branch naming, commit messages, code review process), see [CONTRIBUTING.md](CONTRIBUTING.md).

### Step-by-Step: Your First Pull Request

1. **Create a branch** off `main`:
   ```bash
   git checkout main
   git pull origin main
   git checkout -b your-name/short-description
   ```

2. **Make your changes** — edit code, add features, fix bugs.

3. **Verify your changes build**:
   ```bash
   # For log replay work
   make check-resim

   # For production work
   make check-prod
   ```

4. **Test your changes**:
   - For log replay changes: `make resim`
   - For simulation changes: `make sim SIM_ARENA=ucf`
   - For lunabase changes: `make edit-lunabase`

5. **Commit and push**:
   ```bash
   git add .
   git commit -m "Brief description of your change"
   git push origin your-name/short-description
   ```

6. **Open a Pull Request** on GitHub:
   - Go to the repository page on GitHub
   - Click **Compare & pull request**
   - Fill in a description of what you changed and why
   - Request review from the relevant [CODEOWNERS](CODEOWNERS)

7. **Respond to review feedback**, push fixes, and merge once approved!

> [!TIP]
> Start small — fix a typo, improve documentation, or tackle an issue labeled `good-first-issue` in the issue tracker.

---

## 12. Make Targets Cheatsheet

Run `make help` for the full list. Here are the most common targets:

| Command | What It Does |
|---|---|
| `make resim` | Build and run log replay (opens Rerun) |
| `make sim SIM_ARENA=ucf` | Build and run MuJoCo simulation |
| `make prod` | Build and run for production hardware (Linux only) |
| `make check-resim` | Compile-check the log replay binary |
| `make check-prod` | Compile-check the production binary |
| `make validate-config` | Validate `copperconfig.ron` with friendly errors |
| `make visualize-config` | Generate a task DAG graph (`graph.svg`) |
| `make build-lunabase` | Build the Godot GDExtension library |
| `make edit-lunabase` | Build library + open Godot editor |
| `make discover-cameras` | Run camera discovery tool (Linux only) |
| `make kill` | Kill all lunabot sub-processes |
| `make clear-logs` | Remove all copper unified logs |
| `make clear-simlogs` | Remove only simulation logs |
| `make clear-iceoryx2` | Clean up iceoryx2 artifacts |
| `make help` | Show all available targets |

**Profiling:** Add `PERF=true` to profile with `cargo-samply`:
```bash
make sim PERF=true SIM_ARENA=ucf
make prod PERF=true
```

---

## 13. Production Environment (Linux Only)

This section is for deploying to the actual robot hardware. **You do not need this for regular software development.**

### 13.1 — Additional Dependencies

Install the production system libraries:

```bash
sudo apt install \
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
  libudev-dev
```

Additional production dependencies:
- [Bazelisk](http://github.com/bazelbuild/bazelisk/releases/) — for building the unilidar publisher
- [RealSense SDK](https://github.com/IntelRealSense/librealsense/blob/master/doc/installation.md#building-librealsense2-sdk)
- [AprilTag Library](https://github.com/AprilRobotics/apriltag)

### 13.2 — Permissions

> [!IMPORTANT]
> These are **required** on the robot's PC:

1. Add your user to the following groups: **dialout**, **render**, **adm**
2. Install the [RealSense udev rules](https://github.com/realsenseai/librealsense/blob/master/config/99-realsense-libusb.rules) to your system's udev rules directory
3. Install the USB reset rules from `misc/usb-reset/99-usb-reset-rules.rules`

### 13.3 — Build & Run

```bash
# Sync Bazel dependencies (first time, or when deps change)
make sync

# Build and run in production mode
make prod
```

### 13.4 — Hardware Setup Quick Reference

For detailed sensor setup, camera configuration, VESC motor controllers, actuator flashing, and network configuration, see [FIRST_SETUP.md](FIRST_SETUP.md). That file is the hardware team's reference for:

- T265/T261 tracking cameras
- D456 depth cameras
- RGB cameras and port discovery
- VESC motor controllers
- Pico actuator flashing (prime and secondary)
- Network and SSH configuration
- Camera streaming via the multiplexed camera viewer

---

## 14. Troubleshooting

### Build Issues

| Problem | Solution |
|---|---|
| Stack overflow during compilation | `export RUST_MIN_STACK=107108864` and retry |
| Weird compile errors after pulling | Run `git pull` then `cargo update` |
| `copperconfig.ron` parse errors | Run `make validate-config` for better error messages |
| Copper config "index out of bounds" | Ensure all tasks have inputs/outputs, and all connections are defined — you cannot have a task with an output that isn't connected to another task's input |

### Iceoryx2 Errors

If you see errors related to iceoryx node or service creation (stale artifacts from crashed processes):

```bash
make clear-iceoryx2
# Or manually:
rm -r /tmp/iceoryx2
rm -r /dev/shm/iox2_*
rm -rf /dev/shm/*iceoryx*
```

### Rerun Issues

| Problem | Solution |
|---|---|
| Rerun window doesn't open | Check that `rerun` is in your PATH (`which rerun` or `rerun --version`) |
| Rerun won't spawn on Windows | Start Rerun manually in a separate terminal, then run `make sim/resim/prod` — it will connect automatically |
| Protobuf decode errors | Ensure the Rerun SDK version in `Cargo.toml` matches your installed `rerun --version` |

### Log Replay Issues

| Problem | Solution |
|---|---|
| "No such file or directory" | Check that valid log files exist in `lunabot-cu/logs/` with no nested directories |
| `InvalidIntegerType { expected: I32, found: I64 }` | Logs are incompatible with your current code — checkout the commit hash listed in the Discord log post |
| Other deserialization errors | Same as above — logs and code must be from the same commit |

### Background Tasks

There is a performance optimization that prevents async task `process` functions from being called unless there is an input ready. In the rare case where your async task needs to do work without an input, this can cause unexpected behavior.

For the full troubleshooting reference, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md).

---

## 15. Further Reading & Resources

### Project Documentation
- [README.md](README.md) — Project overview and crate layout
- [FIRST_SETUP.md](FIRST_SETUP.md) — Hardware setup reference
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) — Common issues and fixes
- [CONTRIBUTING.md](CONTRIBUTING.md) — Contribution guidelines (branch naming, code review, etc.)
- [godot/README_GODOT.md](godot/README_GODOT.md) — Godot contribution guidelines
- [mujoco-sim/README.md](mujoco-sim/README.md) — MuJoCo simulation setup and development
- [lunabot-cu/src/tasks/ai/README.md](lunabot-cu/src/tasks/ai/README.md) — Behavior tree and autonomy deep dive
- [embedded/README.md](embedded/README.md) — Embedded firmware system layout and flashing

### External Resources
- [Copper (cu29) Framework](https://github.com/copper-project/copper-rs)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rerun Visualization](https://github.com/rerun-io/rerun)
- [MuJoCo Documentation](https://mujoco.readthedocs.io/en/3.3.7/overview.html)
- [MuJoCo XML Reference](https://mujoco.readthedocs.io/en/3.3.7/XMLreference.html)
- [mujoco-rs API Docs](https://docs.rs/mujoco-rs/latest/mujoco_rs/)
- [Godot GDScript Style Guide](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_styleguide.html)
- [nav2 Behavior Trees](https://docs.nav2.org/behavior_trees/overview/nav2_specific_nodes.html) — Inspiration for our AI helper nodes

### Discord

Join the team Discord (link from your team lead) for:
- Log files in `#lunabot-logs`
- WiFi passwords and robot IPs in pinned admin messages
- Quick help and troubleshooting

---

> [!TIP]
> **Stuck?** Don't spend hours debugging alone. Post in the Discord, check [TROUBLESHOOTING.md](TROUBLESHOOTING.md), or pair up with a teammate. We've all been there. Welcome to the team! 🚀
