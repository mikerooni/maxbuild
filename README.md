# maxbuild

This is a build tool for quickly assembling frozen max4live devices from an unfrozen device (template)
and additional files or directories to be included in the final package.


## Usage

```
maxbuild --template <template-file> --output-file <output-file> --device-type <type>
```


### Including additional files or folders

To include additional files or folders in your frozen `.amxd` file, you can use the `--include` argument
(as many times as necessary).

```
maxbuild -t <template> -o <output> -d <type> --include <folder-a> --include <folder-b> --include <single-file>
```

> #### ⚠ INCLUDED FILE NAMES ⚠
> 
> Note that due to how the Max search path works, all included files **must** have unique names!  
> This includes files across any included folders and their subdirectories.


### Device Types
The device type can be one of the following:

- `instrument`
- `audio-fx`
- `midi-fx`
- `note-generator`
- `note-transformer`


## Special Thanks

This project wouldn't have been possible without the `maxdiff` tool by Ableton, whose source code
has been a great reference for developing the frozen `.amxd` device packaging part of this tool:  
[GitHub Repository of **maxdevtools**](https://github.com/Ableton/maxdevtools)

Special thanks also goes to the open source libraries used in this project!  
See [opensource.txt](./opensource.txt) for a list of used crates and their licenses. 
