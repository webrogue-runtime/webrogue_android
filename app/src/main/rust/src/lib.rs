use webrogue_runtime::WasiFactory;

fn main() -> anyhow::Result<()> {
    let lifecycle = webrogue_runtime::Lifecycle::new();

    let wasi_factory = webrogue_wasi_sync::WasiFactory::new();
    let mut wasi = wasi_factory.make();

    wasi_factory.add_actual_dir(&mut wasi, std::env::current_dir()?, "/");

    webrogue_std_stream_sdl::run_in_terminal(
        wasi,
        std::sync::Arc::new(move |wasi| {
            #[cfg(feature = "backend_v8")]
            let backend = webrogue_backend_v8::Backend::new();
            #[cfg(feature = "backend_wasmtime")]
            let backend = webrogue_backend_wasmtime::Backend::new();

            lifecycle.run(
                backend,
                wasi,
                webrogue_runtime::wrapp::Reader::from_vec(
                    include_bytes!("../external/webrogue_rs/example_apps/bin/simple.wrapp")
                        .to_vec(),
                )?,
            )
        }),
    );

    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn webrogue_android_main() {
    main().unwrap();
}
