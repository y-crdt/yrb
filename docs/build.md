# Build Instructions

## Create a new CI image

The CI pipeline uses a custom image based on `rust:latest` to allow us creating
a multi-platform build. If you want to update the *builder* image, you can
follow the steps below. You need read/write access to the repository and
registry to do so.

```bash
docker login registry.gitlab.com

docker build \
  -f config/build/Dockerfile \
  -t registry.gitlab.com/gitlab-org/incubation-engineering/real-time-editing/yrb \
  .
  
docker push \
  registry.gitlab.com/gitlab-org/incubation-engineering/real-time-editing/yrb
```
