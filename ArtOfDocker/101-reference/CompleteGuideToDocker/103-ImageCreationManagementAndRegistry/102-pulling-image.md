# Pullling an Image

At the shell prompt, type:

```bash
docker image pull alpine
```

```bash
docker image ls
```

The output of *docker image ls* shows the following:

- REPOSITORY: This is the name of the image. In this example, the repository name is alphine.
- TAG: This is the image tag or version. Any image can have several tags for the same image ID. The Docker engine assigns *latest* as the default tag.
- IMAGE ID: Every image can have several tags but will have one unique 64-hex digit ID. The Docker engine autogenerates it. In the default display,, Docker will show the first 12 digits only. 
    
    - To display all the 64 digits:

        ```bash
        docker image ls --no-trunc
        ```
- CREATED: Shows when the image was created.
- SIZE: Shows the size of the image.