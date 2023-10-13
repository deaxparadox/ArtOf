# Using DockerFile

Creating an image from a container using Dockerfile is merely two steps:

- Write the Dockerfile
- Build the Dockerfile


To verify:

- list images
- run the new image in a new container 


# Create File

Create docker file `Dockerfile`:

```Dockerfile
FROM ubuntu:latest
RUN apt-get update && apt-get install -y curl build-essential net-tools 
```

```bash
$ docker image build -t testimagedockerfile .
$ docker images 
$ docker image ls
$ docker container run -d -it --name testimagedockerfile_1 testimagedockerfile /bin/bash
$ docker container ls
$ docker container attach testimagedockerfile_1 
```

- Push your image

```bash
$ docker login
$ docker image tag testimagedockerfile:latest  deaxparadox/testimagedockerfile
$ docker image ls
$ docker push deaxparadox/testimagedockerfile
$ history
$
```

## Docker Commands

#### The FROM instructions

- You must start you Dockerfile with a FROM instructions to set the bash image. If you want to create a new image with no parent image, use `FROM scratch`.

- Example : `FROM ubuntu`

#### The MAINTAINER Instruction

- This is a metadata instruction to indicate the Docker author's contact.

- Example: `MAINTAINER Engy <efoda@ieee.org>`

#### The COPY Instruction

- This instruction is used to copy files and directories from your local host to engraved inside the container.

- Example: COPY test /newdir/test

#### The ADD Instruction

- ADD supports URL handling, while COPY does not that.
- ADD supports extra features like local-copy tar extraction.
- ADD supports regular expression handling, while COPY does not.

### The ENV Instruction

This instructions sets the environment varaibles in the container, for example, settings a log path other thatn the long one that the Docker engine sets by default.

- Example: ENV log_dir /var/log

### The USER Instruction

- By default, the Docker engine will set the container's user to root, which can be harmfull, Acutally no one gives root priviledges like that. Therefore, you should set a USER ID and username to your container

- Example: USER 75 engy

### The WORKDIR Instruction

- You use WORKDIR to set your active directory.

- The tricky question that comes here is that paths provided in the WORKDIR instuctions will concatenate. Let us clairfy that with an example:

- Example:

WORKDIR /var
WORKDIR source
RUN apt-get update

- The RUN instruction will be executed in the path of /var/source

### The VOLUME Instruction

- The VOLUME instructions creates a directory in the image filesystem, which can later be used for mounting volumes from the Docker host or the other containers.

- Example: VOLUME vol

### The EXPOSE Instruction

- This instruction is to make the container port accessible from the browser, outside the container, so that the world can send requests to the container services, 

- You can specify multiple ports in a single line.

- By defualt, the Docker engine will assign TCP as your protocol

- Example: EXPOSE 8081 80/udp

- This example will expose port 8081 as a TCP port and 80 as UDP.

### THE RUN Instruction 

- RUN is one of the most used Instructions in the Dockerfile. You use it to install any packages an a new layer inside the image during the build time. For example, your application needs to update ubuntu and then install curl

- In such case, you Dockerfile will look like this:

```Dockerfile
FROM ubuntu 
RUN apt-get update && apt-get install curl
```

### The CMD instructions

#### Difference between RUN and CMD

- The CMD is executed when you run a container from your image, while the RUN instruction is exectued during the build time of the image.

- You can have only one CMD instruction in a Dockerfile. If you add more only the last one takes effect, while you can have many RUN instructions as you need in the same Dockerfile.

- You can add a health check to the CMD instructions, for example `HEALTHCHECK CMD curl --fail http://localhost/health || exit 1`, which tells the Docker engine to kill the container with exit status 1 if the container health fails.

- The CMD syntax uses this form `["parm", "param", "param"]` when used in conjunction with the ENTRYPOINT instruction. It should be in the following form CMD ['executable', 'param1', 'param2', ...] if used by itself.

- Example: CMD "echo "Hello World!"

### The ENTRYPOINT Instructions

- To instantly run an executable command when a container is created from an image, use the `ENTRYPOINT` instructions.

- Again, you can recevie a tricky question about `ENTRYPOINT` and CMD; remember that ENTRYPOINT overrides the CMD instruction, and CMD's parameters are used as arguments to ENTRYPOINT.

- For example, you have Dockerfile that has the following two instructions

```Dockerfile
CMD "This is my contianer"
ENTRYPOINT echo
```

When you run a container from this image, the Docker engine will execute it as ENTRYPOINT echo "This is my container" and will print "This my my container".

### The inspect Command

You can use the inspect subcommand to get all the details of any object in Docker. For now we will see `--filter` and `--format` options.

let us try inspecting you `testimage`:

```bash
$ docker image inspect testimage
```

The output is to long and it consist of section and subsections. If you want a specific value, you add the in sequence after the `--format` option.

For example to get the architecture and operation system an image is compatible with you run 

```bash
$ docker image inspect --format='{{.Architecture}} {{.Os}}' <image-id>
```

The `Architecture` is the section, where the `OS` is a subsection inside it.

An example for the `--filter` option is to filter the dangled images only. You can do that bu running 

```bash
$ docker image ls --filter dangling=true
```

### Multistage Build

The primary purpose of multistage build is optimizing images by copying artifacts selectively from previous stages to keep the image small. 

The Dockerfile in this case contain more than one FROM instruction. You can name your stages as well


```Dockerfile
FROM golang:1.16 AS builder
WORKDIR /go/src/github.com/alexellis/href-counter/
RUN go get -d -v golang.org/x/net/html
COPY app.go .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o app .

FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /root/
COPY --from=builder /go/src/github.com/alexellis/href-counter/app .
CMD ["./app"]

```