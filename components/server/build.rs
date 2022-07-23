fn main() {
    tonic_build::compile_protos("../../kvproto/kvrpcpb.proto").unwrap();
    tonic_build::compile_protos("../../kvproto/wildkvpb.proto").unwrap();
}