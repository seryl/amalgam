{
  name = "test-app",
  version = "0.1.0",
  description = "Test application using k8s and crossplane types",
  minimal_nickel_version = "1.9.0",
  dependencies = {
    k8s = 'Path "../k8s_io",
    crossplane = 'Path "../crossplane",
  },
} | std.package.Manifest
