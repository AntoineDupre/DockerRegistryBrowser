# Docker registry browser

Tool to browse the maxiv docker registry.
Demo: https://asciinema.org/a/FJBz87bL0ZXXQciLRSqk9ogwl


## How to install
Simply with cargo:
```bash
$ cargo install --git https://github.com/AntoineDupre/DockerRegistryBrowser.git
```

## How to compile
Simply with cargo:
```bash
$ cargo build --release
```

## How to use it
Pull from upstream
```bash
docker-browser -u "your-docker-registry-url"
```


Run the container in interactive mode (docker -it)
```bash
$ maxiv-docker-browser -u "your-docker-registry-url" -r
```

## Dependancies
reqwest = "0.9.20"
skim = "0.6.8"
argparse = "0.2.2"
