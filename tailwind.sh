#!/bin/bash
set -e
echo "Starting Tailwind..."
npx tailwindcss -i ./input.css -o ./app/resources/style/tailwind.css --watch &