#!/usr/bin/env bash
#
# Generate Nickel types from downloaded CRDs
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FIXTURES_DIR="$SCRIPT_DIR/../tests/fixtures"
CRDS_DIR="$FIXTURES_DIR/crds"
GENERATED_DIR="$FIXTURES_DIR/generated"

# Build amalgam if not already built
echo "🔨 Building amalgam..."
cd "$SCRIPT_DIR/../../.." # Go to workspace root
cargo build --release --bin amalgam 2>&1 | tail -5

AMALGAM_BIN="$SCRIPT_DIR/../../../target/release/amalgam"

if [ ! -f "$AMALGAM_BIN" ]; then
    echo "❌ amalgam binary not found at $AMALGAM_BIN"
    exit 1
fi

echo "✅ amalgam built successfully"
echo ""

mkdir -p "$GENERATED_DIR"

echo "🔧 Generating Nickel types from CRDs..."

# Generate from Crossplane CRDs
echo "  → Crossplane types..."
for crd_file in "$CRDS_DIR/crossplane"/*.yaml; do
    echo "    Processing $(basename "$crd_file")..."
    "$AMALGAM_BIN" generate \
        --input "$crd_file" \
        --output "$GENERATED_DIR" \
        --format crd 2>&1 | grep -E "(Generated|Error|Warning)" || true
done
echo "    ✓ Crossplane types generated"

# Generate from ArgoCD CRDs
echo "  → ArgoCD types..."
for crd_file in "$CRDS_DIR/argocd"/*.yaml; do
    echo "    Processing $(basename "$crd_file")..."
    "$AMALGAM_BIN" generate \
        --input "$crd_file" \
        --output "$GENERATED_DIR" \
        --format crd 2>&1 | grep -E "(Generated|Error|Warning)" || true
done
echo "    ✓ ArgoCD types generated"

echo ""
echo "✅ Type generation complete!"
echo "   Output directory: $GENERATED_DIR"
echo ""
echo "📊 Generated types summary:"
find "$GENERATED_DIR" -name "*.ncl" | wc -l | xargs echo "   Total .ncl files:"
echo ""
echo "   Structure:"
find "$GENERATED_DIR" -type d -maxdepth 2 | sort | sed 's/^/   /'
