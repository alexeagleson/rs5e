#!/bin/bash

echo "Building types..."
./typeshare.sh

echo "Building client..."
cd client
npm run build
cd ..

if [ $1 == "release" ]
then
    echo "Building debug server..."
    cargo run --release 127.0.0.1:8080
else 
    echo "Building production server..."
    cargo run 127.0.0.1:8080
fi