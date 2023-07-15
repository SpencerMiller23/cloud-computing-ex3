import requests

MEALS_SERVICE_BASE_URL = 'http://127.0.0.1:8000'

def test_sanity_check():
    res = requests.get(MEALS_SERVICE_BASE_URL)

    assert res.json() == 'OK'