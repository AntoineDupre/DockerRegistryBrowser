# maxiv_docker_browser

Tool to browse the maxiv docker registry

## How to install
Simply with cargo:
```bash
$ cargo install --git https://gitlab.maxiv.lu.se/antdup/maxiv_docker_browser
```

## How to compile
Simply with cargo:
```bash
$ cargo build --release
```

It prints the selected result in stdout. It can be used directly with docker like:
```bash
$ docker_browser | xargs docker pull
```
The previous command will display the docker catalog and allow you to select one of the docker image. Then is image is going to be pulled by docker


It can also be used to run a containt

```bash
function docker_run() {
  local cid
  cid=$(docker_browser | awk '{print $1}')

  [ -n "$cid" ] && docker run -it  "$cid"
}
```
