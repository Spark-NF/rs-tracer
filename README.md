# Raytracer

## About
Raytracer to try out the Rust language.
Generates an images from a scene file using [ray tracing](https://en.wikipedia.org/wiki/Ray_tracing_(graphics)).

### Usage
```
raytracer [-h] [-o FILE] SCENE
```

On first run, the program will ask you for your OAuth token and secret. It will then be stored in a `.oauth.json` file so that you don't need to enter them every single time.

### Arguments
* `SCENE`: the scene file to open, in JSON format (a few of them are provided in the `scenes/` directory)
* `-o FILE`, `--out FILE`: set the output file for the raytracing result (defaults to `out.png`)

### Example
```
cargo run -- -o result.png scenes/basic.json
```

Will ray-trace the [scenes/basic.json](scenes/basic.json) scene and put the result into the `result.png` file.

## Authors
* [Nicolas Faure](https://github.com/Spark-NF)

## License
The program is licensed under the [Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0).