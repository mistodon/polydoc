# polydoc


## Goals

### General

1.  Generate documentation from comments in source code
2.  Parse source to verify properties of documentation, such as argument name/count
3.  Run entirely from the command line
4.  Run at least on Windows, MacOS, and common Linux distros
5.  Be easily extensible, possibly via plugins

### Inputs
1.  Work with multiple input languages
2.  Work with multiple formats of documentation comment

### Outputs
1.  Work with multiple output formats
2.  Preserve structure of input including naming and ordering (it should be possible to re-write the declarations of the source file from the output)


## Non-goals

1.  Static analysis of complex properties of the code
2.  Styling, HTML generation, or any other kind of visual presentation