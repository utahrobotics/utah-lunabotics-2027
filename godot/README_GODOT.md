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
