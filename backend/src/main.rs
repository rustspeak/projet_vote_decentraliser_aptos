mod handlers;
mod model;
mod services;

use warp::Filter;

#[tokio::main]
async fn main() {
    let create_account = warp::path("create_account")
        .and(warp::post())
        .and_then(handlers::create_account_handler);

    let create_proposal = warp::path("create_proposal")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::create_proposal_handler);

    let submit_vote = warp::path("submit_vote")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::submit_vote_handler);

    let routes = create_account.or(create_proposal).or(submit_vote);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
