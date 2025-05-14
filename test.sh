#!/bin/bash

VARS=$(export | grep SRANDR)

echo "${VARS}"

if [[ $SRANDRD_EVENT == "connected" ]]
then
    echo "Switching"
    xrandr --output HDMI-2 --primary

    i3-msg workspace 2
    i3-msg move workspace to output primary

    i3-msg workspace 3
    i3-msg move workspace to output primary

    i3-msg workspace 4
    i3-msg move workspace to output primary

    i3-msg workspace 5
    i3-msg move workspace to output primary

    i3-msg workspace 6
    i3-msg move workspace to output primary

    i3-msg workspace 7
    i3-msg move workspace to output primary

    i3-msg workspace 8
    i3-msg move workspace to output primary

    i3-msg workspace 9
    i3-msg move workspace to output primary

    i3-msg workspace 1
    i3-msg move workspace to output primary

fi 