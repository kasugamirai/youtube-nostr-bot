#!/bin/bash
BIN_PATH="./youtube_bot_run"

while true; do
    $BIN_PATH &
    BIN_PID=$!

    sleep 86400

    kill $BIN_PID

    sleep 5
done
