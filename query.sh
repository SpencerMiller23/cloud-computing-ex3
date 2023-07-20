while IFS= read -r line
do
    echo http://127.0.0.1:8000/dishes/${line::-3}
    echo {"name": "${line::-3}"}
    # curl -X GET -v --location --request --ipv4 "http://127.0.0.1:8000/dishes/${line::-3}"
done < query.txt
