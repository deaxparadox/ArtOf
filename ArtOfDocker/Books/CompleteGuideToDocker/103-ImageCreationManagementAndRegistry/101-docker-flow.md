# Docker Flow

- The Docker flow starts with a command on the client (CLI) pulling an image using the subcommand:

```bash
docker pull
```

- The Docker engine checks if the image in locally at he host or not. If it is not, the engine will pull it fromthe repository. Then it runs it and  crafts a container from that image using subcommand:

```bash
docker run
```

- Think of an image as a stack of layers. This ocntainer is the last layer at the top, where you can manipulate it and add whatevery commands an package on top of the base image. You can save this new image that includes the old parent one and all the new addition using the subcommand (or using the `Dockerfile`):

```bash
docker commit 
```

- The final step in the flow is to push the image to the repository, and you do that using the subcommand"

```bash
docker push
```

In Docker there are two sets of commands: the old and the new.

- The old or the legacy set is only two levels. The first level is *docker*, and the second one is the subcommand as:

    ```bash
    docker pull <image name>
    ```

- The new and start one is three levels. The first level is docker, the second is the object, and the third is the subcommand. Images, containers, networks, node, and volumes are example of Docker objects. In this case, the command to pull an image is:

    ```bash
    docker image pull <image name>
    ```