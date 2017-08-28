#!/bin/sh

if [ "$TRAVIS_PULL_REQUEST" = "true" ]; then
	exit 0
fi

if [ ! -z "$TRAVIS_TAG" ] && [ "$TRAVIS_BRANCH" != "master" ] ; then
	exit 0
fi

if [ "$TRAVIS_BRANCH" = "master" ]; then
	TAG="latest"
else
	TAG="$TRAVIS_TAG"
fi

REPO="garyttierney/tripledb"
IMAGE="$REPO:$TRAVIS_COMMIT"

docker build -f Dockerfile -t "$IMAGE" .
docker tag "$IMAGE" "$REPO:$TAG"
docker push "$REPO"

