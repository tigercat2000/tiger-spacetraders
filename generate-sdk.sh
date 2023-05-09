#!/bin/bash

npx @openapitools/openapi-generator-cli generate \
    -i submodules/api-docs/reference/SpaceTraders.json \
    -o spacetraders-sdk \
    -g rust \
    --additional-properties=packageName=spacetraders-sdk
