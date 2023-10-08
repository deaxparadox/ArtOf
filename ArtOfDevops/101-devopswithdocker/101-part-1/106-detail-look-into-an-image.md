# A detail look into an image

Let's pull Ubuntu:

```bash
$ docker pull ubuntu
  Using default tag: latest
  latest: Pulling from library/ubuntu
```

Since we didn't specify a tag, Docker defaulted to `latest`, which is usually the latest image built and pushed to the registry. **However**, in this case, the repository's README says that the `ubuntu:latest` tag points to the "latest LTS" instead since that's the version recommended for general use.

Images can be tagged to save different versions of the same image. You define an image's tag by adding `:<tag>` after the image's name.

Ubuntu's Docker Hub page reveals that there's a tag named 18.04 which promises us that the image is based on Ubuntu 18.04. Let's pull that as well:

```bash
$ docker pull ubuntu:18.04

  18.04: Pulling from library/ubuntu
  c2ca09a1934b: Downloading [============================================>      ]  34.25MB/38.64MB
  d6c3619d2153: Download complete
  0efe07335a04: Download complete
  6b1bb01b3a3b: Download complete
  43a98c187399: Download complete
```

Images are composed of different layers that are downloaded in parallel to speed up the download.

We can also tag images locally for convenience, for example, `docker tag ubuntu:18.04` `ubuntu:bionic` creates the tag `ubuntu:bionic` which refers to `ubuntu:18.04`.

Tagging is also a way to "rename" images. Run `docker tag ubuntu:18.04 fav_distro:bionic` and check `docker images` to see what effects the command had.

To summarize, an image name may consist of 3 parts plus a tag. 

- Usually like the following: *registry/organisation/image:tag*. 
- But may be as short as ubuntu, then the *registry* will default to *Docker hub*, *organisation* to *library* and *tag* to *latest*. 
- The organisation may also be a user, but calling it an organisation may be more clear.

[<<< Images](105-images.md) ... [Building Images >>>](107.building-images.md)