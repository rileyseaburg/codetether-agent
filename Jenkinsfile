def releaseRefName() {
    def jobBaseName = env.JOB_BASE_NAME?.trim()
    if (jobBaseName) {
        return jobBaseName
    }
    def jobLeaf = env.JOB_NAME?.tokenize('/')?.last()?.trim()
    if (jobLeaf) {
        return jobLeaf
    }
    def revisionAction = currentBuild?.rawBuild?.getAction(jenkins.scm.api.SCMRevisionAction)
    def headName = revisionAction?.revision?.head?.name?.trim()
    if (headName) {
        return headName
    }
    return (env.TAG_NAME ?: env.BRANCH_NAME ?: '').trim()
}

def isReleaseRefBuild() {
    return releaseRefName() ==~ /^v\d+\.\d+\.\d+(?:[-.][0-9A-Za-z.-]+)?$/
}

// Jenkins Job Configuration Requirements:
// - Multibranch Pipeline: Add "Discover tags" behavior in Branch Sources → Git → Behaviors
// - Or configure GitHub webhook to trigger builds on tag push events
// - The "Package & Release" and "Publish to crates.io" stages only run when building tags
// - Linux and Windows release binaries build on Depot remote builders via `depot build`
// - macOS builds run on the local Mac Mini via SSH (requires mac-mini-ssh Jenkins credential: usernamePassword type)
// - sshpass must be installed on the Jenkins agent for macOS builds

