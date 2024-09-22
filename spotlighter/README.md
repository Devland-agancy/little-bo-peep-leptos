# How to use

First make sure you are in spotlighter directory : cd spotlighter

3 commands are available

**comment**  

if used alone it will comment every thing inside content dir , if a file or a folder path is given it will comment it.

```
gleam run comment 
gleam run comment chapter4/section_emu.rs
```

**uncomment** 

if used alone it will uncomment every thing inside content dir , if a file or a folder path is given it will comment it . Not when a file/folder is commented . don't add the "#" in the command . just write the name

```
gleam run uncomment 
gleam run uncomment chapter4/section_emu.rs
```

**spotlight**

uncomment a file or a directory and its children and its ascendors . and comments its siblings and its ascendors siblings

```
gleam run spotlight chapter4/exercises
```
