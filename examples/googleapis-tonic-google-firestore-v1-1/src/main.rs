use anyhow::Context as _;
use firestore_path::DatabaseName;
use googleapis_tonic_google_firestore_v1::google::firestore::v1::GetDocumentRequest;
use std::sync::Arc;

type MyInterceptor =
    Box<dyn FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> + Send + Sync>;
type Client =
    googleapis_tonic_google_firestore_v1::google::firestore::v1::firestore_client::FirestoreClient<
        tonic::service::interceptor::InterceptedService<tonic::transport::Channel, MyInterceptor>,
    >;

#[derive(Debug, thiserror::Error)]
#[error("error")]
pub struct Error;

#[derive(Clone)]
pub struct FirestoreClient {
    channel: tonic::transport::Channel,
    database_name: firestore_path::DatabaseName,
    token_source: Arc<dyn google_cloud_token::TokenSource>,
}

impl FirestoreClient {
    pub async fn new() -> anyhow::Result<Self> {
        let default_token_source_provider =
            google_cloud_auth::token::DefaultTokenSourceProvider::new(
                google_cloud_auth::project::Config::default().with_scopes(&[
                    "https://www.googleapis.com/auth/cloud-platform",
                    "https://www.googleapis.com/auth/datastore",
                ]),
            )
            .await?;
        let token_source =
            google_cloud_token::TokenSourceProvider::token_source(&default_token_source_provider);
        let project_id = default_token_source_provider
            .project_id
            .context("project_id not found")?;
        let channel = tonic::transport::Channel::from_static("https://firestore.googleapis.com")
            .tls_config(tonic::transport::ClientTlsConfig::new().with_webpki_roots())?
            .connect()
            .await?;
        let database_name = DatabaseName::from_project_id(project_id)?;
        Ok(Self {
            channel,
            database_name,
            token_source,
        })
    }

    pub async fn client(&self) -> Result<Client, Error> {
        let inner = self.channel.clone();
        let token = self.token_source.token().await.map_err(|_| Error)?;
        let mut metadata_value =
            tonic::metadata::AsciiMetadataValue::try_from(token).map_err(|_| Error)?;
        metadata_value.set_sensitive(true);
        let interceptor: MyInterceptor = Box::new(
            move |mut request: tonic::Request<()>| -> Result<tonic::Request<()>, tonic::Status> {
                request
                    .metadata_mut()
                    .insert("authorization", metadata_value.clone());
                Ok(request)
            },
        );
        let client = googleapis_tonic_google_firestore_v1::google::firestore::v1::firestore_client::FirestoreClient::with_interceptor(inner, interceptor);
        Ok(client)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let firestore_client = FirestoreClient::new().await?;
    let mut client = firestore_client.client().await?;
    let document = client
        .get_document(GetDocumentRequest {
            consistency_selector: None,
            mask: None,
            name: firestore_client
                .database_name
                .collection("events")?
                .doc("0abd6889-8fb9-4055-a1bf-2a8a42874496")?
                .to_string(),
        })
        .await?;
    println!("{:#?}", document);
    Ok(())
}
