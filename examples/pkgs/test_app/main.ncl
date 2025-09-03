# Test application using amalgam-generated packages
let k8s = import k8s in
let crossplane = import crossplane in

{
  # Create a Deployment using k8s types
  deployment = {
    apiVersion = "apps/v1",
    kind = "Deployment",
    metadata = {
      name = "test-app",
      namespace = "default",
    },
    spec = {
      replicas = 3,
      selector = {
        matchLabels = {
          app = "test",
        },
      },
      template = {
        metadata = {
          labels = {
            app = "test",
          },
        },
        spec = {
          containers = [
            {
              name = "app",
              image = "nginx:latest",
              ports = [
                {
                  containerPort = 80,
                },
              ],
            },
          ],
        },
      },
    },
  },
  
  # Create a Composition using crossplane types
  composition = {
    apiVersion = "apiextensions.crossplane.io/v1",
    kind = "Composition",
    metadata = {
      name = "test-composition",
    },
    spec = {
      compositeTypeRef = {
        apiVersion = "example.org/v1",
        kind = "XDatabase",
      },
      mode = "Pipeline",
      pipeline = [
        {
          step = "create-db",
          functionRef = {
            name = "function-create-db",
          },
        },
      ],
    },
  },
}
