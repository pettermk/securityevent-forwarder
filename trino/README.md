# Reading events with trino

Trino can read events from the event hub. This folder contains trino configuration
that works with event hubs, given an event hub connection string. The events will
not be marked as pulled off the hub.

## Starting trino

Start trino on command line

```
docker run --name trino -d -p 8080:8080 --volume $(pwd)/etc:/etc/trino --env-file ./.env trinodb/trino
```

Execute trino

```
docker exec -it trino trino
```

Then execute query, e.g.
```
select _message from kafka.default."snyk-events";
```
