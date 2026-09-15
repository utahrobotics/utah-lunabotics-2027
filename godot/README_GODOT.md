# Starting Work in Lunabase

Lunabase is the Godot based UI for the rover. It uses a Rust
GDExtension, so the Godot Editor needs both the Lunabase Godot project files and a compiled Rust library before the editor can load without errors.

> Please oh please read all of this before working on lunabase

## First-Time Setup

1. Clone the repository.

2. Install Godot 4.7 stable. (You only need the stable version. Don't worry about the other versions like mono and such)

3. Install Rust if you haven't yet

4. Build the Lunabase GDExtension.


   In lunabase-lib run
   ```
   cargo build
   ```

   This should create the native library that Godot loads:
   - Windows: `target/debug/lunabase_lib.dll`
   - Linux: `target/debug/liblunabase_lib.so`
   - macOS: `target/debug/liblunabase_lib.dylib`

5. Open the Godot project.
   In Godot, click "Import" and select the project.godot file in new-lunabase/

## How to link GDExtension

Running cargo build should be enough since the lunabase.gdextension file tells godot where to look for the rust library. If you get this error"

```
GDExtension dynamic library not found: 'res://lunabase.gdextension'
Cannot get class 'LunabaseConnection'
```

then the Rust extension probably has not been built yet, or it was built in the
wrong location. Run `cargo build` from `lunabase-lib`, then restart Godot.

> Rebuild the GDExtension any time code in `lunabase-lib/src/lib.rs` changes.

# Contributing

Some things to keep in mind to help avoid headaches and keep all our code nice and friendly :)

