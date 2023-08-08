# Using Commit

At the shell, type `docker container commit test1 testimage`. A new image will be created. To check the image has been created, run `docker image ls`.


### Install a package inside a container and commit it into a new image.

```bash
$ docker container run -d -it --name test1 ubuntu /bin/bash
$ docker container ls
$ docker contaier attach test1
$ apt-get update && apt-get install curl
$ docker container commit test1 testimage
```