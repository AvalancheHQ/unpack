use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rspack_resolver::ResolveOptions;
use std::{path::PathBuf, sync::Arc};
use unpack_core::compiler::{CompilerOptions, EntryItem};

fn bench_compiler_options_creation(c: &mut Criterion) {
    c.bench_function("compiler_options_creation", |b| {
        b.iter(|| {
            let context = black_box(PathBuf::from("/tmp/test"));
            let _compiler_options: CompilerOptions = CompilerOptions {
                context: context.clone().try_into().expect("expect utf8 path"),
                entry: vec![EntryItem {
                    name: "main".to_string(),
                    import: "./src/index.js".to_string(),
                }],
                resolve: ResolveOptions {
                    extensions: vec![".js", ".ts", ".mjs", ".jsx"]
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>(),
                    ..Default::default()
                },
                output_dir: context.join("dist").try_into().expect("expect utf8 path"),
            };
        });
    });
}

fn bench_resolve_options(c: &mut Criterion) {
    c.bench_function("resolve_options_creation", |b| {
        b.iter(|| {
            let _options = ResolveOptions {
                extensions: black_box(
                    vec![".js", ".ts", ".mjs", ".jsx"]
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>(),
                ),
                ..Default::default()
            };
        });
    });
}

criterion_group!(
    benches,
    bench_compiler_options_creation,
    bench_resolve_options
);
criterion_main!(benches);
