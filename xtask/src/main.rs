use std::{fs, path::PathBuf, process::Command};

use form::create_directory_structure;

fn main() -> anyhow::Result<()> {
    // First, generate code to tmp directory
    // Then, read in the generated code and use form to move it to src
    // Third, fmt the code

    let workspace_root = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));

    // Create tmp if it doesn't exist
    fs::create_dir_all(workspace_root.join("tmp"))?;

    tonic_build::configure()
        .out_dir(workspace_root.join("tmp"))
        .compile(
            &[
                "p4runtime/proto/p4/v1/p4runtime.proto",
                "p4runtime/proto/p4/v1/p4data.proto",
                "p4runtime/proto/p4/config/v1/p4info.proto",
                "p4runtime/proto/p4/config/v1/p4types.proto",
            ],
            &["./p4runtime/proto", "./googleapis/"],
        )?;

    // Read in the generated code
    let google_rpc_contents = fs::read_to_string(workspace_root.join("tmp/google.rpc.rs"))?;
    let p4_v1_contents = fs::read_to_string(workspace_root.join("tmp/p4.v1.rs"))?;
    let p4_config_v1_contents = fs::read_to_string(workspace_root.join("tmp/p4.config.v1.rs"))?;

    // Construct the whole lib contents
    let lib_contents = format!(
        "pub mod google {{
            pub mod rpc {{
                {google_rpc_contents}
            }}
        }}
        
        pub mod p4 {{
            pub mod v1 {{
                {p4_v1_contents}
            }}
            pub mod config {{
                pub mod v1 {{
                    {p4_config_v1_contents}
                }}
            }}
        }}"
    );

    // Use form to create the directory structure
    create_directory_structure(workspace_root.join("src"), &lib_contents)?;

    // Format the code
    Command::new("cargo")
        .arg("fmt")
        .current_dir(workspace_root)
        .status()?;

    Ok(())
}
