#!/bin/bash
case "$1" in
    start)
        echo "Starting server..."
        sudo systemctl enable --now multiplayer-tierlist
        ;;
    stop)
    sudo systemctl stop multiplayer-tierlist   # Replace with your service name

        ;;
    restart)
    sudo systemctl daemon-reload
    sudo systemctl restart multiplayer-tierlist
    sudo systemctl status multiplayer-tierlist
        ;;
    *)
        echo "Usage: $0 {start|stop|restart}"
        exit 1
esac
