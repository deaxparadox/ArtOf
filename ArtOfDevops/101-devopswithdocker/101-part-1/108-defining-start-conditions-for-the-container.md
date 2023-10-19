# Defining start conditions for the container

Let's build an userful image. *youtube-dl* is a program that downloads youtube videos http  [https://rg3.github.io/youtube-dl/download.html](https://rg3.github.io/youtube-dl/download.html). Let's add it to an image - but this time we will change our process. Instead of our current process where we add things to the Dockefile . This time we will open up an interactive session and test stuff before "storing" it in our Dockerfile. By following the youtube-dl install instructions we will see that:

```bash
$ docker run -it ubuntu:18.04



  root@8c587232a608:/# curl -L https://github.com/ytdl-org/youtube-dl/releases/download/2021.12.17/youtube-dl -o /usr/local/bin/youtube-dl
  bash: curl: command not found
```

..and, we already know, curl is not installed - let's add `curl` and `apt-get` again.

```bash
$ apt-get update && apt-get install -y curl
$ curl -L https://github.com/ytdl-org/youtube-dl/releases/download/2021.12.17/youtube-dl -o /usr/local/bin/youtube-dl
```

At some point, you may have noticed that sdo is not installed either, but since we are root we don't need it.

Next, we will add permissions and run it:

```bash
$ chmod a+rx /usr/local/bin/youtube-dl
$ youtube-dl
  /usr/bin/env: 'python': No such file or directory
```

Okay - On the top of the `youtube-dl` download page we notice this message:

Rmemeber youtube-dl requires Python version 2.6, 2.7 or 3.2+ to work except for Windows.exe.

So we will add Python

```bash
$ apt-get install -y python3.9
```

And let's run it again

```bash
$ youtube-dl

  WARNING: Assuming --restrict-filenames since file system encoding cannot encode all characters. Set the LC_ALL environment variable to fix this.
  Usage: youtube-dl [OPTIONS] URL [URL...]

  youtube-dl: error: You must provide at least one URL.
  Type youtube-dl --help to see a list of all options.
```

