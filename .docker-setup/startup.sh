#!/bin/bash

# enter your gokey-cURL folder path
cd D:/dev/gokey-cURL


# start up the docker engine 

# for windows, simply put your path to docker desktop, the default is the following:
"C:\Program Files\Docker\Docker\Docker Desktop.exe"

# for mac
# open -a Docker


# for mac
# wait for docker to start
while :
do
    if ! docker info 2>&1 | grep -q "ERROR"
    then
        break
    else
        sleep 1
    fi
done


# start up containers
docker-compose up


# for mac
# start up gokey-cURL
open http://localhost:1323
