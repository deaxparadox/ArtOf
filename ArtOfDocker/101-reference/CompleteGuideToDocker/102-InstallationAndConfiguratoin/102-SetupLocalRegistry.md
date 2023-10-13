# How to Set Up a Local Registry

A Docker registry is a location where Docker images can be pushed, pulled, and stored. Registered users have their own spaces in the registry. This user-named space si called repository. 

A repository that is used for storing images. For example, assume a user have a public image called *firstapp* that is pushed to DockerHub. To reach this image, one must use the image repository name, which is \<username\>/\<image name\>.


To Setup a local registry without a GUI, we will use a repository image that is one DockerHUB callled Registry with tag 2 (registry:2). The installation command is:

```bash
docker run -d -p 5000:5000 --restart=always --name registry registry:2
```

- -d: option is to run as a daemon in the background.
- -p: is to set the \<host port\>:\<container port>.
- We set the *restart* policy to be *always* and set the container name as *registry*.


To test this registry, we pull any image from Docker Hub, for example, 

```bash
docker pull busybox
```

Then rename it to add the local repository name using:

```bash
docker tag busybox localhost:5000/firstapp
```

Now, push it to the local repository using:

```bash
docker push localhost:5000/firstapp
```

Delete all the local images using:

```bash
docker image remove busybox
```

and

```bash
docker image remove localhost:5000/firstapp
```

To make sure that we do not have trace of these images, list the images using:

```bash
docker image ls
```

To remove the local registry, stop it and remove it using:

```bash
docker container stop registry && docker container rm -v registry
```