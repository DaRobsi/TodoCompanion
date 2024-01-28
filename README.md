# Todo Companion
This is a RESTful ToDo webapp whoose backend is written in Rust and frontend is written in Sveltekit/Svelte. It will also use the GraphAPI for an integrated calendar.

# Startup
The project is at the moment started through a Dockerfile. This is not in the root directory of the repo, as it will be part of a docker-compose later.

```
cd backend/
docker build -t <image-name> .
docker run <image-name> -p 3000
```

