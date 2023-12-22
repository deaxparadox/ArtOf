# Building Images

We now build our own images and get to talk about `Dockerfile` and why it's so great.

Dockerfile is simply a file that contains the build instructions for an image. You define what should be included in the image with different instructions.

Let's take a most simple application and containerize it first. Here is a script called "hello.sh"

```bash
#!/bin/sh

echo "Hello, docker!"
```

First, we will test that it even works. Create the file, add execution permissions and run it:

```bash
$ chmod +x hello.sh

$ ./hello.sh
    Hello, docker!
```

- If you're using Windows you can skip these two and add chmod +x hello.sh to the Dockerfile.


And now to create an image from it. We'll have to create the `Dockerfile` that declares all of the required dependencies. At least it depends on something that can run shell scripts. So I will choose Alpine, it is a small Linux distribution and often used to create small images.

Even though we're using Alphine here.

Now create a file and name it "Dockerfile" and lets put the following instructions inside it:

```Dockerfile
# Start from the alpine image that is smaller but no fancy tools
FROM alpine:3.13

# Use /usr/src/app as our workdir. The following instructions will be executed in this location.
WORKDIR /usr/src/app

# Copy the hello.sh file from this location to /usr/src/app/ creating /usr/src/app/hello.sh
COPY hello.sh .

# Alternatively, if we skipped chmod earlier, we can add execution permissions during the build.
# RUN chmod +x hello.sh

# When running docker run the command will be ./hello.sh
CMD ./hello.sh
```


Great! We can use the command `docker build` to turn the Dockerfile to an image.

By default `docker build` will look for a file named Dockerfile. Now we can run `docker build` with instructions where to build `.` and given it a name (`-t <name>`):

```bash
$ docker build . -t hello-docker
  Sending build context to Docker daemon  54.78kB
  Step 1/4 : FROM alpine:3.13
   ---> d6e46aa2470d
  Step 2/4 : WORKDIR /usr/src/app
   ---> Running in bd0b4e349cb4
  Removing intermediate container bd0b4e349cb4
   ---> b382ca27c182
  Step 3/4 : COPY hello.sh .
   ---> 7fbc1b6e45ab
  Step 4/4 : CMD ./hello.sh
   ---> Running in 24f28f026b3f
  Removing intermediate container 24f28f026b3f
   ---> 444f21cf7bd5
  Successfully built 444f21cf7bd5
  Successfully tagged hello-docker:latest

$ docker images
  REPOSITORY            TAG          IMAGE ID       CREATED         SIZE
  hello-docker          latest       444f21cf7bd5   2 minutes ago   5.57MB
```

Now executing the application is as simple as running `docker run hello-docker`. Try it!

During the build we see that there are multiple steps with hashes and intermediate containers. The steps here represent the layers so that each step is a new layer to the image.

The **layers** have multiple functions. We often try to limit the number of layers to save on storage space but layers can work as a cache during build time. If we just edit the last lines of Dockerfile the build command can start from the previous layer and skip straight to the section that has changed. COPY automatically detects changes in the files, so if we change the hello.sh it'll run from step 3/4, skipping 1 and 2. This can be used to create faster build pipelines.

The intermediate containers are containers created from the image in which the command is executed. Then the changed state is stored into an image. We can do similiar task and a new layer manually. Create a new file called `additional.txt` and let's copy it inside the container and learn new trick while we're at it! We'll need two terminals so I will label the lines with 1 and 2 representing the two.


```bash
1 $ docker run -it hello-docker sh
1 /usr/src/app #
```

Now we're inside of the container. We replaced the CMD we defined earlier with `sh` and used -i and -t to start the container so that we can interact with it. In the second terminal we will copy the file here.

```bash
2 $ docker ps
2   CONTAINER ID   IMAGE          COMMAND   CREATED         STATUS         PORTS     NAMES
    9c06b95e3e85   hello-docker   "sh"      4 minutes ago   Up 4 minutes             zen_rosalind

2 $ touch additional.txt
2 $ docker cp ./additional.txt zen_rosalind:/usr/src/app/
```

I created the file with touch right before copying it in. Now it's there and we can confirm that with ls:

```bash
1 /usr/src/app # ls
1 additional.txt  hello.sh
```

Great! Now we've made a change to the container. We can use `diff` to check what has changed

```bash
2 $ docker diff zen_rosalind
    C /usr
    C /usr/src
    C /usr/src/app
    A /usr/src/app/additional.txt
    C /root
    A /root/.ash_history
```

The character in front of the file name indicates the type of the change in the container's filesystem: **A = added**, **D = deleted**, **C = changed**. The additional.txt was created and our `ls` created .ash_history.

Next we will save the changes as a new image with the command docker commit:

```bash
2 $ docker commit zen_rosalind hello-docker-additional
    sha256:2f63baa355ce5976bf89fe6000b92717f25dd91172aed716208e784315bfc4fd
2 $ docker images
    REPOSITORY                   TAG          IMAGE ID       CREATED          SIZE
    hello-docker-additional      latest       2f63baa355ce   3 seconds ago    5.57MB
    hello-docker                 latest       444f21cf7bd5   31 minutes ago   5.57MB
```

We will actually never use Docker commit again during this course. This is because defining the changes to the Dockerfile is much more sustainable method of managing changes. No magic actions or scripts, just a Dockerfile that can be version controlled.

Let's do just that and create hello-docker with v2 tag that includes the file additional.txt. The new file can be added with a `RUN` instruction:

```Dockerfile
# Start from the alpine image
FROM alpine:3.13

# Use /usr/src/app as our workdir. The following instructions will be executed in this location.
WORKDIR /usr/src/app

# Copy the hello.sh file from this location to /usr/src/app/ creating /usr/src/app/hello.sh.
COPY hello.sh .

# Execute a command with `/bin/sh -c` prefix.
RUN touch additional.txt

# When running Docker run the command will be ./hello.sh
CMD ./hello.sh
```

Now we used the RUN instruction to execute the command `touch additional.txt` which creates a file inside the resulting image. Pretty much anything that can be executed in the container based on the created image, can be instructed to be run with the RUN instruction during the build of a Dockerfile.

Build now the Dockerfile with `docker build . -t hello-docker:v2` and we are done! Let's compare the output of ls:



```bash
$ docker run hello-docker-additional ls
  additional.txt
  hello.sh

$ docker run hello-docker:v2 ls
  additional.txt
  hello.sh
```

Now we know that all instructions in a Dockerfile except **CMD** are executed during build time. **CMD** is executed when we call docker run, unless we overwrite it.

[<<< Details look into an Image](106-detail-look-into-an-image.md) ... [Defining start conditions for the Container >>>](108-defining-start-conditions-for-the-container.md)