# In-depth dive to images

Images are the basic building blocks for containers and other images. When you "containerize" an application you work towards creating the image.

### Where do the images come from?

When running a command such as `docker run hello-world`, Docker will automatically search `Docker hub` for the image if it not found locally.

This means that we can pull and run any public image from Docker's servers. For example‚ if we wanted to start an instance of the PostgreSQL database, we could just run `docker run postgres`, which would pull and run *https://hub.docker.com/_/postgres/*.

We can search for images in the Docker Hub with docker search. Try running `docker search hello-world`.

The search finds plenty of results, and prints each image's name, short description, amount of stars, and "official" and "automated" statuses.

```bash
$ docker search hello-world

  NAME                         DESCRIPTION    STARS   OFFICIAL   AUTOMATED
  hello-world                  Hello World!…  1988     [OK]
  kitematic/hello-world-nginx  A light-weig…  153
  tutum/hello-world            Image to tes…  90                 [OK]
  ...
```

- When browsing the CLI's search results, you can recognize an official image from the "[OK]" in the "OFFICIAL" column and also from the fact that the image's name has no prefix (aka organization/user). When browsing Docker Hub, the page will show "Docker Official Images" as the repository, instead of a user or organization. For example, see the Docker Hub page of the hello-world image.

- `tutum/hello-world`, is marked as "automated". This means that the image is automatically built from the source repository. Its Docker Hub page shows its previous "Builds" and a link to the image's "Source Repository" (in this case, to GitHub) from which Docker Hub builds the image.

- `kitematic/hello-world-nginx`, is neither an official nor an automated image. We can't know what the image is built from, since its Docker Hub page has no links to any repositories. The only thing its Docker Hub page reveals is that the image is 8 years old. Even if the image's "Overview" section had links to a repository, we would have no guarantees that the published image was built from that source.


[<<< Running and stopping containers](104-running-and-stopping-containers.md) ... [A detail look into an image >>>](106-detail-look-into-an-image.md)