- Follow [Godot's GDScript style guide](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_styleguide.html).
- Keep components modular and scoped.
- Organize each feature (e.g., GUI, system) in its own folder.
- Include a README in your folder if:
  - Your component has dependencies.
  - It needs explanation for usage or integration.

Example: If you're working on a GUI component, it should live in its own folder named after the component. This folder should contain all related scenes, assets, and scripts. You can use subfolders to keep things tidy and organized.
- Try to keep components modular and scoped to their purpose.
- No need to be overly granular, but clarity and separation help avoid future headaches.
- Similar components (like GUI elements) can be grouped under a parent folder such as GUI/.

# Other considerations

As of now I am using the Compatibility renderer for speed. This choice is open to discussion. Feel free to propose changes to this readme.



# Development Guide
Heyo,   
This is a read me for those who are helping develop our mission control application.
in this read me we will be cover these topics 
   
   - Godot in context with our Repository (re how it connects with our rust code)
   - General Folder Layout for Godot
   - Different Tools that are typically used for development

# Godot in Context with Our Repository

Generally, Godot is pretty separated with our backend code. Godot itself really doesn't contain much "code" that handles logic like turning the robot etc. Really we use Godot as our front end just to display and send commands to our robot. 
That being said, In order to have input and output between mission control and our bot. We need to be able to run code that retrieves or sends instructions to lunabot. However, We have a problem. Godot uses gdscript for scripting and our lunabot runs on rust. So we need some sort of "bridge" to send and receive data from lunabot. So, Our answer to this is using gdext (https://godot-rust.github.io/book/ ) <----- learn from this 

Basically, gdext lets us run rust code.
Our "Bridge" file is located in 'lunabase-lib/src/lib.rs'
Though, This file may seem very confusing with all the Mutex,Arc,tokio crap. 
Its not as complex as it looks.

<img width="1021" height="662" alt="gdext-lunabase-struct" src="https://github.com/user-attachments/assets/fc5e6f12-d724-4ed6-ba3c-4f779090f644" />
  
As we see in struct called LunabaseConnection, This struct contains data that is shared. So an example is like our orientation values being passed to our godot app.

Looking at the part where it says
```
impl LunabaseConnection {
```
We notice a lot of functions that are created with the `#[func]` tag attributed to it. Essentially,These functions are functions that godot is able to identify and run, while also being able to utilize our shared data or be able to access rust functions that godot would not be able to run directly. 
Lets take our function
```
    #[func]
    fn set_speed(&mut self, weight: f64) -> f64 {
        self.current_weight = weight;
        self.send_msg(FromLunabase::Steering(Steering::new(
            0.0,
            0.0,
            self.current_weight,
        )));
        self.current_weight
    }
```
as an example.
This function's intention is to change the robots "speed" or how fast it is able to accelerate. 
Basically, 
Since it contains that aforementioned tag. The function is visible to godot.

## The Godot Side

We have only looked at this through the lens of rust development. What does this look like in terms of Godot?

Typically, The most straight forward way to run commands is to create a node that is the type of LunabaseConnection and you can run whatever command you wrote in lib.rs directly.
However, For our project, We have a command pattern. Basically, The reason why we have this pattern is so we can separate our data.

take our Lift Actuator Command

----------------------------------------------------------------------------------
```
class_name LiftActuatorsCommand
extends Command

#Move lift actuators, positive up, negative downed back

#In rust its an i8 (-128 to 127)
@export var lift: int = 0


func execute(actor: Node) -> void:
	if actor.has_method("send_lift_actuators"):
		# Convert from i8 range (-128 to 127) to float range (-1.0 to 1.0)
		var speed: float = lift / 127.0 if lift >= 0 else lift / 128.0
		actor.send_lift_actuators(speed)
	else:
		push_error("Actor does not have send_lift_actuators method")
        
```
-----------------------------------------------------------------------------
for example 
Essentially for each function we make in our lib.rs for lunabase. We need to create a command that is located in Systems/CommandPatter/Commands 
The Commands essentially follow the structure seen above. Basically, our field called lift is able to be accessed like this 

```
var cmd = LiftActuatorsCommand.new()
cmd.lift = 50
cmd.execute(lunabaseConn)   [lunabaseConn being a our node that we created in our rust script with gdext]
```

# General Folder Layout of Our Godot project :)
<img width="686" height="267" alt="godot-project-overview-img" src="https://github.com/user-attachments/assets/e4ec90c5-5fc8-4696-80f9-e3753598b09b" />


So Here is the main directories of lunabase. 
The main important directories are Systems,Main Controller,GUI, Assets

    1. Systems
        This directory contains all of the gdext commands that we use. We also have control schemes which contain scenes and scripts that are used for our controller binding. We also have our lunabase_connection.tscn which is our gdext node and is a global node.
    2. MainController
        This directory is the heart of our program.This directory contains out main scene and our main script. This is our main. 
    3. GUI 
        This directory contains all GUI related things. Meaning all things like extra indicators, Arena Map,Settings menus etc. Please separate GUI related thing into this folder just to keep out Main Controller Folder clean :>)
    4.  Assets
        Pretty self explanatory, This contains all of the assets and images that we need. 
The rest of the unmentioned directories are pretty extra. Things like demos or unimplemented features.

# Different Tools that are typically used in Development.
    Godot Editor - duh. 
    Any Code editor - duh.
    Mujoco -
        So Mujoco is our simulation environment that we use for testing without a physical robot (duh), Mujoco is a pretty tough thing to set up, So im not going to explain it here. Linked below is our
        readme for setting up our sim environment 
        https://github.com/utahrobotics/utah-lunabotics-2027/blob/main/mujoco-sim/README.md  <-----
        However, I will give some advice with mujoco. 
        1. Make a script or something to stop wasting time to set up static linking.
            This is something that made me save like hours of time. I think its pretty important to have.
            Personally, Here is my script that I run to set up the mujoco environment
            
            echo "exporting  var :)"
            cd /home/maximillionchua/robotics/mujoco_stuff/mujoco-3.3.5-linux-x86_64/mujoco-3.3.5
            export  MUJOCO_STATIC_LINK_DIR=/home/maximillionchua/robotics/mujoco_stuff/mujoco-rs/mujoco/build/lib64
            echo $MUJOCO_STATIC_LINK_DIR
            cd  /home/maximillionchua/utah-lunabotics-2027


            Just make it easier for yourself brah.

        2. use and memorize these commands
            make edit-lunabase - This command basically compiles the lunabase rust code and opens the editor 
            SIM_ARENA=ucf make sim
            SIM_ARENA=artemis make sim  - given that you already set up mujoco correctly, This runs out sim environment.


# Questions?
Maximillion Chua @wumbothumbodumbocumbo - Discord :) 

Carlos Alatorre @Yayito - Discord    

ASK US :)



