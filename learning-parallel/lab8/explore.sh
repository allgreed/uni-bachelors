for i in {-10..10}
do
    echo -n "$i "
    ./main $i
    sleep 1
done
