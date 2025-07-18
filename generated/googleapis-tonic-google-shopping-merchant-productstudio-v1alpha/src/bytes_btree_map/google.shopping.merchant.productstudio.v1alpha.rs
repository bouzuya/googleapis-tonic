// This file is @generated by prost-build.
/// Represents an input image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputImage {
    /// The input image.
    #[prost(oneof = "input_image::Image", tags = "1, 2")]
    pub image: ::core::option::Option<input_image::Image>,
}
/// Nested message and enum types in `InputImage`.
pub mod input_image {
    /// The input image.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Image {
        /// Public uri of the image.
        #[prost(string, tag = "1")]
        ImageUri(::prost::alloc::string::String),
        /// Raw image bytes.
        #[prost(bytes, tag = "2")]
        ImageBytes(::prost::bytes::Bytes),
    }
}
/// Request message for the GenerateProductImageBackground method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProductImageBackgroundRequest {
    /// Required. The account for which to generate an image. This acts as a
    /// container for the request and does not affect the generation itself.
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Configuration for how the output image should be returned.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<OutputImageConfig>,
    /// Required. The input image.
    #[prost(message, optional, tag = "3")]
    pub input_image: ::core::option::Option<InputImage>,
    /// Required. Configuration parameters for the generation of the background.
    #[prost(message, optional, tag = "4")]
    pub config: ::core::option::Option<GenerateImageBackgroundConfig>,
}
/// Response message for the GenerateProductImageBackground method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProductImageBackgroundResponse {
    /// The generated output image.
    #[prost(message, optional, tag = "1")]
    pub generated_image: ::core::option::Option<GeneratedImage>,
}
/// Request message for the RemoveProductImageBackground method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProductImageBackgroundRequest {
    /// Required. The account for which to generate an image. This acts as a
    /// container for the request and does not affect the generation itself.
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Configuration for how the output image should be returned.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<OutputImageConfig>,
    /// Required. The input image.
    #[prost(message, optional, tag = "3")]
    pub input_image: ::core::option::Option<InputImage>,
    /// Optional. Configuration parameters for the removal of the background.
    #[prost(message, optional, tag = "4")]
    pub config: ::core::option::Option<RemoveImageBackgroundConfig>,
}
/// Response message for the RemoveProductImageBackground method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProductImageBackgroundResponse {
    /// The generated output image.
    #[prost(message, optional, tag = "1")]
    pub generated_image: ::core::option::Option<GeneratedImage>,
}
/// Request message for the UpscaleProductImage method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpscaleProductImageRequest {
    /// Required. The account for which to generate an image. This acts as a
    /// container for the request and does not affect the generation itself.
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Configuration for how the output image should be returned.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<OutputImageConfig>,
    /// Required. The input image.
    #[prost(message, optional, tag = "3")]
    pub input_image: ::core::option::Option<InputImage>,
}
/// Response message for the UpscaleProductImage method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpscaleProductImageResponse {
    /// The generated output image.
    #[prost(message, optional, tag = "1")]
    pub generated_image: ::core::option::Option<GeneratedImage>,
}
/// Represents a generated image object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneratedImage {
    /// Identifier. The unique key for the image.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The timestamp when the image was generated.
    #[prost(message, optional, tag = "4")]
    pub generation_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The generated image.
    #[prost(oneof = "generated_image::Image", tags = "2, 3")]
    pub image: ::core::option::Option<generated_image::Image>,
}
/// Nested message and enum types in `GeneratedImage`.
pub mod generated_image {
    /// The generated image.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Image {
        /// Generally web-requestable URI of the generated image. This is a temporary
        /// URI and will expire after 6 months. A URI may not be populated
        /// immediately after generation. Use get or list api using image_id to get
        /// the URI.
        #[prost(string, tag = "2")]
        Uri(::prost::alloc::string::String),
        /// Raw bytes for the image.
        #[prost(bytes, tag = "3")]
        ImageBytes(::prost::bytes::Bytes),
    }
}
/// Configuration for how the output image should be returned.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct OutputImageConfig {
    /// Optional. If true, returns the output images as serving uris instead of
    /// bytes.
    #[prost(bool, tag = "1")]
    pub return_image_uri: bool,
}
/// Client provided input configuration for generating the background.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateImageBackgroundConfig {
    /// Required. Example: "Hat on a baseball field"
    /// "Hat" = product description
    /// Description of product.
    #[prost(string, tag = "1")]
    pub product_description: ::prost::alloc::string::String,
    /// Required. Example: "Hat on a baseball field"
    /// "on a baseball field" = background description
    /// Description of wanted background.
    #[prost(string, tag = "2")]
    pub background_description: ::prost::alloc::string::String,
}
/// Client provided input configuration for removing the background.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RemoveImageBackgroundConfig {
    /// Optional. If set, the result of background removal will be an RGB image
    /// with this given color as the background, instead of an RGBA 4-channel
    /// transparent image.
    #[prost(message, optional, tag = "1")]
    pub background_color: ::core::option::Option<RgbColor>,
}
/// Represents a color in RGB format.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RgbColor {
    /// Optional. Values in \[0, 255\].
    #[prost(int32, tag = "1")]
    pub red: i32,
    /// Optional. Values in \[0, 255\].
    #[prost(int32, tag = "2")]
    pub green: i32,
    /// Optional. Values in \[0, 255\].
    #[prost(int32, tag = "3")]
    pub blue: i32,
}
/// Generated client implementations.
pub mod image_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service that exposes Generative AI (GenAI) endpoints for creating and
    /// enhancing product image content.
    #[derive(Debug, Clone)]
    pub struct ImageServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ImageServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ImageServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ImageServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// GenerateProductImageBackground generates a new image where the background
        /// of the original image is replaced by an AI generated scene based on
        /// provided product information and a text prompt.
        pub async fn generate_product_image_background(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GenerateProductImageBackgroundRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GenerateProductImageBackgroundResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.shopping.merchant.productstudio.v1alpha.ImageService/GenerateProductImageBackground",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.productstudio.v1alpha.ImageService",
                        "GenerateProductImageBackground",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// RemoveProductImageBackground generates a new image where the background of
        /// the original image is removed.
        pub async fn remove_product_image_background(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveProductImageBackgroundRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveProductImageBackgroundResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.shopping.merchant.productstudio.v1alpha.ImageService/RemoveProductImageBackground",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.productstudio.v1alpha.ImageService",
                        "RemoveProductImageBackground",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpscaleProductImage generates a new image where the resolution of the
        /// original image is enhanced.
        pub async fn upscale_product_image(
            &mut self,
            request: impl tonic::IntoRequest<super::UpscaleProductImageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpscaleProductImageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.shopping.merchant.productstudio.v1alpha.ImageService/UpscaleProductImage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.productstudio.v1alpha.ImageService",
                        "UpscaleProductImage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for the GenerateProductTextSuggestions method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProductTextSuggestionsRequest {
    /// Required. The name of the account to generate text suggestions for. This
    /// acts as a container for the request and does not affect the generation
    /// itself, as this is a stateless API. Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Available information about the product. Used to inform the genAI
    /// models.
    #[prost(message, optional, tag = "2")]
    pub product_info: ::core::option::Option<ProductInfo>,
    /// Optional. Configuration parameters that directly influence what content is
    /// generated, and how that content is rendered in the final response.
    #[prost(message, optional, tag = "3")]
    pub output_spec: ::core::option::Option<OutputSpec>,
    /// Optional. Provide some hand-crafted examples of title improvements that are
    /// unique to your use case. This is a general tool that handles multiple
    /// product categories, but your brand identity may require custom
    /// functionality. Feel free to specify that here.
    #[prost(message, repeated, tag = "4")]
    pub title_examples: ::prost::alloc::vec::Vec<TitleExample>,
}
/// Response message for the GenerateProductTextSuggestions method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProductTextSuggestionsResponse {
    /// Generated title suggestion.
    #[prost(message, optional, tag = "1")]
    pub title: ::core::option::Option<ProductTextGenerationSuggestion>,
    /// Generated description suggestion.
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<ProductTextGenerationSuggestion>,
    /// Any other generated attributes.
    #[prost(btree_map = "string, string", tag = "3")]
    pub attributes: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Additional info that clients may want to audit surrounding the generation.
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<ProductTextGenerationMetadata>,
}
/// A hand-crafted example of a product title improvement. These examples are
/// provided to the AI to improve its quality and guide it towards required
/// outputs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TitleExample {
    /// Required. A map containing all existing product information. For example:
    /// {"title": "dress", "description": "A red dress", "brand": "Dresses4All"}
    /// Any information that you might use to populate your product feed.
    #[prost(btree_map = "string, string", tag = "1")]
    pub product_info: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. The product's category. This helps the AI understand when certain
    /// examples are more relevant than others.
    #[prost(string, optional, tag = "2")]
    pub category: ::core::option::Option<::prost::alloc::string::String>,
    /// Required. The attributes or approximate attributes that make up the title.
    /// For example, title "Google GShoe M" title_format can be "brand | product |
    /// size".
    #[prost(string, optional, tag = "3")]
    pub title_format: ::core::option::Option<::prost::alloc::string::String>,
    /// Required. A map in the same format as product_info but with all
    /// improvements included. For example, {"brand": "Dresses4All", "product":
    /// "dress", "color": "red", ...}. The order of attributes in this map may be
    /// used to guide the order in which they appear in the final generated title.
    /// For instance, the above will become: Dresses4All dress | red
    #[prost(btree_map = "string, string", tag = "4")]
    pub final_product_info: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Wrapper data type for any metadata associated with text generation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductTextGenerationMetadata {
    /// Metadata is a pretty loose concept. The data is modeled as a map here to
    /// indicate that there is no guaranteed structure to the output past a simple
    /// K:V association.
    /// The first use-case is to track words added/removed/changed in generations.
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<::prost_types::Struct>,
}
/// Product image represented as bytes directly or a URI.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// Required. The image to use for text generation.
    #[prost(oneof = "image::Image", tags = "1, 2")]
    pub image: ::core::option::Option<image::Image>,
}
/// Nested message and enum types in `Image`.
pub mod image {
    /// Required. The image to use for text generation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Image {
        /// Generally web-requestable URI.
        #[prost(string, tag = "1")]
        Uri(::prost::alloc::string::String),
        /// Raw bytes for the image.
        #[prost(bytes, tag = "2")]
        Data(::prost::bytes::Bytes),
    }
}
/// Available information about the product. Used to inform the genAI models.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductInfo {
    /// Required. A mapping of all available product attributes. This may include
    /// title, description, brand, gender, color, size, etc.
    #[prost(btree_map = "string, string", tag = "1")]
    pub product_attributes: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Image associated with the product.
    #[prost(message, optional, tag = "2")]
    pub product_image: ::core::option::Option<Image>,
}
/// Configuration parameters that directly influence what content is generated,
/// and how that content is rendered in the final response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputSpec {
    /// Optional. The workflow to execute for the provided product data. Workflows
    /// may populate the response's title, description, or both. Currently
    /// supported workflow_ids are: "title", "description", and "tide"
    #[prost(string, optional, tag = "1")]
    pub workflow_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. The tone of the output generated text. Supported tones are:
    /// "playful", "formal", "persuasive", and "conversational"
    #[prost(string, optional, tag = "2")]
    pub tone: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. Any editorial changes for the generated product data. For
    /// example, replace Small with "S", do not modify color if already present.
    #[prost(string, optional, tag = "3")]
    pub editorial_changes: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. The language for output titles/descriptions. For example.
    /// 'German', 'es', 'FR'. Default is 'en'.
    #[prost(string, optional, tag = "4")]
    pub target_language: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. The order that generated attributes should be placed in the
    /// generated title. Eg., if the attribute order is ["brand", "product",
    /// "size"], the generated title will have brand first, followed by the product
    /// name, and then size information after that.
    #[prost(string, repeated, tag = "5")]
    pub attribute_order: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Character used to separate attributes in the generated title.
    /// For example, '|', '-', ','.
    #[prost(string, optional, tag = "6")]
    pub attribute_separator: ::core::option::Option<::prost::alloc::string::String>,
}
/// Text generated for a product, optionally including its quality score.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductTextGenerationSuggestion {
    /// The text generated
    #[prost(string, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The quality score associated with the generation. Heuristic implemented
    /// according to the feedgen team's implementation styles.
    #[prost(float, optional, tag = "2")]
    pub score: ::core::option::Option<f32>,
    /// A brief summarization of all the changes that have been made.
    #[prost(string, optional, tag = "3")]
    pub change_summary: ::core::option::Option<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod text_suggestions_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service that exposes Generative AI (GenAI) endpoints for creating and
    /// enhancing product text content, such as titles, descriptions, etc.
    #[derive(Debug, Clone)]
    pub struct TextSuggestionsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TextSuggestionsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TextSuggestionsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            TextSuggestionsServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// GenerateProductTextSuggestions generates a set of candidate text
        /// completions (e.g., product titles, descriptions) based on provided product
        /// information. This endpoint leverages GenAI models to create suggestions for
        /// improving existing product text or generating new content.
        pub async fn generate_product_text_suggestions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GenerateProductTextSuggestionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GenerateProductTextSuggestionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.shopping.merchant.productstudio.v1alpha.TextSuggestionsService/GenerateProductTextSuggestions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.productstudio.v1alpha.TextSuggestionsService",
                        "GenerateProductTextSuggestions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
