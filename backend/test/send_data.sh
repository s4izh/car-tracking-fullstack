#!/bin/sh

curl -X POST -H "Content-Type: application/json" -d @data.json http://localhost:3000/api/mobile/send-data  -i
