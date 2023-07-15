use actix_web::{
    get,
    post,
    delete,
    web::{Path, Header, Data, Payload, BytesMut},
    HttpResponse,
    Error,
    http::header::ContentType,
};

use futures::StreamExt;

use serde::Deserialize;

use crate::repository::state::{AppState, Dish};

#[derive(Deserialize, Debug)]
pub struct CreateDishRequest {
    name: String,
}

#[get("/rebuild")] // Delete
pub async fn rebuild(state: Data<AppState>) -> HttpResponse {
    let id = state.rebuild();
    
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(id)
}

#[get("/dishes")]
pub async fn get_dishes(state: Data<AppState>) -> HttpResponse {
    let dishes = state.get_dishes();
    
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(dishes)
}

#[post("/dishes")]
pub async fn create_dish(content_type: Header<ContentType>, mut payload: Payload, state: Data<AppState>) -> Result<HttpResponse, Error> {
    if content_type.to_string() != String::from("application/json") {
        return Ok(
            HttpResponse::UnsupportedMediaType()
                .content_type(ContentType::json())
                .json(0)
        )
    }
    
    let data: CreateDishRequest;
    let mut body = BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        body.extend_from_slice(&chunk);
    }

    match serde_json::from_slice::<CreateDishRequest>(&body) {
        Ok(body) => data = body,
        _ => {
            return Ok(
                HttpResponse::UnprocessableEntity()
                    .content_type(ContentType::json())
                    .json(-1)
            )
        }
    }

    let dish_id = state.create_dish(data.name.clone()).await;

    match dish_id {
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

#[delete("/dishes")]
pub async fn delete_dishes() -> HttpResponse {
    HttpResponse::MethodNotAllowed()
        .content_type(ContentType::json())
        .json(String::from("Not implemented"))
}

#[get("/dishes/{name}")]
pub async fn get_dish(path: Path<String>, state: Data<AppState>) -> Result<HttpResponse, Error> {
    let name = path.into_inner();

    let dish: Result<Dish, i32>;

    match name.parse::<i32>() {
        Ok(id) => dish = state.get_dish_by_id(id),
        Err(_) => dish = state.get_dish_by_name(name)
    }

    match dish {
        Ok(data) => {
            return Ok(
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(data)
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

#[delete("/dishes/{name}")]
pub async fn delete_dish(path: Path<String>, state: Data<AppState>) -> Result<HttpResponse, Error> {
    let name = path.into_inner();

    match name.parse::<i32>() {
        Ok(id) => return delete_dish_by_id(id, state).await,
        Err(_) => return delete_dish_by_name(name, state).await
    }
}

async fn delete_dish_by_id(dish_id: i32, state: Data<AppState>) -> Result<HttpResponse, Error> {
    let res = state.delete_dish_by_id(dish_id);

    match res {
        Ok(id) => {
            return Ok(
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(id)
            )
        },
        Err(id) => {
            return Ok(
                HttpResponse::NotFound()
                    .content_type(ContentType::json())
                    .json(id)
            )
        }
    }
}

async fn delete_dish_by_name(name: String, state: Data<AppState>) -> Result<HttpResponse, Error> {
    let res = state.delete_dish_by_name(name);
    
    match res {
        Ok(id) => {
            return Ok(
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(id)
            )
        },
        Err(id) => {
            return Ok(
                HttpResponse::NotFound()
                    .content_type(ContentType::json())
                    .json(id)
            )
        }
    }
}