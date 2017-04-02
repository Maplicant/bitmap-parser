bitmap-parser
===
A high performance /r/place bitmap parser
### Installation
Clone this repo to a local repository. Build using the following command:
```shell
cargo build --release
```
The binary should appear in `./target/release`

## Usage
Run using the following command:
```shell
./bitmap-parser input_file.bmp
```
Or use 
```shell
./bitmap-parser
```
if you want that it fetches a bitmap itself.
A file called `out.png` should appear
