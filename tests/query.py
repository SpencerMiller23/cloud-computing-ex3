import requests

MEALS_SERVICE_BASE_URL = 'http://127.0.0.1:8000'

if __name__ == '__main__':
    output = []

    with open('../query.txt', 'r') as f:
        lines = f.read().splitlines()

    lines = [line[:-2] for line in lines]

    for line in lines:
        res = requests.post(
            f'{MEALS_SERVICE_BASE_URL}/dishes',
            json={'name': line},
            headers={'Content-Type': 'application/json'}
        )

        res = requests.get(f'{MEALS_SERVICE_BASE_URL}/dishes/{line}')

        res = res.json()

        output.append(f'{line} contains {res["cal"]} calories, {res["sodium"]} mgs of sodium, and {res["sugar"]} grams of sugar')
    
    with open('../response.txt', 'w') as f:
        for s in output:
            f.write(s)
            f.write('\n')