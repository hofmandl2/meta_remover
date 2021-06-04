# meta_remover

A simple, non-UI program stripping meta-information from `jpg` and `png` files
also when they are contained within `zip` files.

## Usage

```
meta_remover a.png b.jpg c.zip d.txt
```

 * Unknown file-types will be ignored
 * Files that cannot be found will be ignored
 * Stripped files will have a copy created at `a_no_meta.png`, `b_no_meta.jpg`, `c_no_meta.zip` and so on
 * no pre-existing files will be overwritten

## Build & Install

```
cargo build --release
```

should produce the standalone executable that you can copy or symlink to e.g. `~/.local/bin`
or `/usr/local/bin`.

## Integration into nautilus

Simply copy the file `nautilus/Remove Image Metadata`
to the folder `~/.local/share/nautilus/scripts`, make sure it is executable
and you should have a handy right-click menu entry available to
invoke the script on a file or a selection of files.
