![Alt Text](./images/banner.svg)
# Enki

## WIP

A non vim Cross platform terminal Text Editor powered by [xi-editor](https://github.com/xi-editor/xi-editor).

### Requirements

  True color support for the terminal

### Building
```bash
   git clone https://github.com/Bytebuddha/Enki
   cd Enki
   cargo build --release
```

### Arguments
 - conf(--conf -c) Configuration dirextory to use
 - log(--log  -l) Name of a file to write logs to
 - xi(--xi -x)  specify the xi executable to use
 - verbose(--verbose -v) The verbosity of the generated log file(requires log)

### Config Directory
  If no configuration directory is specified `$XDG_CONFIG_DIRS/enki` will be used.
The xi & extras subdirectories will be passed to xi-editor. xi editor plugins like
the syntax highlighting plugin must be installed here to function
### Startup File
  If a file named `startup.json` is located at the root of the configuration directory,
enki will load config parameters, run actions, and set some default cli arguments.
### Config
  Configuration values can be set with the prompt or initialially loaded by setting
  with the them in the startup file.

  |key|type|default|description|
  |---|----|-------|-----------|
  |display_gutter|Boolean|True|Toggle displaying the syntax guttter|
  |display_top_bar|Boolean|True|Toggle displaying the top bar|
  |display_status_bar|Boolean|True|Toggle Displaying the status bar|
  |display_line_endings|Boolean|False|Toggle Displaying of line ending characters|
  |tab_size|Number|4|The length of the tab characters (this can be overwritter by xi)|
  |prompt_chars|String|»|The character to use for the prompt input|

### Actions
  Actions represent something enki can perform, saving, switching views, etc..
Actions can be bound to specific events using the command prompt allowing keybindings
to be set/unset from the startup file, or the command prompt.