It works (we just need to give an URL), but we notice that it outputs a warning about `LC_ALL`. In a regular Ubuntu desktop/server install the localization settings are (usually) set, but in this image they are not set, as we can see by running env in our container. To fix this without installing additional locales, see this: [https://stackoverflow.com/a/41648500](https://stackoverflow.com/a/41648500)

```bash
$ LC_ALL=C.UTF-8 youtube-dl
```

And it works!. Let's persists it for our session and try downloading a video:

```bash
$ export LC_ALL=C.UTF-8
$ youtube-dl https://imgur.com/JY5tHqr
```

So now when we know exactly what we need. Starting FROM ubuntu:18.04, add these to our `Dockerfile`. We should always try to keep the most prone to change rows at the bottom, by adding the instructions to the bottom we can preserve our cached layers - this is handy practise to speed up creating the initial version of a Dockerfile when it has time-consuming operations like downloads. Also added WORKDIR, this will ensure the videos will be downloaded there.

```Dockerfile
FROM ubuntu:18.04

WORKDIR /mydir

RUN apt-get update && apt-get install -y curl python
RUN curl -L https://github.com/ytdl-org/youtube-dl/releases/download/2021.12.17/youtube-dl -o /usr/local/bin/youtube-dl
RUN chmod a+x /usr/local/bin/youtube-dl

ENV LC_ALL=C.UTF-8

CMD ["/usr/local/bin/youtube-dl"]
```

- Instead of using `RUN export LC_ALL=C.UTF-8` we store the environment directly in the image with ENV

- We also override `bash` as our image command (set on the base image) with *youtube-dl* itself. This will not work, but let's see why.

When we bild this as youtube-dl and run it:

```bash
$ docker build -t youtube-dl .
  ...

$ docker run youtube-dl

  Usage: youtube-dl [OPTIONS] URL [URL...]

  youtube-dl: error: You must provide at least one URL.

  Type youtube-dl --help to see a list of all options.
```

So far so good, but now the natural way to use this image would be to give the URL as an argument:

```bash
$ docker run youtube-dl https://imgur.com/JY5tHqr

  /usr/local/bin/docker: Error response from daemon: OCI runtime create failed: container_linux.go:296: starting container process caused "exec: \"https://imgur.com/JY5tHqr\": stat https://imgur.com/JY5tHqr: no such file or directory": unknown.

  ERRO[0001] error waiting for container: context canceled
```

As we now know, the argument we gave it is replacing the command or `CMD`:

```bash
$ docker run -it youtube-dl ps
  PID TTY          TIME CMD
    1 pts/0    00:00:00 ps
$ docker run -it youtube-dl ls -l
total 0
$ docker run -it youtube-dl pwd
/mydir
```


We need a way to have something *before* the command. Luckily we have a way to do this: we can use `ENTRYPOINT` to define the main executable and then Docker will combine our run arguments for it.

```Dockerfile
FROM ubuntu:18.04

WORKDIR /mydir

RUN apt-get update && apt-get install -y curl python
RUN curl -L https://github.com/ytdl-org/youtube-dl/releases/download/2021.12.17/youtube-dl -o /usr/local/bin/youtube-dl
RUN chmod a+x /usr/local/bin/youtube-dl

ENV LC_ALL=C.UTF-8

# Replacing CMD with ENTRYPOINT
ENTRYPOINT ["/usr/local/bin/youtube-dl"]
```

And now it works like it should:

```bash
$ docker build -t youtube-dl .
$ docker run youtube-dl https://imgur.com/JY5tHqr

  [Imgur] JY5tHqr: Downloading webpage
  [download] Destination: Imgur-JY5tHqr.mp4
  [download] 100% of 190.20KiB in 00:0044MiB/s ETA 00:000
```

With `ENTRYPOINT` docker run now executed the combined `/usr/local/bin/youtube-dl https://imgur.com/JY5tHqr` inside the container with that command!


`ENTRYPOINT` vs `CMD` can be confusing - in a properly set up image such as our youtube-dl the command represents an argument list for the entrypoint. By default entrypoint is set as `/bin/sh` and this is passed if no entrypoint is set. This is why giving path to a script file as `CMD` works: you're giving the file as a parameter to `/bin/sh`.

If an image defines both, then the CMD is used to give default arguments to the entrypoint. Let us now add a CMD to the Dockerfile:

```bash
FROM ubuntu:18.04

WORKDIR /mydir

RUN apt-get update && apt-get install -y curl python
RUN curl -L https://github.com/ytdl-org/youtube-dl/releases/download/2021.12.17/youtube-dl -o /usr/local/bin/youtube-dl
RUN chmod a+x /usr/local/bin/youtube-dl

ENV LC_ALL=C.UTF-8

ENTRYPOINT ["/usr/local/bin/youtube-dl"]

# define a default argument
CMD ["https://imgur.com/gallery/xwJgflf"]
```

Now (after bulid again) the image can be run without arguments to download the video defined in CMD:

```bash
$ docker run youtube-dl

  [Imgur] JY5tHqr: Downloading webpage
  [download] Destination: Imgur-JY5tHqr.mp4
  [download] 100% of 190.20KiB in 00:0044MiB/s ETA 00:000
```

The argument defined by CMD can be overridden by giving one in the command line:

```bash
$ docker run youtube-dl https://imgur.com/gallery/iT3U4K4
[imgur:gallery] iT3U4K4: Downloading JSON metadata
[download] Downloading playlist: A candlelight dinner needs a doggie serenade
[imgur:gallery] playlist A candlelight dinner needs a doggie serenade: Collected 1 video ids (downloading 1 of them)
[download] Downloading video 1 of 1
[Imgur] ZkjbtYw: Downloading webpage
[download] Destination: Imgur-ZkjbtYw.mp4
[download] 100% of 5.02MiB in 00:0183MiB/s ETA 00:00known ETA
[download] Finished downloading playlist: A candlelight dinner needs a doggie serenade
```