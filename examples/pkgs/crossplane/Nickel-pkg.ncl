# Amalgam Package Manifest
# Generated: 2025-09-01T23:35:46.786769+00:00
# Generator: amalgam v0.6.3
# Source: https://github.com/crossplane/crossplane/tree/v2.0.2/cluster/crds
# Git ref: v2.0.2
{
  # Package identity
  name = "crossplane",
  version = "2.0.2",
  
  # Package information
  description = "Crossplane CRD type definitions for infrastructure as code",
  authors = ["amalgam"],
  keywords = ["crossplane", "kubernetes", "infrastructure", "gitops"],
  license = "Apache-2.0",
  
  # Dependencies
  dependencies = {
    k8s_io = 'Index { package = "github:seryl/nickel-pkgs/pkgs/k8s-io", version = "1.33.4" }
  },
  
  # Nickel version requirement
  minimal_nickel_version = "1.9.0",
} | std.package.Manifest
