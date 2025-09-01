# Amalgam Package Manifest
# Generated: 2025-09-01T13:36:01.564571+00:00
# Generator: amalgam v0.4.1
# Source: https://github.com/crossplane/crossplane/tree/main/cluster/crds
# Git ref: v1.17.2
{
  # Package identity
  name = "crossplane",
  version = "1.17.2",
  
  # Package information
  description = "Crossplane CRD type definitions for infrastructure as code",
  authors = ["amalgam"],
  keywords = ["crossplane", "kubernetes", "infrastructure", "gitops"],
  license = "Apache-2.0",
  
  # Dependencies
  dependencies = {
    k8s_io = 'Index { package = "github:seryl/nickel-pkgs/k8s-io", version = "1.31.0" }
  },
  
  # Nickel version requirement
  minimal_nickel_version = "1.9.0",
} | std.package.Manifest
