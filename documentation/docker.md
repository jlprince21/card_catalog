# Docker Instructions for PostgreSQL

Note: it is possible that when setting up pgAdmin that if you don't name the
PostgreSQL container with matching `name` and `hostname` (used postgres below)
that adding the server will fail. Make them match to be safe!

Emails and passwords that need replacing are in `<>` below.

```bash
# common network for both containers
docker network create --driver bridge pgnetwork

# volumes for PostgreSQL and pgAdmin data
docker volume create --driver local --name=pgvolume
docker volume create --driver local --name=pga4volume

# build PostgreSQL container
docker run --publish 5432:5432 \
--volume=pgvolume:/pgdata \
--name postgres \
--env POSTGRES_PASSWORD=<PASSWORD> \
--network=pgnetwork \
--hostname=postgres \
--detach \
postgres

# build pgAdmin container
docker run --publish 80:80 \
--volume=pga4volume:/var/lib/pgadmin \
--name pgadmin4 \
--env "PGADMIN_DEFAULT_EMAIL=<EMAIL>" \
--env "PGADMIN_DEFAULT_PASSWORD=<PASSWORD>" \
--hostname=pgadmin4 \
--network=pgnetwork \
--detach \
dpage/pgadmin4
```

Containers should start back up with:
```
docker start postgres pgadmin4
```