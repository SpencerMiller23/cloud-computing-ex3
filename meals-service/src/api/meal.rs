use actix_web::{
    get,
    post,
    delete,
    put,
    web::{Path, Header, Data, Payload, BytesMut},
    HttpResponse,
    Error,
    http::header::ContentType,
};

use futures::StreamExt;

use serde::Deserialize;

use crate::repository::state::{AppState, Meal};

#[derive(Deserialize, Debug)]
pub struct CreateMealRequest {
    name: String,
    appetizer: i32,
    main: i32,
    dessert: i32
}

#[post("/meals")]
pub async fn create_meal(content_type: Header<ContentType>, mut payload: Payload, state: Data<AppState>) -> Result<HttpResponse, Error> {
    if content_type.to_string() != String::from("application/json") {
        return Ok(
            HttpResponse::UnsupportedMediaType()
                .content_type(ContentType::json())
                .json(0)
        )
    }

    let data: CreateMealRequest;
    let mut body = BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        body.extend_from_slice(&chunk);
    }

    match serde_json::from_slice::<CreateMealRequest>(&body) {
        Ok(body) => data = body,
        _ => {
            return Ok(
                HttpResponse::UnprocessableEntity()
                    .content_type(ContentType::json())
                    .json(-1)
            )
        }
    }

    let meal_id = state.create_meal(data.name, &data.appetizer, &data.main, &data.dessert);

    match meal_id {
        Ok(id) => {
            Ok(
                HttpResponse::Created()
                    .content_type(ContentType::json())
                    .json(id)
            )
        },
        Err(err_id) => {
            Ok(
                HttpResponse::UnprocessableEntity()
                    .content_type(ContentType::json())
                    .json(err_id)
            )
        }
    }
}

#[get("/meals")]
pub async fn get_meals(state: Data<AppState>) -> Result<HttpResponse, Error> {
    let meals = state.get_meals();

    Ok(
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(meals)
    )
}

#[delete("/meals")]
pub async fn delete_meals() -> HttpResponse {
    HttpResponse::MethodNotAllowed()
        .content_type(ContentType::json())
        .json(String::from("Not implemented"))
}

#[get("/meals/{name}")]
pub async fn get_meal(path: Path<String>, state: Data<AppState>) -> Result<HttpResponse, Error> {
    let name = path.into_inner();

    let meal: Result<Meal, i32>;

    match name.parse::<i32>() {
        Ok(id) => meal = state.get_meal_by_id(id),
        Err(_) => meal = state.get_meal_by_name(&name)
    }

    match meal {
        Ok(data) => {
            return Ok(
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(data)
            )
        }
        _ => {
            return Ok(
                HttpResponse::NotFound()
                    .content_type(ContentType::json())
                    .json(-5)
            )
        }
    }
}

#[delete("/meals/{name}")]
pub async fn delete_meal(path: Path<String>, state: Data<AppState>) -> Result<HttpResponse, Error> {
    let name = path.into_inner();

    let res: Result<i32, i32>;

    match name.parse::<i32>() {
        Ok(id) => res = state.delete_meal_by_id(&id),
        Err(_) => res = state.delete_meal_by_name(&name)
    }

    match res {
        Ok(id) => {
            return Ok(
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(id)
            )
        },
        Err(_) => {
            return Ok(
                HttpResponse::NotFound()
                    .content_type(ContentType::json())
                    .json(-5)
            )
        }
    }
}

#[put("/meals/{ID}")]
pub async fn update_meal(path: Path<i32>, content_type: Header<ContentType>, mut payload: Payload, state: Data<AppState>) -> Result<HttpResponse, Error> {
    if content_type.to_string() != String::from("application/json") {
        return Ok(
            HttpResponse::UnsupportedMediaType()
                .content_type(ContentType::json())
                .json(0)
        )
    }

    let data: CreateMealRequest;
    let mut body = BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        body.extend_from_slice(&chunk);
    }

    match serde_json::from_slice::<CreateMealRequest>(&body) {
        Ok(body) => data = body,
        _ => {
            return Ok(
                HttpResponse::UnprocessableEntity()
                    .content_type(ContentType::json())
                    .json(-1)
            )
        }
    }
    
    let meal_id: i32 = path.into_inner();

    let res = state.update_meal(&meal_id, &data.name, &data.appetizer, &data.main, &data.dessert);

    match res {
        Ok(id) => {
            Ok(
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(id)
            )
        },
        Err(err_id) => {
            Ok(
                HttpResponse::NotFound()
                    .content_type(ContentType::json())
                    .json(err_id)
            )
        }
    }
}
