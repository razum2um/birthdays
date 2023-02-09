#!/bin/bash
set -euo pipefail

PORT="${1:-8000}"

curl -s -X PUT "http://localhost:${PORT}/hello/Vlad" -H "Content-Type: application/json" -d '{ "dateOfBirth": "1989-06-16" }'
curl "http://localhost:${PORT}/hello/Vlad"
echo
curl -s -X PUT "http://localhost:${PORT}/hello/Vlad" -H "Content-Type: application/json" -d '{ "dateOfBirth": "1989-06-15" }'
curl -s "http://localhost:${PORT}/hello/Vlad"
echo
curl -s "http://localhost:${PORT}/hello/Vlad?on=2023-06-14"
echo
