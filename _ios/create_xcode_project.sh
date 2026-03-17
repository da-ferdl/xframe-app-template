#!/bin/sh

#### Uses XcodeGen to create a Xcode project from the 'PROJECT_YAML' definition ###
# -> run from within the directory where this file resides:
# $ ./create_xcode_project.sh

CRATE_NAME="xframe_app"
PROD_NAME=${CRATE_NAME//_/-}

#str="${PROD_NAME/_ /.}"

#PROD_BUNDLE_ID="one.lopes.${PROD_NAME}"
PROD_BUNDLE_ID="one.lopes.${PROD_NAME}"
DEV_TEAM_ID="638T4K7GQ3"

#echo "$CRATE_NAME"
#echo "$PROD_NAME"

#exit 0

PROJECT_YAML="
name: ${PROD_NAME}
options:
  bundleIdPrefix: ${PROD_BUNDLE_ID}
  deploymentTarget:
    iOS: 13.1
#fileGroups: [../src]
configs:
  debug: debug
  release: release
settingGroups:
  app:
    base:
      PRODUCT_NAME: ${PROD_NAME}
      PRODUCT_BUNDLE_IDENTIFIER: ${PROD_BUNDLE_ID}
      DEVELOPMENT_TEAM: ${DEV_TEAM_ID}
targetTemplates:
  app:
    type: application
    sources:
      - path: Sources
    scheme:
      environmentVariables:
        RUST_BACKTRACE: full
        RUST_LOG: info
    settings:
      groups: [app]
targets:
  ${PROD_NAME}:
    type: application
    platform: iOS
    sources:
      - path: Sources
      - path: Assets.xcassets
      - path: LaunchScreen.storyboard
      - path: assets
        buildPhase: resources
        type: folder
    info:
      path: Info.plist
      properties:
        LSRequiresIPhoneOS: true
        UIRequiredDeviceCapabilities: [arm64, metal]
        UISupportedInterfaceOrientations:
          - UIInterfaceOrientationPortrait
          - UIInterfaceOrientationLandscapeLeft
          - UIInterfaceOrientationLandscapeRight
        UISupportedInterfaceOrientations~ipad:
          - UIInterfaceOrientationPortrait
          - UIInterfaceOrientationPortraitUpsideDown
          - UIInterfaceOrientationLandscapeLeft
          - UIInterfaceOrientationLandscapeRight
        CFBundleShortVersionString: 0.1.0
        CFBundleVersion: "0.1.0"
        UILaunchStoryboardName: LaunchScreen.storyboard
    entitlements:
      path: ${PROD_NAME}.entitlements
    scheme:
      environmentVariables:
        RUST_BACKTRACE: full
        RUST_LOG: info
    settings:
      base:
        ENABLE_BITCODE: false
        HEADER_SEARCH_PATHS: \$(inherited)
        ARCHS: [arm64, x86_64]
        VALID_ARCHS: arm64  x86_64
        ALWAYS_EMBED_SWIFT_STANDARD_LIBRARIES: true
        EXCLUDED_ARCHS[sdk=iphoneos*]: x86_64
      groups: [app]
    dependencies:
      - framework: ${CRATE_NAME}.xcframework
        embed: false
      - sdk: CoreGraphics.framework
      - sdk: Metal.framework
      - sdk: MetalKit.framework
      - sdk: QuartzCore.framework
      - sdk: Security.framework
      - sdk: UIKit.framework
    preBuildScripts:
      - script: LIB_NAME=${CRATE_NAME} ./build_ios_rust.sh
        name: Build Rust Code
        basedOnDependencyAnalysis: false
"

[[ -f tmp_project.yaml ]] && rm tmp_project.yaml
echo "$PROJECT_YAML" > tmp_project.yaml

xcodegen generate --spec tmp_project.yaml

[[ -f tmp_project.yaml ]] && rm tmp_project.yaml

LIB_NAME=$CRATE_NAME ./build_ios_rust.sh
