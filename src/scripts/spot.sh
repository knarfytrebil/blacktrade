#!bin/bash
while true
do 
    echo "Started\n"
    curl https://shapeshift.io/rate/btc_eth &&
    curl https://index.bitcoin.com/api/v0/price/usd 
done
