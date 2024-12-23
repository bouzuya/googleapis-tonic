use googleapis_tonic_google_storage_v2::google::storage::v2::WriteObjectRequest;

fn main() {
    let _request = WriteObjectRequest {
        write_offset: 0_i64,
        object_checksums: None,
        finish_write: false,
        common_object_request_params: None,
        first_message: None,
        data: None,
    };
}
