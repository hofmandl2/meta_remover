# meta_remover

A simple, non-UI program stripping meta-information from jpg and png files
also when they are contained within zip files.

## Usage

```
meta_remover a.png b.jpg c.zip d.txt
```

 * Unknown file-types will be ignored
 * Files that cannot be found will be ingored
 * Stripped files will have a copy created at `a_no_meta.png`, `b_no_meta.jpg`, `c_no_meta.zip` and so on
 * no pre-existing files will be overwritten
