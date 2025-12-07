#!/bin/bash
case "$1" in
    start)
        echo "Starting server..."
        sudo systemctl enable --now template
        ;;
    stop)
    sudo systemctl stop template   # Replace with your service name

        ;;
    restart)
    sudo systemctl daemon-reload
    sudo systemctl restart template
    sudo systemctl status template
        ;;
    *)
        echo "Usage: $0 {start|stop|restart}"
        exit 1
esac
