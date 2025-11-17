#!/usr/bin/env bash
#
# Download CRDs from various Kubernetes ecosystem projects
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FIXTURES_DIR="$SCRIPT_DIR/../tests/fixtures"
CRDS_DIR="$FIXTURES_DIR/crds"

mkdir -p "$CRDS_DIR"/{k8s-core,crossplane,argocd,cert-manager}

echo "📦 Downloading CRDs for verification..."

# Kubernetes Core CRDs
# Note: k8s core types are built-in, but we can get them from the API
echo "  → Kubernetes core types (from k8s-openapi schemas)..."
mkdir -p "$CRDS_DIR/k8s-core"

# We'll use kubectl to extract core schemas if available
if command -v kubectl &> /dev/null; then
    echo "    ✓ kubectl found, extracting core type schemas..."
    # Get Deployment, Service, Pod, ConfigMap, Secret schemas
    for kind in Deployment Service Pod ConfigMap Secret; do
        kubectl explain --api-version=v1 "$kind" --output=yaml > "$CRDS_DIR/k8s-core/${kind}.yaml" 2>/dev/null || true
    done
else
    echo "    ℹ kubectl not found, skipping core type extraction"
    echo "    (Core types will be generated from k8s-openapi crate)"
fi

# Crossplane CRDs
echo "  → Crossplane CRDs..."
CROSSPLANE_VERSION="v1.14.5"
curl -sL "https://raw.githubusercontent.com/crossplane/crossplane/${CROSSPLANE_VERSION}/cluster/crds/apiextensions.crossplane.io_compositeresourcedefinitions.yaml" \
    -o "$CRDS_DIR/crossplane/compositeresourcedefinitions.yaml"

curl -sL "https://raw.githubusercontent.com/crossplane/crossplane/${CROSSPLANE_VERSION}/cluster/crds/apiextensions.crossplane.io_compositions.yaml" \
    -o "$CRDS_DIR/crossplane/compositions.yaml"

curl -sL "https://raw.githubusercontent.com/crossplane/crossplane/${CROSSPLANE_VERSION}/cluster/crds/pkg.crossplane.io_providers.yaml" \
    -o "$CRDS_DIR/crossplane/providers.yaml"

echo "    ✓ Downloaded Crossplane CRDs (${CROSSPLANE_VERSION})"

# ArgoCD CRDs
echo "  → ArgoCD CRDs..."
ARGOCD_VERSION="v2.9.5"
ARGOCD_MANIFEST_BASE="https://raw.githubusercontent.com/argoproj/argo-cd/${ARGOCD_VERSION}/manifests/crds"

curl -sL "${ARGOCD_MANIFEST_BASE}/application-crd.yaml" \
    -o "$CRDS_DIR/argocd/application-crd.yaml"

curl -sL "${ARGOCD_MANIFEST_BASE}/applicationset-crd.yaml" \
    -o "$CRDS_DIR/argocd/applicationset-crd.yaml"

curl -sL "${ARGOCD_MANIFEST_BASE}/appproject-crd.yaml" \
    -o "$CRDS_DIR/argocd/appproject-crd.yaml"

echo "    ✓ Downloaded ArgoCD CRDs (${ARGOCD_VERSION})"

# Cert-manager CRDs
echo "  → Cert-manager CRDs..."
CERT_MANAGER_VERSION="v1.13.3"
curl -sL "https://github.com/cert-manager/cert-manager/releases/download/${CERT_MANAGER_VERSION}/cert-manager.crds.yaml" \
    -o "$CRDS_DIR/cert-manager/cert-manager-crds.yaml"

echo "    ✓ Downloaded Cert-manager CRDs (${CERT_MANAGER_VERSION})"

# Create a manifest file documenting what was downloaded
cat > "$CRDS_DIR/manifest.yaml" <<EOF
# CRD Sources for Verification
downloaded_at: $(date -u +"%Y-%m-%dT%H:%M:%SZ")

sources:
  kubernetes:
    note: "Core types from k8s-openapi crate"
    types:
      - v1/Pod
      - v1/Service
      - v1/ConfigMap
      - v1/Secret
      - apps/v1/Deployment

  crossplane:
    version: ${CROSSPLANE_VERSION}
    repository: https://github.com/crossplane/crossplane
    crds:
      - apiextensions.crossplane.io/v1/CompositeResourceDefinition
      - apiextensions.crossplane.io/v1/Composition
      - pkg.crossplane.io/v1/Provider

  argocd:
    version: ${ARGOCD_VERSION}
    repository: https://github.com/argoproj/argo-cd
    crds:
      - argoproj.io/v1alpha1/Application
      - argoproj.io/v1alpha1/ApplicationSet
      - argoproj.io/v1alpha1/AppProject

  cert-manager:
    version: ${CERT_MANAGER_VERSION}
    repository: https://github.com/cert-manager/cert-manager
    crds:
      - cert-manager.io/v1/Certificate
      - cert-manager.io/v1/CertificateRequest
      - cert-manager.io/v1/Issuer
      - cert-manager.io/v1/ClusterIssuer
EOF

echo ""
echo "✅ CRD download complete!"
echo "   Location: $CRDS_DIR"
echo "   Manifest: $CRDS_DIR/manifest.yaml"
