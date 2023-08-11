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