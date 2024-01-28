#!/bin/bash
BIN_PATH="./rss_youtube_api"

while true; do
    $BIN_PATH &
    BIN_PID=$!

    # check if the process is still running
    while kill -0 $BIN_PID 2>/dev/null; do
        # process is still running
        sleep 1
    done

    # if we get here, the process has exited
    echo "Process exited. Restarting..."
done
