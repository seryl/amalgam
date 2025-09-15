# Test file to validate generated packages evaluate correctly in Nickel
# This tests the new naming conventions: PascalCase files, camelCase variables

# Import k8s_io types
let pod = import "./pkgs/k8s_io/v1/Pod.ncl" in
let objectMeta = import "./pkgs/k8s_io/v1/ObjectMeta.ncl" in
let podSpec = import "./pkgs/k8s_io/v1/PodSpec.ncl" in
let container = import "./pkgs/k8s_io/v1/Container.ncl" in

# Test basic Pod construction
let test_pod = pod & {
  apiVersion = "v1",
  kind = "Pod",
  metadata = objectMeta & {
    name = "test-pod",
    namespace = "default",
    labels = {
      app = "test",
      environment = "dev"
    }
  },
  spec = podSpec & {
    containers = [
      container & {
        name = "main",
        image = "nginx:latest",
        ports = [{
          containerPort = 80,
          protocol = "TCP"
        }]
      }
    ]
  }
} in

# Export the test pod for evaluation
test_pod