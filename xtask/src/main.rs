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
        .type_attribute(
            ".p4.config.v1",
            "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]",
        )
        .type_attribute(
            ".google.protobuf",
            "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]",
        )
        .field_attribute(
            ".google.protobuf.Any.type_url",
            "#[cfg_attr(feature = \"serde\", serde(rename = \"@type\"))]",
        )
        .compile_well_known_types(true)
        .compile(
            &[
                workspace_root.join("proto/p4runtime/proto/p4/v1/p4runtime.proto"),
                workspace_root.join("proto/p4runtime/proto/p4/v1/p4data.proto"),
                workspace_root.join("proto/p4runtime/proto/p4/config/v1/p4info.proto"),
                workspace_root.join("proto/p4runtime/proto/p4/config/v1/p4types.proto"),
            ],
            &[
                workspace_root.join("proto/p4runtime/proto"),
                workspace_root.join("proto/googleapis"),
            ],
        )?;

    // Read in the generated code
    let google_rpc_contents = fs::read_to_string(workspace_root.join("tmp/google.rpc.rs"))?;
    let google_protobuf_contents =
        fs::read_to_string(workspace_root.join("tmp/google.protobuf.rs"))?;
    let p4_v1_contents = fs::read_to_string(workspace_root.join("tmp/p4.v1.rs"))?;
    let p4_config_v1_contents = fs::read_to_string(workspace_root.join("tmp/p4.config.v1.rs"))?;

    // Construct the whole lib contents
    let lib_contents = format!(
        "#![allow(unused_variables)]
        
        pub mod google {{
            pub mod rpc {{
                {google_rpc_contents}
            }}

            pub mod protobuf {{
                {google_protobuf_contents}
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
        }}
        
        pub mod utils;
        "
    );

    // Use form to create the directory structure
    create_directory_structure(workspace_root.join("src"), &lib_contents)?;

    // Format the code
    Command::new("cargo")
        .arg("fmt")
        .current_dir(&workspace_root)
        .status()?;

    // Clean up the tmp directory
    fs::remove_dir_all(workspace_root.join("tmp"))?;

    Ok(())
}
