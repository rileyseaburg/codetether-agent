// Jenkins Job Configuration Requirements:
// - Multibranch Pipeline: Add "Discover tags" behavior in Branch Sources → Git → Behaviors
// - Or configure GitHub webhook to trigger builds on tag push events
// - The "Package & Release" and "Publish to crates.io" stages only run when building tags
// - Windows cross-compilation requires: mingw-w64 (gcc + g++), rustup target x86_64-pc-windows-gnu
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

        // sccache backed by MinIO S3
        RUSTC_WRAPPER         = 'sccache'
        SCCACHE_BUCKET        = 'sccache'
        SCCACHE_ENDPOINT      = 'http://192.168.50.223:9000'
        SCCACHE_REGION        = 'us-east-1'
        SCCACHE_S3_KEY_PREFIX = 'codetether'
        SCCACHE_S3_USE_SSL    = 'false'

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
                            string(credentialsId: 'minio-access-key', variable: 'AWS_ACCESS_KEY_ID'),
                            string(credentialsId: 'minio-secret-key', variable: 'AWS_SECRET_ACCESS_KEY')
                        ]) {
                            sh '''
                                rustc --version
                                sccache --start-server 2>/dev/null || true
                                sccache --show-stats 2>&1 | grep "Cache location" || true
                                cargo build --release --features functiongemma
                                echo "=== sccache stats ==="
                                sccache --show-stats || true
                            '''
                        }
                    }
                }
                stage('Windows x86_64') {
                    steps {
                        withCredentials([
                            string(credentialsId: 'minio-access-key', variable: 'AWS_ACCESS_KEY_ID'),
                            string(credentialsId: 'minio-secret-key', variable: 'AWS_SECRET_ACCESS_KEY')
                        ]) {
                            sh '''
                                echo "Cross-compiling for Windows..."
                                cargo build --target x86_64-pc-windows-gnu --release --features functiongemma
                            '''
                        }
                    }
                }
            }
        }

        stage('macOS (native)') {
            when {
                buildingTag()
            }
            steps {
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
                    sshpass -e ssh ${MAC_SSH_OPTS} "${MAC_HOST_ADDR}" bash -s <<'REMOTE'
                        set -euo pipefail
                        cd /tmp/codetether-build
                        source $HOME/.cargo/env 2>/dev/null || true

                        echo "===> Building arm64 (native)..."
                        cargo build --release --target aarch64-apple-darwin --features functiongemma

                        echo "===> Building x86_64 (cross)..."
                        cargo build --release --target x86_64-apple-darwin --features functiongemma
                    REMOTE

                    # Fetch artifacts back
                    mkdir -p dist
                    sshpass -e scp ${MAC_SSH_OPTS} \
                        "${MAC_HOST_ADDR}":/tmp/codetether-build/target/aarch64-apple-darwin/release/${BINARY_NAME} \
                        dist/${BINARY_NAME}-${TAG_NAME}-aarch64-apple-darwin
                    sshpass -e scp ${MAC_SSH_OPTS} \
                        "${MAC_HOST_ADDR}":/tmp/codetether-build/target/x86_64-apple-darwin/release/${BINARY_NAME} \
                        dist/${BINARY_NAME}-${TAG_NAME}-x86_64-apple-darwin

                    chmod 755 dist/${BINARY_NAME}-*-apple-darwin
                    echo "==> macOS binaries fetched successfully"
                    '''
                }
            }
        }

        stage('Package & Release') {
            when {
                buildingTag()
            }
            steps {
                script {
                    env.VERSION = env.TAG_NAME
                }
                sh """
                    mkdir -p dist

                    # Linux x86_64
                    LINUX_ARTIFACT="${BINARY_NAME}-${VERSION}-x86_64-unknown-linux-gnu"
                    cp target/release/${BINARY_NAME} "dist/\${LINUX_ARTIFACT}"
                    cd dist && tar czf "\${LINUX_ARTIFACT}.tar.gz" "\${LINUX_ARTIFACT}" && cd ..

                    # Windows x86_64
                    WIN_ARTIFACT="${BINARY_NAME}-${VERSION}-x86_64-pc-windows-gnu.exe"
                    cp target/x86_64-pc-windows-gnu/release/${BINARY_NAME}.exe "dist/\${WIN_ARTIFACT}"
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
                        TAG_BODY=\$(git tag -l --format='%(contents:body)' ${env.TAG_NAME} 2>/dev/null || echo '')
                        if [ -n "\$TAG_BODY" ]; then
                            echo "\$TAG_BODY" > /tmp/release-notes.md
                            NOTES_FLAG="--notes-file /tmp/release-notes.md"
                        else
                            NOTES_FLAG="--generate-notes"
                        fi

                        gh release create ${env.TAG_NAME} \
                            dist/*.tar.gz \
                            dist/*.exe \
                            dist/SHA256SUMS-${VERSION}.txt \
                            --repo ${REPO} \
                            --title "${env.TAG_NAME} - CodeTether Agent" \
                            \$NOTES_FLAG \
                            --latest \
                            --verify-tag || \
                        gh release upload ${env.TAG_NAME} \
                            dist/*.tar.gz \
                            dist/*.exe \
                            dist/SHA256SUMS-${VERSION}.txt \
                            --repo ${REPO} --clobber
                    """
                }
            }
        }

        stage('Publish to crates.io') {
            when {
                buildingTag()
            }
            steps {
                withCredentials([
                    string(credentialsId: 'cargo-registry-token', variable: 'CARGO_REGISTRY_TOKEN'),
                    string(credentialsId: 'minio-access-key', variable: 'AWS_ACCESS_KEY_ID'),
                    string(credentialsId: 'minio-secret-key', variable: 'AWS_SECRET_ACCESS_KEY')
                ]) {
                    sh '''
                        echo "Publishing ${TAG_NAME} to crates.io ..."
                        cargo publish --allow-dirty --features functiongemma
                    '''
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
