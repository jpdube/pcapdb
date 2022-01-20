use dropshot::endpoint;
use dropshot::ApiDescription;
use dropshot::HttpError;
use dropshot::HttpResponseOk;
use dropshot::RequestContext;
// use http::Method;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use dropshot::ConfigDropshot;
use dropshot::ConfigLogging;
use dropshot::ConfigLoggingLevel;
use dropshot::HttpServerStarter;
use dropshot::TypedBody;


/** Represents a project in our API */
#[derive(Serialize, JsonSchema)]
struct Project {
    name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct Query {
    query: String,
}

#[endpoint {
    method = POST,
    path = "/api/query",
}]
async fn exec_query(_rqctx: Arc<RequestContext<()>>, body_param: TypedBody<Query>,) -> Result<HttpResponseOk<Project>, HttpError>
{
    let query = body_param.into_inner();
    let project = Project { name: format!("Got query: {}", query.query)};
    println!("Query body: {:?}", query);

    Ok(HttpResponseOk(project))
}

#[endpoint {
    method = GET,
    path = "/api/query",
}]
async fn myapi_projects_get_project(_rqctx: Arc<RequestContext<()>>,) -> Result<HttpResponseOk<Project>, HttpError>
{
   let project = Project { name: String::from("project1") };
   Ok(HttpResponseOk(project))
}

#[tokio::main]
async fn start() -> Result<(), String> {
    // Set up a logger.
    let log =
        ConfigLogging::StderrTerminal {
            level: ConfigLoggingLevel::Info,
        }
        .to_logger("minimal-example")
        .map_err(|e| e.to_string())?;

    // Describe the API.
    let mut api = ApiDescription::new();
    api.register(myapi_projects_get_project).unwrap();
    api.register(exec_query).unwrap();
    // Register API functions -- see detailed example or ApiDescription docs.

    // Start the server.
    let server =
        HttpServerStarter::new(
            &ConfigDropshot {
                bind_address: "127.0.0.1:8080".parse().unwrap(),
                request_body_max_bytes: 1024,
            },
            api,
            (),
            &log,
        )
        .map_err(|error| format!("failed to start server: {}", error))?
        .start();

    server.await
}


pub fn main() {
    start().unwrap();
}
