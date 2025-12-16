use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion, BenchmarkId};
use unpack_core::resolver::{UnpackResolver, ResolveOptions};
use camino::Utf8Path;

fn bench_resolver_basic(c: &mut Criterion) {
    let options = ResolveOptions {
        ..Default::default()
    };
    let resolver = UnpackResolver::new(options);
    
    c.bench_function("resolver_basic", |b| {
        b.iter(|| {
            let _ = resolver.resolve(
                Utf8Path::new("/workspace/unpack"),
                "./src/lib.rs"
            );
        });
    });
}

fn bench_resolver_multiple_requests(c: &mut Criterion) {
    let options = ResolveOptions {
        ..Default::default()
    };
    let resolver = UnpackResolver::new(options);
    
    let requests = vec![
        "./src/lib.rs",
        "./src/resolver.rs",
        "./src/module.rs",
        "./src/compiler.rs",
    ];
    
    for request in &requests {
        c.bench_with_input(
            BenchmarkId::new("resolver_request", request),
            request,
            |b, &request| {
                b.iter(|| {
                    let _ = resolver.resolve(
                        Utf8Path::new("/workspace/unpack"),
                        request
                    );
                });
            }
        );
    }
}

criterion_group!(benches, bench_resolver_basic, bench_resolver_multiple_requests);
criterion_main!(benches);
