use crate::model::{CreateProposalRequest, SubmitVoteRequest};
use crate::services::{create_account, create_proposal, submit_vote};
use warp::http::StatusCode;
use warp::reply::Json;
use warp::Rejection;

pub async fn create_account_handler() -> Result<Json, Rejection> {
    match create_account().await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(_) => Err(warp::reject::custom(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub async fn create_proposal_handler(body: CreateProposalRequest) -> Result<Json, Rejection> {
    match create_proposal(body).await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(_) => Err(warp::reject::custom(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub async fn submit_vote_handler(body: SubmitVoteRequest) -> Result<Json, Rejection> {
    match submit_vote(body).await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(_) => Err(warp::reject::custom(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}
