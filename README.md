    cargo run --help
    Run a binary or example of the local package
    
    Usage: cargo run [OPTIONS] [ARGS]...
    
    Arguments:
      [ARGS]...  Arguments for the binary or example to run
    
    Options:
          --message-format <FMT>  Error format
      -v, --verbose...            Use verbose output (-vv very verbose/build.rs output)
      -q, --quiet                 Do not print cargo log messages
          --color <WHEN>          Coloring: auto, always, never
          --config <KEY=VALUE>    Override a configuration value
      -Z <FLAG>                   Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
      -h, --help                  Print help
    
    Package Selection:
      -p, --package [<SPEC>]  Package with the target to run
    
    Target Selection:
          --bin [<NAME>]      Name of the bin target to run
          --example [<NAME>]  Name of the example target to run
    
    Feature Selection:
      -F, --features <FEATURES>  Space or comma separated list of features to activate
          --all-features         Activate all available features
          --no-default-features  Do not activate the `default` feature
    
    Compilation Options:
      -j, --jobs <N>                Number of parallel jobs, defaults to # of CPUs.
          --keep-going              Do not abort the build as soon as there is an error
      -r, --release                 Build artifacts in release mode, with optimizations
          --profile <PROFILE-NAME>  Build artifacts with the specified profile
          --target [<TRIPLE>]       Build for the target triple
          --target-dir <DIRECTORY>  Directory for all generated artifacts
          --unit-graph              Output build graph in JSON (unstable)
          --timings[=<FMTS>]        Timing output formats (unstable) (comma separated): html, json
    
    Manifest Options:
          --manifest-path <PATH>  Path to Cargo.toml
          --ignore-rust-version   Ignore `rust-version` specification in packages
          --locked                Assert that `Cargo.lock` will remain unchanged
          --offline               Run without accessing the network
          --frozen                Equivalent to specifying both --locked and --offline
    
    Run `cargo help run` for more detailed information.
