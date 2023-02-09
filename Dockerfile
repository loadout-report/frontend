# Build Stage
FROM ghcr.io/loadout-report/spa-server:main

ENV ASSET_DIR=/assets

COPY dist $ASSET_DIR
