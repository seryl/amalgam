# Kubernetes API Package Module
# Auto-generated - do not edit manually

{
  api = import "./api/mod.ncl",
  "apimachinery.pkg" = import "./apimachinery.pkg/mod.ncl",

  # Version shortcuts for convenience
  v0 = import "./v0.ncl",
  v1 = import "./api/core/v1.ncl",
}