# image

The `image` component takes in a path as an argument and renders the image stored at that path to the terminal.
A supported terminal must be used for this to function, as not all terminal emulators are capable of displaying images.

The path is relative to the directory that the program was executed from, and not the source file's root.

!> Due to cache issues, loading many images will permanently unload previously loaded images. This is a known bug and the only workaround at this time is to not load too many images in a single session.

## Example usage

```
layout {
    // Renders the provided image file to the terminal.
    image "path/to/image.png";
}
```
