update_changelog() {
    local crate=$1
    local new_version=$2
    local changelog="$crate/CHANGELOG.md"
    local cargo_toml="$crate/Cargo.toml"
    local dependency_updated=$3

    # Format the release date
    local release_date
    release_date=$(date +"%Y-%b-%d")

    if [ ! -f "$changelog" ]; then
        echo "Warning: CHANGELOG.md not found for $crate"
        return
    fi

    if grep -q "## $new_version" "$changelog"; then
        echo "Note: Changelog for $crate already has version $new_version"
        return
    fi

    echo "Updating changelog for $crate"

    # Identify updated opentelemetry dependencies
    local dependencies_list=""
    while IFS= read -r line; do
        dep_name=$(echo "$line" | grep -oE '^\s*[^ ]+')
        if [[ "$dep_name" == opentelemetry* ]]; then
            dependencies_list+="- Update \`$dep_name\` dependency version to $new_version"$'\n'
        fi
    done < <(grep -E '^\s*opentelemetry[^ ]*\s*=\s*.*version\s*=\s*"'$new_version'"' "$cargo_toml")

    # Trim any trailing newline from dependencies_list
    dependencies_list=$(echo -n "$dependencies_list")

    # Create a temporary file for editing
    local temp_file=$(mktemp)

    # Write the new version section with release date and dependency updates
    {
        echo "# Changelog"
        echo
        echo "## vNext"
        echo
        echo "## $new_version"
        echo
        echo "Released $release_date"
        if [ -n "$dependencies_list" ]; then
            echo
            echo "$dependencies_list" | sed '/^$/d'  # Remove any accidental empty lines
        fi
    } > "$temp_file"

    # Append the rest of the changelog, skipping the old `vNext` section
    sed '1,/^## vNext/d' "$changelog" >> "$temp_file"

    # Ensure no trailing newline in the temp file to prevent extra blank lines between new and existing content
    truncate -s -1 "$temp_file"

    # Replace the original changelog with the updated content
    mv "$temp_file" "$changelog"
}
