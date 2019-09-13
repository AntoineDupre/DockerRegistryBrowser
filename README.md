# Docker registry browser

Tool to browse the maxiv docker registry

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