pipeline {
    agent any

    environment {
        CARGO_HOME            = "${env.HOME}/.cargo"
        RUSTUP_HOME           = "${env.HOME}/.rustup"
        PATH                  = "/usr/local/bin:${env.HOME}/.cargo/bin:${env.PATH}"
        REPO                  = 'rileyseaburg/codetether-agent'
        BINARY_NAME           = 'codetether'

        // macOS SSH options (Mac Mini uses password auth via sshpass)
        MAC_SSH_OPTS          = '-o StrictHostKeyChecking=no -o ConnectTimeout=30'
    }

    options {
        timestamps()
        timeout(time: 60, unit: 'MINUTES')
        disableConcurrentBuilds()
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
                // Clean stale .cargo from workspace (CARGO_HOME is now $HOME/.cargo)
                sh 'rm -rf "${WORKSPACE}/.cargo"'
            }
        }

        stage('Build') {
            parallel {
                stage('Linux x86_64') {
                    steps {
                        withCredentials([
                            string(credentialsId: 'depot-token', variable: 'DEPOT_TOKEN'),
                            string(credentialsId: 'depot-project-id', variable: 'DEPOT_PROJECT_ID')
                        ]) {
                            sh '''
                                export DEPOT_INSTALL_DIR="${WORKSPACE}/.depot/bin"
                                mkdir -p "${DEPOT_INSTALL_DIR}"
                                export PATH="${DEPOT_INSTALL_DIR}:${PATH}"
                                rm -rf dist/linux
                                mkdir -p dist/linux
                                curl -L https://depot.dev/install-cli.sh | DEPOT_INSTALL_DIR="${DEPOT_INSTALL_DIR}" sh
                                depot build \
                                  --project "${DEPOT_PROJECT_ID}" \
                                  --progress plain \
                                  --platform linux/amd64 \
                                  --file docker/release/linux.Dockerfile \
                                  --target artifact \
                                  --output "type=local,dest=dist/linux" \
                                  .
                                chmod 755 dist/linux/codetether
                            '''
                        }
                    }
                }
                stage('Windows x86_64') {
                    steps {
                        withCredentials([
                            string(credentialsId: 'depot-token', variable: 'DEPOT_TOKEN'),
                            string(credentialsId: 'depot-project-id', variable: 'DEPOT_PROJECT_ID')
                        ]) {
                            sh '''
                                export DEPOT_INSTALL_DIR="${WORKSPACE}/.depot/bin"
                                mkdir -p "${DEPOT_INSTALL_DIR}"
                                export PATH="${DEPOT_INSTALL_DIR}:${PATH}"
                                rm -rf dist/windows
                                mkdir -p dist/windows
                                curl -L https://depot.dev/install-cli.sh | DEPOT_INSTALL_DIR="${DEPOT_INSTALL_DIR}" sh
                                depot build \
                                  --project "${DEPOT_PROJECT_ID}" \
                                  --progress plain \
                                  --platform linux/amd64 \
                                  --file docker/release/windows.Dockerfile \
                                  --target artifact \
                                  --output "type=local,dest=dist/windows" \
                                  .
                            '''
                        }
                    }
                }
            }
        }

        stage('macOS (native)') {
            steps {
                script {
                    if (!isReleaseRefBuild()) {
                        echo "Skipping macOS build for non-release ref: ${releaseRefName()}"
                        return
                    }
                    env.RELEASE_REF = releaseRefName()
                    withCredentials([
                        usernamePassword(credentialsId: 'mac-mini-ssh',
                                         usernameVariable: 'MAC_USER',
                                         passwordVariable: 'MAC_PASS')
                    ]) {
                        sh '''#!/bin/bash
                        set -euo pipefail
                        export SSHPASS="${MAC_PASS}"
                        MAC_HOST_ADDR="${MAC_USER}@192.168.50.251"
                        echo "==> Building macOS binaries on Mac Mini (${MAC_HOST_ADDR})..."

                        # Copy source to Mac Mini
                        sshpass -e rsync -az --delete --exclude target --exclude .git \
                            -e "ssh ${MAC_SSH_OPTS}" \
                            ./ "${MAC_HOST_ADDR}":/tmp/codetether-build/

                        # Build both architectures (native arm64 + cross-compiled x86_64)
                        sshpass -e ssh ${MAC_SSH_OPTS} "${MAC_HOST_ADDR}" \
                            "bash -s" <<'REMOTE'
set -euo pipefail
cd /tmp/codetether-build
source "$HOME/.cargo/env" 2>/dev/null || true
rustup target add aarch64-apple-darwin x86_64-apple-darwin

echo "===> Building arm64 (native)..."
cargo build --release --target aarch64-apple-darwin

echo "===> Building x86_64 (cross)..."
cargo build --release --target x86_64-apple-darwin
REMOTE

                        # Fetch artifacts back
                        mkdir -p dist
                        sshpass -e scp ${MAC_SSH_OPTS} \
                            "${MAC_HOST_ADDR}":/tmp/codetether-build/target/aarch64-apple-darwin/release/${BINARY_NAME} \
                            dist/${BINARY_NAME}-${RELEASE_REF}-aarch64-apple-darwin
                        sshpass -e scp ${MAC_SSH_OPTS} \
                            "${MAC_HOST_ADDR}":/tmp/codetether-build/target/x86_64-apple-darwin/release/${BINARY_NAME} \
                            dist/${BINARY_NAME}-${RELEASE_REF}-x86_64-apple-darwin

                        chmod 755 dist/${BINARY_NAME}-*-apple-darwin
                        echo "==> macOS binaries fetched successfully"
                        '''
                    }
                }
            }
        }

        stage('Package & Release') {
            steps {
                script {
                    if (!isReleaseRefBuild()) {
                        echo "Skipping packaging for non-release ref: ${releaseRefName()}"
                        return
                    }
                    env.VERSION = releaseRefName()
                    env.RELEASE_REF = env.VERSION
                    sh """
                        mkdir -p dist

                        # Linux x86_64
                        LINUX_ARTIFACT="${BINARY_NAME}-${VERSION}-x86_64-unknown-linux-gnu"
                        cp dist/linux/${BINARY_NAME} "dist/\${LINUX_ARTIFACT}"
                        cd dist && tar czf "\${LINUX_ARTIFACT}.tar.gz" "\${LINUX_ARTIFACT}" && cd ..

                        # Windows x86_64
                        WIN_ARTIFACT="${BINARY_NAME}-${VERSION}-x86_64-pc-windows-gnu.exe"
                        cp dist/windows/${BINARY_NAME}.exe "dist/\${WIN_ARTIFACT}"
                        cd dist && tar czf "\${WIN_ARTIFACT%.exe}.tar.gz" "\${WIN_ARTIFACT}" && cd ..

                        # macOS arm64
                        MAC_ARM64="${BINARY_NAME}-${VERSION}-aarch64-apple-darwin"
                        if [ -f "dist/${BINARY_NAME}-\${VERSION}-aarch64-apple-darwin" ]; then
                            cd dist && tar czf "\${MAC_ARM64}.tar.gz" "${BINARY_NAME}-\${VERSION}-aarch64-apple-darwin" && cd ..
                        fi

                        # macOS x86_64
                        MAC_X86="${BINARY_NAME}-${VERSION}-x86_64-apple-darwin"
                        if [ -f "dist/${BINARY_NAME}-\${VERSION}-x86_64-apple-darwin" ]; then
                            cd dist && tar czf "\${MAC_X86}.tar.gz" "${BINARY_NAME}-\${VERSION}-x86_64-apple-darwin" && cd ..
                        fi

                        # Checksums for all artifacts
                        cd dist && sha256sum *.tar.gz *.exe > SHA256SUMS-${VERSION}.txt && cd ..
                    """
                    archiveArtifacts artifacts: 'dist/*.tar.gz, dist/*.exe, dist/SHA256SUMS-*.txt', fingerprint: true

                    withCredentials([string(credentialsId: 'github-token', variable: 'GH_TOKEN')]) {
                        sh """
                            # Extract release notes from annotated tag (generated by release.sh via codetether)
                            TAG_BODY=\$(git tag -l --format='%(contents:body)' ${env.RELEASE_REF} 2>/dev/null || echo '')
                            if [ -n "\$TAG_BODY" ]; then
                                echo "\$TAG_BODY" > /tmp/release-notes.md
                                NOTES_FLAG="--notes-file /tmp/release-notes.md"
                            else
                                NOTES_FLAG="--generate-notes"
                            fi

                            gh release create ${env.RELEASE_REF} \
                                dist/*.tar.gz \
                                dist/*.exe \
                                dist/SHA256SUMS-${VERSION}.txt \
                                --repo ${REPO} \
                                --title "${env.RELEASE_REF} - CodeTether Agent" \
                                \$NOTES_FLAG \
                                --latest \
                                --verify-tag || \
                            gh release upload ${env.RELEASE_REF} \
                                dist/*.tar.gz \
                                dist/*.exe \
                                dist/SHA256SUMS-${VERSION}.txt \
                                --repo ${REPO} --clobber
                        """
                    }
                }
            }
        }

        stage('Publish to crates.io') {
            steps {
                script {
                    if (!isReleaseRefBuild()) {
                        echo "Skipping crates publish for non-release ref: ${releaseRefName()}"
                        return
                    }
                    env.VERSION = releaseRefName()
                    withCredentials([
                        string(credentialsId: 'cargo-registry-token', variable: 'CARGO_REGISTRY_TOKEN')
                    ]) {
                        sh '''
                            echo "Publishing ${VERSION} to crates.io ..."
                            cargo publish --allow-dirty
                        '''
                    }
                }
            }
        }
    }

    post {
        success {
            echo "Build successful: ${env.TAG_NAME ?: env.BRANCH_NAME}"
        }
        failure {
            echo "Build failed: ${env.TAG_NAME ?: env.BRANCH_NAME}"
        }
    }
}
