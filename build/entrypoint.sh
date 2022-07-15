#!/usr/bin/env bash

/bin/bash -l -c "rvm --default use $RUBY_VERSION" && exec "$@"
