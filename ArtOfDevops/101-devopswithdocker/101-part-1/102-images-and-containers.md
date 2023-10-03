# Image and containers

Containers are instances of images. A basic mistake is to confuse images and containers.

In short, an image is like a blueprint or template, while a container is an instance of that blueprint or template.

## Image

A Docker image is a file. An image never changes; you can not edit an existing file. Creating a new image happens by starting from a base image and adding new **layers** to it. We will talk about layers later, but you should think of images as *immutable*, they can not be changed after they are created.

