# jpeg compression exploration

this repo contains the code used for a project that explored JPEG compression and the discrete cosine transform from a mathematical perspective. the contained program can either compress an image using a simple JPEG compression algorithm, or generate an image that shows the basis functions of the discrete cosine transform that are used in JPEG.

## running
to compress an image:
```
cargo run -- compress examples/stonehenge.png 80
```

to generate the dct basis image:
```
cargo run -- basis
```
