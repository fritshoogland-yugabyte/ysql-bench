#[allow(unused_imports)]
use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use postgres_openssl::MakeTlsConnector;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use rust_postgres_sync::*;

const FULL_PATH_CA_CERT: &str = "/tmp/ca.cert";

#[allow(dead_code)]
fn notls_connection(url: &str) {
    let _c = crate::create_notls_connection(url);
}

#[allow(dead_code)]
fn tls_connection(url: &str, connection: MakeTlsConnector) {
    let _c = crate::create_tls_connection(url, connection);
}


fn criterion_benchmark(c: &mut Criterion) {

    let mut builder = SslConnector::builder(SslMethod::tls()).expect("unable to create sslconnector builder");
    builder.set_ca_file(FULL_PATH_CA_CERT).expect("unable to load ca.cert");
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());

    let mut group = c.benchmark_group("connections");
    group.sample_size(1000);
    group.measurement_time(Duration::from_secs(60));
    // 1
    let connection = "host=/tmp sslmode=disable user=postgres password=postgres";
    group.bench_function("socket", |b| b.iter(|| notls_connection(connection)));
    // 2
    let connection = "host=localhost port=5432 sslmode=disable user=postgres password=postgres";
    group.bench_function("localhost-notls", |b| b.iter(|| notls_connection(connection)));
    // 3
    let connection = "host=localhost port=5432 sslmode=require user=postgres password=postgres";
    group.bench_function("localhost-tls", |b| b.iter(|| { let connector = connector.clone(); tls_connection(connection, connector)}));
    // 4
    let connection = "host=10.0.2.15 port=5432 sslmode=disable user=postgres password=postgres";
    group.bench_function("public-nic-notls", |b| b.iter(|| notls_connection(connection)));
    // 5
    let connection = "host=10.0.2.15 port=5432 sslmode=require user=postgres password=postgres";
    group.bench_function("public-nic-tls", |b| b.iter(|| { let connector = connector.clone(); tls_connection(connection, connector)}));
    // 6
    let connection = "host=10.0.2.15 port=6432 sslmode=disable user=postgres password=postgres";
    group.bench_function("pgbouncer-notls", |b| b.iter(|| notls_connection(connection)));
    // 7
    let connection = "host=10.0.2.15 port=6432 sslmode=require user=postgres password=postgres";
    group.bench_function("pgbouncer-tls", |b| b.iter(|| { let connector = connector.clone(); tls_connection(connection, connector)}));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);