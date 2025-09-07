# Test that k8s_io generated packages evaluate correctly
# Tests PascalCase file imports and camelCase variable naming

let pod = import "../../pkgs/k8s_io/v1/Pod.ncl" in
let objectMeta = import "../../pkgs/k8s_io/v1/ObjectMeta.ncl" in
let podSpec = import "../../pkgs/k8s_io/v1/PodSpec.ncl" in
let container = import "../../pkgs/k8s_io/v1/Container.ncl" in

# Create a test pod using the imported types
pod & {
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
}