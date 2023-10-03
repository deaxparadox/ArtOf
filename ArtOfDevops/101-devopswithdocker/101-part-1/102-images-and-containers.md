# Image and containers

Containers are instances of images. A basic mistake is to confuse images and containers.

In short, an image is like a blueprint or template, while a container is an instance of that blueprint or template.

## Image

A Docker image is a file. An image never changes; you can not edit an existing file. Creating a new image happens by starting from a base image and adding new **layers** to it. We will talk about layers later, but you should think of images as *immutable*, they can not be changed after they are created.

List all your images with `docker image ls`:

```bash
$ docker image ls
REPOSITORY                        TAG        IMAGE ID       CREATED        SIZE
testimagedockerfile               latest     e8ca7607aa0b   7 weeks ago    401MB
hello-world                       latest     9c7a54a9a43c   5 months ago   13.3kB
```

Containers are created from images, so when we ran hello-world twice we downloaded one image and created two of them from the single image.

Well then, if images are used to create containers, where do images come from? This image file is built from an instructional file named **Dockerfile** that is parsed when you run `docker image build`.

Dockerfile is a file named Dockerfile, that looks something like this

**Dockerfile** 

```Dockerfile
FROM <image>:<tag>

RUN <install some dependencies>

CMD <command that is executed on `docker container run`>
```

The only difference is that Dockerfile is written by us, whereas image is written by our machine based on the Dockerfile!


## Containers

Containers only contain that which is required to execute an application; and you can start, stop and interact with them. They are **isolated** environments in the host machine with the ability to interact with each other and the host machine itself via defined methods (TCP/UDP).

List all your containers with `docker container ls`:

```bash
$ docker container ls     
CONTAINER ID   IMAGE     COMMAND   CREATED   STATUS    PORTS     NAMES
```

Without `-a` flag it will only print running containers. The hello-worlds we ran already exited.

```bash
$ docker container ls -a
  CONTAINER ID   IMAGE           COMMAND      CREATED          STATUS                      PORTS     NAMES
  b7a53260b513   hello-world     "/hello"     5 minutes ago    Exited (0) 5 minutes ago              brave_bhabha
  1cd4cb01482d   hello-world     "/hello"     8 minutes ago    Exited (0) 8 minutes ago              vibrant_bell
```

The command `docker container ls` has also a shorter form `docker ps` that is preferred by many since it requires much less typing...