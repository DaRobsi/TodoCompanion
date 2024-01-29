# Todo Companion
This is a RESTful ToDo webapp whoose backend is written in Rust and frontend is written in Sveltekit/Svelte. It will also use the GraphAPI for an integrated calendar.

# Startup
The project is started through the docker-compose.yaml in the root directory, from there the Dockerfiles for the respective services are executed. At the moment, this is only the Rust backend. In the root directory, a .env has to be created with `GRAPH_TOKEN` containing your token to the graph API, as this is not automated yet.

After that, the container can be started:
```
docker-compose up
```

