# Container Lifecycle

Let ur try to control the container and change its status. 

In one terminal try this:

```bash
docker container run -it --name test ubuntu /bin/bash
```

The `-it` stands for an interactive terminal to interact with the container commands inside it. 

In another temrinal, try pausing the infinite loop of printing the date every 5 seconds:

```bash
docker container pause test
```

The first terminal will halt. Write:

```bash
docker container ls
```

To unpause:

```bash
docker container unpause test
```

To stop:

```bash
docker container stop test
```

To verify the status of docker container use :

```bash 
docker container ls
```

To list all the continers on your host, including the exited ones, run 

```bash
docker container ls -a
```

- To entirely remove it from the host, run 

```bash
docker container rm test
```

To run the container in a daemon or detached mode, you use the `-d` option in the run subcommand. 

```bash
docker container run -d -it --name test1 ubuntu
```

To attach the container:

```bash
docker container attach test1
```

To known everything that has been executed in the container, you can use:

```bash
docker logs test1
```