#!/bin/bash

get_all_stats() {
    awk '/:/ && !/lo/ {sub(/:/, ""); print $1, $2, $10}' /proc/net/dev
}

clear
echo "=================================================="
echo "  Titanium Multi-Network Monitor v0.2 (Shell)     "
echo "=================================================="

declare -A RX_OLD
declare -A TX_OLD

while read -r IFACE RX TX; do
    if [ ! -z "$IFACE" ]; then
        RX_OLD[$IFACE]=$RX
        TX_OLD[$IFACE]=$TX
    fi
done <<< "$(get_all_stats)"

sleep 1

while true; do
    clear
    echo "=================================================="
    echo "  Titanium Multi-Network Monitor v0.2 (Shell)     "
    echo "=================================================="
    printf " %-15s | %-14s | %-14s\n" "Ethernet" "DOWNLOAD(RX)" "UPLOAD(TX)"
    echo "--------------------------------------------------"

    while read -r IFACE RX_NEW TX_NEW; do
        if [ ! -z "$IFACE" ]; then
            RX_PREV=${RX_OLD[$IFACE]}
            TX_PREV=${TX_OLD[$IFACE]}

            RX_SPEED=$(( (RX_NEW - RX_PREV) / 1024 ))
            TX_SPEED=$(( (TX_NEW - TX_PREV) / 1024 ))

            BARS=$(( RX_SPEED / 100 ))
            if [ $BARS -gt 15 ]; then BARS=15; fi
            
            GRAPH=""
            for ((i=0; i<BARS; i++)); do GRAPH+="#"; done
            for ((i=BARS; i<15; i++)); do GRAPH+=" "; done

            printf " %-15s | %6d KB/s     | %6d KB/s\n" "$IFACE" "$RX_SPEED" "$TX_SPEED"
            printf "                 | GRAPH: [%s]\n" "$GRAPH"
            echo "--------------------------------------------------"

            RX_OLD[$IFACE]=$RX_NEW
            TX_OLD[$IFACE]=$TX_NEW
        fi
    done <<< "$(get_all_stats)"

    echo " [ press Ctrl+C to exit ]"
    sleep 1
done
