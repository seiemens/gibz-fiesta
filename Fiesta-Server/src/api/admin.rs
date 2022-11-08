use crate::{
    data::mongo_connector::Connector,
    helpers::{biscuit::biscuit, endecr, token},
    models::{skill_model::Skill, user_model::User},
};
use argon2::Error;
use mongodb::results::InsertOneResult;
use rocket::{
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
    Request, Response, State,
};
