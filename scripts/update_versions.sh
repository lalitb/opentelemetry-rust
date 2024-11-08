#!/bin/bash

set -euo pipefail

# Print usage information
print_usage() {
    echo "Usage: $0 <new_version>"
    echo "Updates versions and changelogs for all OpenTelemetry crates in the workspace"
    echo
    echo "Arguments:"
    echo "  new_version    The new version number (e.g., 1.0.0)"
}

# Validate semantic version format
validate_version() {
    local version=$1
    if ! echo "$version" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$'; then
        echo "Error: Version must be in semantic versioning format (X.Y.Z)"
        exit 1
    fi
}

# Update changelog for a crate
update_changelog() {
    local crate=$1
    local new_version=$2
    local changelog="$crate/CHANGELOG.md"
    local dependency_updated=$3

    if [ ! -f "$changelog" ]; then
        echo "Warning: CHANGELOG.md not found for $crate"
        return
    fi

    if grep -q "## $new_version" "$changelog"; then
        echo "Note: Changelog for $crate already has version $new_version"
        return
    fi

    echo "Updating changelog for $crate"
    
    # Create temporary file
    local temp_file=$(mktemp)
    
    # Add new version header
    echo "# Changelog" > "$temp_file"
    echo >> "$temp_file"
    echo "## vNext" >> "$temp_file"
    echo >> "$temp_file"
    
    # Replace vNext with new version and append rest of the file
    sed "1,/^## vNext/d" "$changelog" | sed "1s/^## vNext/## $new_version/" >> "$temp_file"
    
    # Add dependency update entry if needed
    if [ "$dependency_updated" = "true" ]; then
        sed -i "/^## $new_version/a - Updated dependencies to version $new_version" "$temp_file"
    fi
    
    mv "$temp_file" "$changelog"
}

# Update version in Cargo.toml
update_cargo_toml() {
    local crate=$1
    local new_version=$2
    local cargo_toml="$crate/Cargo.toml"

    if [ ! -f "$cargo_toml" ]; then
        echo "Error: Cargo.toml not found for $crate"
        return 1
    fi

    local current_version
    current_version=$(grep -E '^version = "' "$cargo_toml" | sed -E 's/version = "(.*)"/\1/')
    
    if [ "$current_version" = "$new_version" ]; then
        echo "Note: Version for $crate is already $new_version"
        return 0
    fi

    echo "Updating version for $crate to $new_version"
    
    # Create temporary file
    local temp_file=$(mktemp)
    
    # Update versions while preserving formatting
    awk -v new_ver="$new_version" '
        /^rust-version =/ { print; next }
        /version = "[0-9]+\.[0-9]+(\.[0-9]+)?"/ { 
            gsub(/"[0-9]+\.[0-9]+(\.[0-9]+)?"/, "\"" new_ver "\"")
            updated = 1
        }
        { print }
    ' "$cargo_toml" > "$temp_file"
    
    mv "$temp_file" "$cargo_toml"
    
    return 0
}

main() {
    # Check arguments
    if [ $# -ne 1 ]; then
        print_usage
        exit 1
    fi

    local new_version=$1
    validate_version "$new_version"

    # Find workspace crates
    local workspace_crates
    workspace_crates=$(find . -type f -name "Cargo.toml" -exec dirname {} \; | 
                      grep -E '^./opentelemetry(-|$)' | 
                      sed 's|./||' |
                      sort)

    if [ -z "$workspace_crates" ]; then
        echo "Error: No OpenTelemetry crates found in workspace"
        exit 1
    fi

    # Process each crate
    local exit_code=0
    for crate in $workspace_crates; do
        echo "Processing $crate..."
        
        if ! update_cargo_toml "$crate" "$new_version"; then
            echo "Error: Failed to update $crate"
            exit_code=1
            continue
        fi
        
        # Check if dependencies were updated
        local dependency_updated=false
        if grep -q "version = \"$new_version\"" "$crate/Cargo.toml" | grep -v "^version ="; then
            dependency_updated=true
        fi
        
        update_changelog "$crate" "$new_version" "$dependency_updated"
        echo "----------------------------------------"
    done

    if [ $exit_code -eq 0 ]; then
        echo "Successfully updated all crates to version $new_version"
    else
        echo "Completed with errors. Please check the output above."
    fi

    exit $exit_code
}

main "$@"