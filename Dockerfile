FROM golang:1.19-alpine

WORKDIR /app

COPY go.mod ./
COPY go.sum ./
RUN go mod download

COPY *.go ./

RUN go build -o /securityevents-forwarder

# Run as arbitrary user id in openshift
RUN chgrp -R 0 /app && \
    chmod -R g=u /app

EXPOSE 8080

CMD [ "/securityevents-forwarder" ]