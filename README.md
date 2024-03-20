<!-- gen-readme start - generated by https://github.com/jetpack-io/devbox/ -->
## Getting Started
This project uses [devbox](https://github.com/jetpack-io/devbox) to manage its development environment.

Install devbox:
```sh
curl -fsSL https://get.jetpack.io/devbox | bash
```

Start the devbox shell:
```sh 
devbox shell
```

Run a script in the devbox environment:
```sh
devbox run <script>
```
## Scripts
Scripts are custom commands that can be run using this project's environment. This project has the following scripts:

* [cluster-create](#devbox-run-cluster-create)
* [cluster-delete](#devbox-run-cluster-delete)
* [cluster-update](#devbox-run-cluster-update)
* [echo](#devbox-run-echo)
* [just](#devbox-run-just)
* [just-create](#devbox-run-just-create)
* [just-update](#devbox-run-just-update)
* [start](#devbox-run-start)
* [stop](#devbox-run-stop)
* [test](#devbox-run-test)

## Shell Init Hook
The Shell Init Hook is a script that runs whenever the devbox environment is instantiated. It runs 
on `devbox shell` and on `devbox run`.
```sh
echo 'Welcome to devbox!' > /dev/null
```

## Packages

* [go-task](https://www.nixhub.io/packages/go-task)
* [tilt](https://www.nixhub.io/packages/tilt)
* [kind@latest](https://www.nixhub.io/packages/kind)
* [hello@latest](https://www.nixhub.io/packages/hello)
* [just](https://www.nixhub.io/packages/just)

## Script Details

### devbox run cluster-create
```sh
just create
```
&ensp;

### devbox run cluster-delete
```sh
kind delete cluster -n devbox
```
&ensp;

### devbox run cluster-update
```sh
kind create cluster -n devbox
```
&ensp;

### devbox run echo
```sh
hello sht!
```
&ensp;

### devbox run just
```sh
just
```
&ensp;

### devbox run just-create
```sh
just create
```
&ensp;

### devbox run just-update
```sh
just update
```
&ensp;

### devbox run start
```sh
echo 'Starting devbox!' > /dev/null
```
&ensp;

### devbox run stop
```sh
echo 'Stopping devbox!' > /dev/null
```
&ensp;

### devbox run test
```sh
echo 'Starting devbox!' > /dev/null
```
&ensp;



<!-- gen-readme end -->
