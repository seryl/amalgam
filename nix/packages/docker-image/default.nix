# NixOS Docker/OCI image builder for Amalgam
{ pkgs
, lib
, amalgam
, nickel
, generated-packages ? null
}:

let
  # Create a minimal runtime environment
  runtimeEnv = pkgs.buildEnv {
    name = "amalgam-runtime-env";
    paths = with pkgs; [
      bashInteractive
      coreutils
      cacert
      git
      curl
    ];
  };

  # Amalgam compiler image
  amalgamImage = pkgs.dockerTools.buildImage {
    name = "amalgam-compiler";
    tag = "latest";
    
    contents = [
      runtimeEnv
      amalgam
    ];
    
    config = {
      Cmd = [ "${amalgam}/bin/amalgam" ];
      WorkingDir = "/workspace";
      Env = [
        "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
        "PATH=/bin:${amalgam}/bin"
      ];
      Labels = {
        "org.opencontainers.image.source" = "https://github.com/seryl/amalgam";
        "org.opencontainers.image.description" = "Amalgam compiler for generating Nickel types";
        "org.opencontainers.image.licenses" = "Apache-2.0";
      };
    };
  };

  # Nickel packages distribution image
  packagesImage = pkgs.dockerTools.buildImage {
    name = "nickel-packages";
    tag = "latest";
    
    copyToRoot = pkgs.buildEnv {
      name = "packages-root";
      paths = [
        runtimeEnv
        nickel
        (pkgs.runCommand "packages-dir" {} ''
          mkdir -p $out/packages
          ${lib.optionalString (generated-packages != null) ''
            cp -r ${generated-packages}/* $out/packages/
          ''}
        '')
      ] ++ lib.optionals (generated-packages != null) [ generated-packages ];
    };
    
    config = {
      Cmd = [ "${pkgs.bashInteractive}/bin/bash" ];
      WorkingDir = "/packages";
      Env = [
        "PATH=/bin:${nickel}/bin"
      ];
      Labels = {
        "org.opencontainers.image.source" = "https://github.com/seryl/amalgam";
        "org.opencontainers.image.description" = "Nickel type packages for Kubernetes and cloud resources";
        "org.opencontainers.image.licenses" = "Apache-2.0";
      };
    };
  };

  # Layered image with better caching
  amalgamLayeredImage = pkgs.dockerTools.buildLayeredImage {
    name = "amalgam-compiler";
    tag = "latest";
    
    contents = [
      runtimeEnv
      amalgam
    ];
    
    config = {
      Cmd = [ "${amalgam}/bin/amalgam" ];
      WorkingDir = "/workspace";
      Env = [
        "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
        "PATH=/bin:${amalgam}/bin"
      ];
      Labels = {
        "org.opencontainers.image.source" = "https://github.com/seryl/amalgam";
        "org.opencontainers.image.description" = "Amalgam compiler for generating Nickel types";
        "org.opencontainers.image.licenses" = "Apache-2.0";
      };
    };
    
    # Maximum number of layers for better caching
    maxLayers = 100;
  };

  # Stream image (most efficient for CI)
  amalgamStreamImage = pkgs.dockerTools.streamLayeredImage {
    name = "amalgam-compiler";
    tag = "latest";
    
    contents = [
      runtimeEnv
      amalgam
    ];
    
    config = {
      Cmd = [ "${amalgam}/bin/amalgam" ];
      WorkingDir = "/workspace";
      Env = [
        "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
        "PATH=/bin:${amalgam}/bin"
      ];
      Labels = {
        "org.opencontainers.image.source" = "https://github.com/seryl/amalgam";
        "org.opencontainers.image.description" = "Amalgam compiler for generating Nickel types";
        "org.opencontainers.image.licenses" = "Apache-2.0";
      };
    };
  };

  # Multi-platform image builder
  multiPlatformImage = { name, contents, config }:
    let
      # Build for multiple architectures
      platforms = {
        "linux/amd64" = pkgs.pkgsCross.musl64;
        "linux/arm64" = pkgs.pkgsCross.aarch64-multiplatform;
      };
      
      images = lib.mapAttrs (platform: crossPkgs:
        crossPkgs.dockerTools.buildImage {
          inherit name config;
          tag = platform;
          contents = contents crossPkgs;
        }
      ) platforms;
    in
    images;

in
{
  # Single images
  inherit amalgamImage packagesImage amalgamLayeredImage;
  
  # Streaming image (for piping directly to docker load)
  amalgamStream = amalgamStreamImage;
  
  # Build script for CI
  pushToRegistry = pkgs.writeShellScriptBin "push-to-registry" ''
    #!${pkgs.stdenv.shell}
    set -euo pipefail
    
    REGISTRY=''${REGISTRY:-ghcr.io}
    REPO=''${REPO:-seryl/amalgam}
    TAG=''${TAG:-latest}
    
    echo "Loading amalgam image..."
    docker load < ${amalgamImage}
    
    echo "Loading packages image..."
    docker load < ${packagesImage}
    
    echo "Tagging images..."
    docker tag amalgam-compiler:latest $REGISTRY/$REPO/amalgam:$TAG
    docker tag nickel-packages:latest $REGISTRY/$REPO/packages:$TAG
    
    echo "Pushing to $REGISTRY..."
    docker push $REGISTRY/$REPO/amalgam:$TAG
    docker push $REGISTRY/$REPO/packages:$TAG
    
    echo "✅ Images pushed successfully!"
  '';
  
  # Skopeo-based push (no Docker daemon needed)
  pushWithSkopeo = pkgs.writeShellScriptBin "push-with-skopeo" ''
    #!${pkgs.stdenv.shell}
    set -euo pipefail
    
    REGISTRY=''${REGISTRY:-ghcr.io}
    REPO=''${REPO:-seryl/amalgam}
    TAG=''${TAG:-latest}
    
    echo "Pushing amalgam image with skopeo..."
    ${pkgs.skopeo}/bin/skopeo copy \
      docker-archive:${amalgamImage} \
      docker://$REGISTRY/$REPO/amalgam:$TAG \
      --dest-creds="$REGISTRY_USER:$REGISTRY_PASSWORD"
    
    echo "Pushing packages image with skopeo..."  
    ${pkgs.skopeo}/bin/skopeo copy \
      docker-archive:${packagesImage} \
      docker://$REGISTRY/$REPO/packages:$TAG \
      --dest-creds="$REGISTRY_USER:$REGISTRY_PASSWORD"
      
    echo "✅ Images pushed successfully!"
  '';
}