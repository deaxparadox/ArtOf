# Confiuration of Logging Drivers

By default, Docker uses the json-file logging driver, which caches container logs as JSON internally. 

To cache the log, use the :

```bash
docker logs <Container ID or name>
```

There are other drivers that Docker can use, for example *splunk*, *journald*, and *none* to disable logging. 

To configure the logging driver, run the:

```bash
docker run -it --log-driver <log driver> <image>
```

To fetch the driver type, run:

```bash
docker inspect -f '{{.HostConfig.LogConfig.Type}}' <CONTAINER>
```