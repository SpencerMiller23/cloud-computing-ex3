# Meals Service

Service for computing and storing nutritional data for meals. Built using [Actix Web](https://actix.rs/) and uses the [API Ninjas Nutrition API](https://api-ninjas.com/api/nutrition) to compute nutritional values.

## Running With Docker

- Build docker image: `docker build -f ./Dockerfile -t meals-service .`
- Run container: `docker run -p 8000:8000 meals-service`
- Send requests: `http://localhost:8000/`

## API Docs

### Dishes

#### GET `/dishes`

Returns a JSON object listing all dishes, indexed by ID

Response status codes:

| Status code | Description |
|-------------|-------------|
| 200 | Success |

Response body parameters:

| Parameter | Type | Description |
|-----------|------|-------------|
| name | String | Name of the dish |
| ID | Integer | Dish ID |
| cal | Float | Number of calories |
| size | Float | Serving size in grams |
| sodium | Float | Amount of sodium in mg |
| sugar | Float | Amount of sugar in grams |

Example response body:

```
{
    "1": {
        "name": "pasta",
        "ID": 1,
        "cal": 500.0,
        "size": 150.0,
        "sodium": 12.0,
        "sugar": 1.0
    }
}
```

#### POST `/dishes`

Creates a dish with the given name and returns the ID of the new dish

Request body parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| name | String | True | Name of the dish |

Example request body:

```
{
    "name": "pasta"
}
```

Response status codes:

| Status code | Description |
|-------------|-------------|
| 201 | Created |
| 415 | Unsupported Media Type |
| 422 | Unprocessable Content |
| 504 | Gateway Timeout |

Response body parameters:

| ID | Description |
|----|-------------|
| >= 1 | New dish was created |
| 0 | Content-Type is not `application/json` |
| -1 | `name` parameter was not specified in the request body |
| -2 | A dish with the given name already exists |
| -3 | Nutrition API does not recognize the name of the dish |
| -4 | Nutrition API was not reachable |

#### GET `/dishes/{ID}`

Returns the name and nutrition information for the corresponding dish. Returns -5 if no corresponding dish exists

Request path parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ID | Integer | True | ID of the dish |

Response status codes:

| Status code | Description |
|-------------|-------------|
| 200 | Ok |
| 404 | Not found |

Response body parameters:

| Parameter | Type | Description |
|-----------|------|-------------|
| name | String | Name of the dish |
| ID | Integer | Dish ID |
| cal | Float | Number of calories |
| size | Float | Serving size in grams |
| sodium | Float | Amount of sodium in mg |
| sugar | Float | Amount of sugar in grams |

Example response body:

```
{
    "name": "pasta",
    "ID": 1,
    "cal": 500.0,
    "size": 150.0,
    "sodium": 12.0,
    "sugar": 1.0
}
```

#### DELETE `/dishes/{ID}`

Removes the dish corresponding to the given ID from storage

Request path parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ID | Integer | True | ID of the dish |

Response status codes:

| Status code | Description |
|-------------|-------------|
| 200 | Ok |
| 404 | Not found |

Response body parameters:

| ID | Description |
|----|-------------|
| >= 1 | Dish was deleted |
| -5 | Dish not found |

#### GET `/dishes/{name}`

Returns the name and nutrition information for the corresponding dish. Returns -5 if no corresponding dish exists

Request path parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| name | String | True | Name of the dish |

Response status codes:

| Status code | Description |
|-------------|-------------|
| 200 | Ok |
| 404 | Not found |

Response body parameters:

| Parameter | Type | Description |
|-----------|------|-------------|
| name | String | Name of the dish |
| ID | Integer | Dish ID |
| cal | Float | Number of calories |
| size | Float | Serving size in grams |
| sodium | Float | Amount of sodium in mg |
| sugar | Float | Amount of sugar in grams |

Example response body:

```
{
    "name": "pasta",
    "ID": 1,
    "cal": 500.0,
    "size": 150.0,
    "sodium": 12.0,
    "sugar": 1.0
}
```

#### DELETE `/dishes/{name}`

Removes the dish from storage

Request path parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| name | String | True | Name of the dish |

Response status codes:

| Status code | Description |
|-------------|-------------|
| 200 | Ok |
| 404 | Not found |

Response body parameters:

| ID | Description |
|----|-------------|
| >= 1 | Dish was deleted |
| -5 | Dish not found |

### Meals

#### POST `/meals`

Creates a meal with the given name, appetizer ID, main ID and dessert ID, and returns the ID of the new meal

Request body parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| name | String | True | Name of the meal |
| appetizer | Integer | True | ID of the appetizer dish |
| main | Integer | True | ID of the main dish |
| dessert | Integer | True | ID of the dessert dish |

Example request body:

```
{
    "name": "italian dinner",
    "appetizer": 1,
    "main": 2,
    "dessert": 3
}
```

Response status codes:

| Status code | Description |
|-------------|-------------|
| 201 | Created |
| 415 | Unsupported Media Type |
| 422 | Unprocessable Content |

Response body parameters:

| ID | Description |
|----|-------------|
| >= 1 | New dish was created |
| 0 | Content-Type is not `application/json` |
| -1 | At least one required parameter was not specified in the request body |
| -2 | A meal with the given name already exists |
| -6 | At least one of the dish IDs given does not correspond to a dish |

#### GET `/meals`

Returns a JSON object listing all meals, indexed by ID

Response status codes:

| Status code | Description |
|-------------|-------------|
| 200 | Success |

Response body parameters:

| Parameter | Type | Description |
|-----------|------|-------------|
| name | String | Name of the dish |
| ID | Integer | Meal ID |
| appetizer | Integer | Appetizer dish ID |
| main | Integer | Main dish ID |
| dessert | Integer | Dessert dish ID |
| cal | Float | Number of calories |
| size | Float | Serving size in grams |
| sodium | Float | Amount of sodium in mg |
| sugar | Float | Amount of sugar in grams |

Example response body:

```
{
    "1": {
        "name": "italian dinner",
        "ID": 1,
        "appetizer": 1,
        "main": 2,
        "dessert": 3,
        "cal": 784.60004,
        "sodium": 812.0,
        "sugar": 3.8
    }
}
```

#### GET `/meals/{ID}`

Returns the name and nutrition information for the corresponding meal. Returns -5 if no corresponding meal exists

Request path parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ID | Integer | True | ID of the meal |

Response status codes:

| Status code | Description |
|-------------|-------------|
| 200 | Ok |
| 404 | Not found |

Response body parameters:

| Parameter | Type | Description |
|-----------|------|-------------|
| name | String | Name of the dish |
| ID | Integer | Meal ID |
| appetizer | Integer | Appetizer dish ID |
| main | Integer | Main dish ID |
| dessert | Integer | Dessert dish ID |
| cal | Float | Number of calories |
| size | Float | Serving size in grams |
| sodium | Float | Amount of sodium in mg |
| sugar | Float | Amount of sugar in grams |

Example response body:

```
{
    "name": "italian dinner",
    "ID": 1,
    "appetizer": 1,
    "main": 2,
    "dessert": 3,
    "cal": 784.60004,
    "sodium": 812.0,
    "sugar": 3.8
}
```

#### DELETE `/meals/{ID}`

Removes the meal corresponding to the given ID from storage

Request path parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ID | Integer | True | ID of the meal |

Response status codes:

| Status code | Description |
|-------------|-------------|
| 200 | Ok |
| 404 | Not found |

Response body parameters:

| ID | Description |
|----|-------------|
| >= 1 | Meal was deleted |
| -5 | Meal not found |

#### GET `/meals/{name}`

Returns the name and nutrition information for the corresponding meal. Returns -5 if no corresponding meal exists

Request path parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| name | String | True | Name of the meal |

Response status codes:

| Status code | Description |
|-------------|-------------|
| 200 | Ok |
| 404 | Not found |

Response body parameters:

| Parameter | Type | Description |
|-----------|------|-------------|
| name | String | Name of the dish |
| ID | Integer | Meal ID |
| appetizer | Integer | Appetizer dish ID |
| main | Integer | Main dish ID |
| dessert | Integer | Dessert dish ID |
| cal | Float | Number of calories |
| size | Float | Serving size in grams |
| sodium | Float | Amount of sodium in mg |
| sugar | Float | Amount of sugar in grams |

Example response body:

```
{
    "name": "italian dinner",
    "ID": 1,
    "appetizer": 1,
    "main": 2,
    "dessert": 3,
    "cal": 784.60004,
    "sodium": 812.0,
    "sugar": 3.8
}
```

#### DELETE `/meals/{name}`

Removes the meal from storage

Request path parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| name | String | True | Name of the meal |

Response status codes:

| Status code | Description |
|-------------|-------------|
| 200 | Ok |
| 404 | Not found |

Response body parameters:

| ID | Description |
|----|-------------|
| >= 1 | Meal was deleted |
| -5 | Meal not found |

#### PUT `/meals/{ID}`

Creates a dish with the given name and returns the ID of the new dish

Request path parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ID | Integer | True | ID of the meal |

Request body parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| name | String | True | Updated name of the meal |
| appetizer | Integer | True | Updated ID of the appetizer dish |
| main | Integer | True | Updated ID of the main dish |
| dessert | Integer | True | Updated ID of the dessert dish |

Example request body:

```
{
    "name": "healthy dinner",
    "appetizer": 4,
    "main": 1,
    "dessert": 3
}
```

Response status codes:

| Status code | Description |
|-------------|-------------|
| 201 | Created |
| 415 | Unsupported Media Type |
| 422 | Unprocessable Content |

Response body parameters:

| ID | Description |
|----|-------------|
| >= 1 | New dish was created |
| 0 | Content-Type is not `application/json` |
| -1 | At least one required parameter was not specified in the request body |
| -2 | A meal with the given name already exists |
| -6 | At least one of the dish IDs given does not correspond to a dish |