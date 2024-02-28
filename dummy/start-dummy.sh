#!/bin/bash

set -e;

cd ./dummy/8081 && python -m http.server 8081 &
cd ./dummy/8082 && python -m http.server 8082 & 
cd ./dummy/8083 && python -m http.server 8083 &
