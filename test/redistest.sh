echo "ping" | nc localhost 7001 && sleep 0.5
echo "ping" | nc localhost 7200 && sleep 0.5 
echo "ping" | nc localhost 7300 && sleep 0.5
echo "set tool hammer" | nc localhost 7400 && sleep 0.5
echo "set cookie secretcookie" | nc localhost 7001 && sleep 0.5
echo "get tool" | nc localhost 7300 && sleep 0.5
echo "get cookie" | nc localhost 7400 && sleep 0.5
echo "mget tool  cookie" | nc localhost 7200
