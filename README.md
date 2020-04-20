# The Cocktail Catalogue Backend
## Prerequisites
### Rust
Install Rust. Guide TBA.

### SQLite
Install SQLite using `apt-get`:
```
$ apt-get update
$ apt-get install libsqlite3-dev
```

## Using Docker to build for Raspberry Pi
This project was made to be run on a Raspberry Pi I had lying around at home, so
to make developing from a Windows machine easier, Docker was used.

To run the program with Docker in a Raspbian Jessie environment, build the image:
```bash
$ docker build . -t rusty-pie
```
First time you run the container, set it up with `cargo vendor` so that future
cargo builds are cached:
```
$ docker run -v $(pwd):/app -it rusty-pie bash
root@c70b8c729e19:/app# mkdir .cargo
root@c70b8c729e19:/app# cargo vendor > .cargo/config
```
Now you should be good to simply run
```
docker run -v $(pwd):/app -it rusty-pie cargo run
```
or other build commands, and the build will be cached.

## TODO
- [ ] Add regular automatic backups of the database (for example using `.backup ?DB? FILE` from SQLite)
- [ ] Investigate `unwrap`s on `query_and_then` in `database.rs`.
- [ ] Write guides on installation and setup.
- [x] Utilize serde deserialization from [serde_rusqlite](https://github.com/twistedfall/serde_rusqlite).