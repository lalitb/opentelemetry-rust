#!/bin/bash

# Check for version argument
if [ -z "$1" ]; then
    echo "Usage: $0 <new_version>"
    exit 1
fi

NEW_VERSION=$1
WORKSPACE_MANIFEST="Cargo.toml"

# Retrieve workspace member directories that start with "opentelemetry" or "opentelemetry-"
WORKSPACE_CRATES=$(find . -type f -name "Cargo.toml" -exec dirname {} \; | grep -E '^./opentelemetry(-|$)' | sed 's|./||')

# Update each crate version and dependencies
for CRATE in $WORKSPACE_CRATES; do
    CARGO_TOML="$CRATE/Cargo.toml"
    CHANGELOG="$CRATE/CHANGELOG.md"
    DEPENDENCY_UPDATED=false

    if [ -f "$CARGO_TOML" ]; then
        # Check if the current version already matches the new version
        CURRENT_VERSION=$(grep -E '^version = "' "$CARGO_TOML" | sed -E 's/version = "(.*)"/\1/')
        if [ "$CURRENT_VERSION" == "$NEW_VERSION" ]; then
            echo "Version for $CRATE is already $NEW_VERSION, skipping."
            continue
        fi

        # Update the crate's main version, but skip `rust-version`
        echo "Updating version for $CRATE to $NEW_VERSION"
        sed -i.bak -E "/^rust-version =/!s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"

        # Update dependencies with a specified version key in any section, excluding `rust-version`
        perl -i.bak -pe "
            if (/version = \"\d+\.\d+(\.\d+)?\"/ && !/rust-version/) {
                s/version = \"\d+\.\d+(\.\d+)?\"/version = \"$NEW_VERSION\"/g;
                \$DEPENDENCY_UPDATED = 1;
            }
        " "$CARGO_TOML" && DEPENDENCY_UPDATED=true

        rm "$CARGO_TOML.bak"
    else
        echo "Cargo.toml not found for $CRATE, skipping."
    fi

    # Update the changelog only if the "vNext" section exists
    if [ -f "$CHANGELOG" ]; then
        # Check if changelog already has the new version as the latest entry
        if grep -q "## $NEW_VERSION" "$CHANGELOG"; then
            echo "Changelog for $CRATE already has version $NEW_VERSION, skipping."
            continue
        fi

        echo "Updating changelog for $CRATE"
        # Replace "vNext" with the new version, then add an empty vNext section at the top
        sed -i.bak -E "s/^## vNext/## $NEW_VERSION/" "$CHANGELOG"
        sed -i "1s/^/# Changelog\n\n## vNext\n\n/" "$CHANGELOG"

        # Add entry for dependency updates if there are any
        if [ "$DEPENDENCY_UPDATED" = true ]; then
            echo "- Updated dependencies to version $NEW_VERSION" >> "$CHANGELOG"
        fi

        rm "$CHANGELOG.bak"
    else
        echo "CHANGELOG.md not found for $CRATE, skipping."
    fi
done

echo "All versions and changelogs updated to $NEW_VERSION where necessary."
