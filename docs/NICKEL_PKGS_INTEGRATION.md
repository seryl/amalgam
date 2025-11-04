# Nickel Packages Integration

This document describes how Amalgam integrates with the [nickel-pkgs](https://github.com/seryl/nickel-pkgs) repository for automated package generation.

## Overview

When changes are merged to the `main` branch of the Amalgam repository, a GitHub Actions workflow automatically triggers a rebuild of the nickel-pkgs repository. This ensures that all generated Nickel type packages stay up-to-date with the latest Amalgam tooling.

## Architecture

```
┌─────────────────────┐
│  Amalgam Repo       │
│  (Tool Changes)     │
└──────────┬──────────┘
           │
           │ Merge to main
           │
           ▼
    ┌──────────────────┐
    │ GitHub Actions   │
    │ Workflow         │
    └──────┬───────────┘
           │
           │ Repository Dispatch
           │ (event: amalgam-updated)
           ▼
┌─────────────────────┐
│  nickel-pkgs Repo   │
│  (Package Output)   │
└──────────┬──────────┘
           │
           │ Regenerate packages
           │
           ▼
    ┌──────────────────┐
    │ Updated Packages │
    │ Committed & PR   │
    └──────────────────┘
```

## Setup Requirements

### 1. GitHub Token Setup

Create a Personal Access Token (PAT) with `repo` scope:

1. Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Click "Generate new token (classic)"
3. Give it a descriptive name: `NICKEL_PKGS_TRIGGER_TOKEN`
4. Select scopes:
   - ✅ `repo` (Full control of private repositories)
5. Generate and save the token
6. Add the token to Amalgam repository secrets:
   - Go to Amalgam repo → Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `NICKEL_PKGS_TRIGGER_TOKEN`
   - Value: Your PAT token

### 2. nickel-pkgs Workflow Setup

Create `.github/workflows/rebuild-packages.yml` in the nickel-pkgs repository:

```yaml
name: Rebuild Packages

on:
  # Triggered by amalgam repo
  repository_dispatch:
    types: [amalgam-updated]

  # Manual trigger for testing
  workflow_dispatch:
    inputs:
      amalgam_commit:
        description: 'Specific Amalgam commit to use (optional)'
        required: false

  # Scheduled rebuild (weekly on Monday)
  schedule:
    - cron: '0 2 * * 1'  # 2 AM UTC every Monday

jobs:
  regenerate-packages:
    name: Regenerate All Packages
    runs-on: ubuntu-latest

    steps:
      - name: Checkout nickel-pkgs
        uses: actions/checkout@v4
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          fetch-depth: 0

      - name: Checkout amalgam (latest or specific commit)
        uses: actions/checkout@v4
        with:
          repository: seryl/amalgam
          path: amalgam
          ref: ${{ github.event.client_payload.amalgam_commit || github.event.inputs.amalgam_commit || 'main' }}

      - name: Setup Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1
        with:
          toolchain: stable

      - name: Build Amalgam
        working-directory: amalgam
        run: |
          cargo build --release --bin amalgam
          echo "$PWD/target/release" >> $GITHUB_PATH

      - name: Generate packages from manifest
        run: |
          # Run amalgam with the manifest in nickel-pkgs
          if [ -f ".amalgam-manifest.toml" ]; then
            amalgam generate-from-manifest
          else
            echo "No .amalgam-manifest.toml found, creating default..."
            cat > .amalgam-manifest.toml << 'EOF'
          [config]
          output_base = "pkgs"
          package_mode = true
          base_package_id = "github:seryl/nickel-pkgs/pkgs"

          [[packages]]
          source = "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.33.4/api/openapi-spec/swagger.json"

          [[packages]]
          source = "https://github.com/crossplane/crossplane/tree/v2.0.2/cluster/crds"
          EOF
            amalgam generate-from-manifest
          fi

      - name: Commit and create PR if changes
        id: commit
        run: |
          git config user.name "amalgam-bot[bot]"
          git config user.email "amalgam-bot[bot]@users.noreply.github.com"

          if [ -n "$(git status --porcelain pkgs/)" ]; then
            # Get trigger info
            AMALGAM_COMMIT="${{ github.event.client_payload.amalgam_commit_short || 'main' }}"
            TRIGGER_REASON="${{ github.event.client_payload.trigger_reason || 'Scheduled rebuild' }}"

            # Create branch
            BRANCH_NAME="auto-rebuild/$(date +%Y%m%d-%H%M%S)"
            git checkout -b "$BRANCH_NAME"

            # Commit changes
            git add pkgs/
            git commit -m "chore: regenerate packages

Triggered by: $TRIGGER_REASON
Amalgam commit: $AMALGAM_COMMIT

Updated packages:
$(git diff --name-only HEAD~1 | grep '^pkgs/' | head -10)
$([ $(git diff --name-only HEAD~1 | grep '^pkgs/' | wc -l) -gt 10 ] && echo "... and more")"

            # Push branch
            git push origin "$BRANCH_NAME"

            # Create PR
            gh pr create \
              --title "Regenerate packages ($(date +%Y-%m-%d))" \
              --body "## 🤖 Automated Package Regeneration

**Trigger Reason:** $TRIGGER_REASON
**Amalgam Commit:** [$AMALGAM_COMMIT](https://github.com/seryl/amalgam/commit/${{ github.event.client_payload.amalgam_commit }})
**Workflow:** [View run](${{ github.event.client_payload.workflow_url }})

### Changes
This PR contains regenerated Nickel type packages using the latest Amalgam tooling.

### Verification
- [ ] Check that mod.ncl files have proper documentation
- [ ] Verify package cross-references are correct
- [ ] Test import resolution in dependent projects

cc @seryl" \
              --label "automated,packages"

            echo "pr_created=true" >> $GITHUB_OUTPUT
            echo "branch=$BRANCH_NAME" >> $GITHUB_OUTPUT
          else
            echo "No changes to commit"
            echo "pr_created=false" >> $GITHUB_OUTPUT
          fi
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}

      - name: Create summary
        if: always()
        run: |
          echo "## 📦 Package Regeneration Complete" >> $GITHUB_STEP_SUMMARY
          echo "" >> $GITHUB_STEP_SUMMARY

          if [ "${{ steps.commit.outputs.pr_created }}" == "true" ]; then
            echo "✅ **Status:** Changes detected and PR created" >> $GITHUB_STEP_SUMMARY
            echo "**Branch:** \`${{ steps.commit.outputs.branch }}\`" >> $GITHUB_STEP_SUMMARY
          else
            echo "ℹ️ **Status:** No changes detected" >> $GITHUB_STEP_SUMMARY
          fi

          echo "" >> $GITHUB_STEP_SUMMARY
          echo "**Amalgam Commit:** ${{ github.event.client_payload.amalgam_commit_short || 'main' }}" >> $GITHUB_STEP_SUMMARY
          echo "**Trigger:** ${{ github.event.client_payload.trigger_reason || 'Scheduled' }}" >> $GITHUB_STEP_SUMMARY
```

### 3. nickel-pkgs Manifest Setup

Create or update `.amalgam-manifest.toml` in the nickel-pkgs repository:

```toml
[config]
# Output directory for generated packages
output_base = "pkgs"

# Enable package mode (generates Nickel-pkg.ncl files)
package_mode = true

# Base package ID for dependencies
base_package_id = "github:seryl/nickel-pkgs/pkgs"

# ===========================
# Core Kubernetes Types
# ===========================

[[packages]]
# Kubernetes OpenAPI - Core types for all k8s resources
source = "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.33.4/api/openapi-spec/swagger.json"
description = "Kubernetes core types from OpenAPI specification"

# ===========================
# CrossPlane CRDs
# ===========================

[[packages]]
# CrossPlane core CRDs (multi-domain: apiextensions, pkg, ops, etc.)
source = "https://github.com/crossplane/crossplane/tree/v2.0.2/cluster/crds"
description = "CrossPlane CRDs for composition and XRDs"

# ===========================
# Cert Manager
# ===========================

[[packages]]
source = [
  "https://raw.githubusercontent.com/cert-manager/cert-manager/v1.12.0/deploy/crds/crd-certificates.yaml",
  "https://raw.githubusercontent.com/cert-manager/cert-manager/v1.12.0/deploy/crds/crd-issuers.yaml",
  "https://raw.githubusercontent.com/cert-manager/cert-manager/v1.12.0/deploy/crds/crd-clusterissuers.yaml",
]
description = "Cert Manager CRDs for certificate management"

# ===========================
# ArgoCD
# ===========================

[[packages]]
source = [
  "https://raw.githubusercontent.com/argoproj/argo-cd/v2.9.0/manifests/crds/application-crd.yaml",
  "https://raw.githubusercontent.com/argoproj/argo-cd/v2.9.0/manifests/crds/applicationset-crd.yaml",
  "https://raw.githubusercontent.com/argoproj/argo-cd/v2.9.0/manifests/crds/appproject-crd.yaml",
]
description = "ArgoCD CRDs for GitOps deployments"

# ===========================
# Prometheus Operator
# ===========================

[[packages]]
source = "https://raw.githubusercontent.com/prometheus-operator/prometheus-operator/v0.68.0/example/prometheus-operator-crd/monitoring.coreos.com_alertmanagers.yaml"
description = "Prometheus Operator CRDs for monitoring"

# ===========================
# Tekton Pipelines
# ===========================

[[packages]]
source = [
  "https://raw.githubusercontent.com/tektoncd/pipeline/v0.53.0/config/300-crds/300-pipeline.yaml",
  "https://raw.githubusercontent.com/tektoncd/pipeline/v0.53.0/config/300-crds/300-pipelinerun.yaml",
  "https://raw.githubusercontent.com/tektoncd/pipeline/v0.53.0/config/300-crds/300-task.yaml",
  "https://raw.githubusercontent.com/tektoncd/pipeline/v0.53.0/config/300-crds/300-taskrun.yaml",
]
description = "Tekton Pipeline CRDs for CI/CD"

# ===========================
# Knative Serving
# ===========================

[[packages]]
source = "https://github.com/knative/serving/tree/v1.12.0/config/core/300-crds"
description = "Knative Serving CRDs for serverless"

# ===========================
# Velero Backup
# ===========================

[[packages]]
source = [
  "https://raw.githubusercontent.com/vmware-tanzu/velero/v1.12.0/config/crd/v1/bases/velero.io_backups.yaml",
  "https://raw.githubusercontent.com/vmware-tanzu/velero/v1.12.0/config/crd/v1/bases/velero.io_restores.yaml",
  "https://raw.githubusercontent.com/vmware-tanzu/velero/v1.12.0/config/crd/v1/bases/velero.io_schedules.yaml",
]
description = "Velero CRDs for backup and disaster recovery"

# Add more packages as needed...
```

## Testing the Integration

### Test Manual Trigger

1. Go to Amalgam repo → Actions → "Trigger Nickel Packages Rebuild"
2. Click "Run workflow"
3. Enter a reason: "Testing integration"
4. Check that nickel-pkgs repo receives the event and starts rebuild

### Test Automatic Trigger

1. Make a change to Amalgam code (e.g., update mod.ncl generation)
2. Create PR and merge to `main`
3. Verify that the workflow triggers automatically
4. Check nickel-pkgs for new PR with regenerated packages

## Monitoring

- **Amalgam Workflow:** Check `.github/workflows/trigger-nickel-pkgs-rebuild.yml`
- **nickel-pkgs Workflow:** Check `seryl/nickel-pkgs` Actions tab
- **Generated Packages:** Check `seryl/nickel-pkgs/pkgs/` directory

## Troubleshooting

### Token Issues
- Ensure `NICKEL_PKGS_TRIGGER_TOKEN` has `repo` scope
- Token must not be expired
- Token owner must have write access to nickel-pkgs

### Workflow Not Triggering
- Check that the workflow file is on the `main` branch
- Verify the event type matches: `amalgam-updated`
- Check GitHub Actions logs for errors

### Build Failures
- Ensure Rust toolchain version is compatible
- Check that Cargo.toml dependencies are up to date
- Verify network access to CRD sources

## Benefits

1. **Always Up-to-Date:** Packages regenerate automatically when Amalgam improves
2. **Rich Documentation:** Enhanced mod.ncl files with context and cross-references
3. **Traceable:** Each regeneration links back to the triggering Amalgam commit
4. **Reviewable:** Changes go through PR process for verification
5. **Scheduled Fallback:** Weekly rebuilds ensure packages stay fresh

## Future Enhancements

- [ ] Add package validation tests before committing
- [ ] Generate change summaries comparing old vs new types
- [ ] Semantic versioning based on breaking changes
- [ ] Dependency graph visualization in PRs
- [ ] Nickel LSP integration tests
