#!bin/bash

docker build -t parser ../../ingredient-phrase-tagger/

docker run --name parser --rm -d -t parser bash

docker exec parser bash run.sh

docker cp parser:/app/temp/labeled temp/labeled

docker stop parser
