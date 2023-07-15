import requests

MEALS_SERVICE_BASE_URL = 'http://127.0.0.1:8000'

def test_post_new_dishes():
    id_list = []

    res = requests.post(
        f'{MEALS_SERVICE_BASE_URL}/dishes',
        json={'name': 'orange'},
        headers={'Content-Type': 'application/json'}
    )

    assert res.status_code == 201
    assert res.json() not in id_list

    id_list.append(res.json())

    res = requests.post(
        f'{MEALS_SERVICE_BASE_URL}/dishes',
        json={'name': 'spaghetti'},
        headers={'Content-Type': 'application/json'}
    )

    assert res.status_code == 201
    assert res.json() not in id_list

    id_list.append(res.json())

    res = requests.post(
        f'{MEALS_SERVICE_BASE_URL}/dishes',
        json={'name': 'apple pie'},
        headers={'Content-Type': 'application/json'}
    )

    assert res.status_code == 201
    assert res.json() not in id_list

def test_get_dish_by_id():
    res = requests.get(f'{MEALS_SERVICE_BASE_URL}/dishes/1')

    assert res.status_code == 200
    assert 0.9 <= res.json()['sodium'] <= 1.1

def test_get_dishes():
    res = requests.get(f'{MEALS_SERVICE_BASE_URL}/dishes')

    assert res.status_code == 200
    
    dishes = res.json()

    assert len(dishes.keys()) == 3

    for dish in dishes.values():
        assert type(dish) == dict

def test_post_invalid_dish():
    res = requests.post(
        f'{MEALS_SERVICE_BASE_URL}/dishes',
        json={'name': 'blah'},
        headers={'Content-Type': 'application/json'}
    )

    assert res.status_code in [404, 400, 422]
    assert res.json() == -3

def test_post_duplicate_dish():
    res = requests.post(
        f'{MEALS_SERVICE_BASE_URL}/dishes',
        json={'name': 'orange'},
        headers={'Content-Type': 'application/json'}
    )

    assert res.status_code in [404, 400, 422]
    assert res.json() == -2

def test_post_new_meal():
    res = requests.post(
        f'{MEALS_SERVICE_BASE_URL}/meals',
        json={
            'name': 'delicious',
            'appetizer': 1,
            'main': 2,
            'dessert': 3
        },
        headers={'Content-Type': 'application/json'}
    )

    assert res.status_code == 201
    assert res.json() > 0

def test_get_meals():
    res = requests.get(f'{MEALS_SERVICE_BASE_URL}/meals')

    assert res.status_code == 200
    
    dishes = res.json()

    assert len(dishes.keys()) == 1
    assert 400 <= dishes['1']['cal'] <= 500

def test_post_duplicate_meal():
    res = requests.post(
        f'{MEALS_SERVICE_BASE_URL}/meals',
        json={
            'name': 'delicious',
            'appetizer': 1,
            'main': 2,
            'dessert': 3
        },
        headers={'Content-Type': 'application/json'}
    )

    assert res.status_code in [400, 422]
    assert res.json() == -2