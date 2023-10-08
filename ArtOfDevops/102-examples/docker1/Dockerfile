FROM ubuntu

ARG user=normal

RUN adduser --disabled-password ${user}

WORKDIR /home/${user}/app

COPY  . .

ENTRYPOINT [ "bash", "entrypoint.sh" ]