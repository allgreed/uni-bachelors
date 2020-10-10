if [ -z "$1" ]
then
     	ble=5
else
	ble=$1
fi

echo $$
((ble=ble-1))
sleep 2

if [ "$ble" -gt 0 ]
then
	$0 $ble &
	$0 $ble &
fi
